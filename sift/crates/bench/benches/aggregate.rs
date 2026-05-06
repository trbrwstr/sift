use criterion::{criterion_group, criterion_main, Criterion};
use sift_core::aggregate::Aggregator;
use sift_core::types::LogEntry;

fn bench_aggregate(c: &mut Criterion) {
    let entries: Vec<LogEntry> = (0..100_000)
        .map(|i| LogEntry {
            timestamp: None,
            level: Some(if i % 2 == 0 { "INFO".into() } else { "ERROR".into() }),
            message: format!("msg {}", i % 10),
            fields: vec![],
        })
        .collect();

    c.bench_function("aggregate_100k", |b| {
        b.iter(|| {
            let mut agg = Aggregator::default();
            for e in &entries {
                agg.process(e);
            }
        })
    });
}

criterion_group!(benches, bench_aggregate);
criterion_main!(benches);