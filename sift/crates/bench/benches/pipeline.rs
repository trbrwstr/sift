use criterion::{criterion_group, criterion_main, Criterion};
use logforge_core::engine::run_pipeline;
use logforge_formats::plain::PlainParser;

fn bench_pipeline(c: &mut Criterion) {
    c.bench_function("process_log_file", |b| {
        b.iter(|| {
            let _ = run_pipeline("examples/sample.log", PlainParser, None);
        })
    });
}

criterion_group!(benches, bench_pipeline);
criterion_main!(benches);