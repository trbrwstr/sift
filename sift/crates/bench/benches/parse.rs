use criterion::{criterion_group, criterion_main, Criterion};
use sift_formats::plain::PlainParser;
use sift_core::engine::LogParser;

fn bench_parse(c: &mut Criterion) {
    let parser = PlainParser;

    let lines: Vec<String> = (0..100_000)
        .map(|i| format!("log line number {}", i))
        .collect();

    c.bench_function("parse_100k_lines", |b| {
        b.iter(|| {
            for line in &lines {
                let _ = parser.parse_line(line);
            }
        })
    });
}

criterion_group!(benches, bench_parse);
criterion_main!(benches);