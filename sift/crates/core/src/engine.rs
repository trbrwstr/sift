use crate::{aggregate::Aggregator, filter};
use crossbeam_channel::unbounded;
use rayon::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread;

pub trait LogParser: Send + Sync {
    fn parse_line(&self, line: &str) -> Option<crate::types::LogEntry>;
}

pub fn run_pipeline<P: LogParser + 'static>(
    file_path: &str,
    parser: P,
    query: Option<String>,
) -> Result<Aggregator, std::io::Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let (tx, rx) = unbounded::<Vec<String>>();

    // Producer thread (streaming read)
    thread::spawn(move || {
        let mut buffer = Vec::with_capacity(10_000);

        for line in reader.lines().flatten() {
            buffer.push(line);

            if buffer.len() >= 10_000 {
                tx.send(buffer).ok();
                buffer = Vec::with_capacity(10_000);
            }
        }

        if !buffer.is_empty() {
            tx.send(buffer).ok();
        }
    });

    // Consumers (parallel)
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

    Ok(agg)
}