use crate::{aggregate::Aggregator, filter};
use crossbeam_channel::bounded;
use rayon::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread;

pub trait LogParser: Send + Sync {
    fn parse_line(&self, line: &str) -> Option<crate::types::LogEntry>;
}

impl<T: LogParser + ?Sized> LogParser for Box<T> {
    fn parse_line(&self, line: &str) -> Option<crate::types::LogEntry> {
        (**self).parse_line(line)
    }
}

pub fn run_pipeline<P: LogParser + 'static>(
    file_path: &str,
    parser: P,
    query: Option<String>,
) -> Result<Aggregator, std::io::Error> {
    let path = std::fs::canonicalize(file_path)?;
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let parallelism = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);
    let (tx, rx) = bounded::<Vec<String>>(parallelism * 4);

    let producer = thread::spawn(move || {
        let mut buffer = Vec::with_capacity(10_000);

        for line in reader.lines() {
            match line {
                Ok(l) => buffer.push(l),
                Err(_) => continue,
            }

            if buffer.len() >= 10_000 {
                if tx.send(buffer).is_err() {
                    return;
                }
                buffer = Vec::with_capacity(10_000);
            }
        }

        if !buffer.is_empty() {
            tx.send(buffer).ok();
        }
    });

    let agg = rx
        .into_iter()
        .par_bridge()
        .map(|chunk| {
            let mut local = Aggregator::default();

            for line in chunk {
                if let Some(entry) = parser.parse_line(&line) {
                    if filter::matches(&entry, &query) {
                        local.process(&entry);
                    }
                }
            }

            local
        })
        .reduce(Aggregator::default, |mut a, b| {
            a.merge(b);
            a
        });

    producer.join().ok();

    Ok(agg)
}
