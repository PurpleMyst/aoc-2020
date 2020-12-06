use std::time::Duration;

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

pub fn day4_benchmark(c: &mut Criterion) {
    c.bench_function("day4", |b| b.iter(|| day4::solve()));
}

pub fn day5_benchmark(c: &mut Criterion) {
    c.bench_function("day5", |b| b.iter(|| day5::solve()));
}

pub fn day6_benchmark(c: &mut Criterion) {
    c.bench_function("day6", |b| b.iter(|| day6::solve()));
}

pub fn alldays_benchmark(c: &mut Criterion) {
    c.bench_function("all", |b| {
        b.iter(|| {
            (
                day1::solve(),
                day2::solve(),
                day3::solve(),
                day4::solve(),
                day5::solve(),
                day6::solve(),
            )
        })
    });
}

criterion_group! {
    name = benches;

    config = Criterion::default()
        .significance_level(0.1)
        .sample_size(500)
        .measurement_time(Duration::from_secs(30))
        .warm_up_time(Duration::from_secs(15))
        .noise_threshold(0.05);

    targets =
        day1_benchmark,
        day2_benchmark,
        day3_benchmark,
        day4_benchmark,
        day5_benchmark,
        day6_benchmark,
        alldays_benchmark
}

criterion_main!(benches);
