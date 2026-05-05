use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn stream_lines(
    path: &str,
    chunk_size: usize,
) -> Result<impl Iterator<Item = Vec<String>>, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines();

    Ok(std::iter::from_fn(move || {
        let mut chunk = Vec::with_capacity(chunk_size);

        for _ in 0..chunk_size {
            match lines.next() {
                Some(Ok(line)) => chunk.push(line),
                _ => break,
            }
        }

        if chunk.is_empty() {
            None
        } else {
            Some(chunk)
        }
    }))
}