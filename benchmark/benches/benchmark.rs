use criterion::{criterion_group, criterion_main, Criterion};

pub fn day1_benchmark(c: &mut Criterion) {
    c.bench_function("day1", |b| b.iter(|| day1::solve()));
}

pub fn day2_benchmark(c: &mut Criterion) {
    c.bench_function("day2", |b| b.iter(|| day2::solve()));
}

pub fn day3_benchmark(c: &mut Criterion) {
    c.bench_function("day3", |b| b.iter(|| day3::solve()));
}

pub fn alldays_benchmark(c: &mut Criterion) {
    c.bench_function("all", |b| {
        b.iter(|| (day1::solve(), day2::solve(), day3::solve()))
    });
}

criterion_group!(
    benches,
    day1_benchmark,
    day2_benchmark,
    day3_benchmark,
    alldays_benchmark
);
criterion_main!(benches);
