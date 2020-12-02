use criterion::{criterion_group, criterion_main, Criterion};

pub fn day1_benchmark(c: &mut Criterion) {
    c.bench_function("day1", |b| b.iter(|| day1::solve()));
}

pub fn day2_benchmark(c: &mut Criterion) {
    c.bench_function("day2", |b| b.iter(|| day2::solve()));
}

criterion_group!(benches, day1_benchmark, day2_benchmark);
criterion_main!(benches);
