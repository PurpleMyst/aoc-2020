use criterion::{criterion_group, criterion_main, Criterion};

pub fn day1_benchmark(c: &mut Criterion) {
    c.bench_function("day1 solve", |b| b.iter(|| day1::solve()));
}

criterion_group!(benches, day1_benchmark);
criterion_main!(benches);
