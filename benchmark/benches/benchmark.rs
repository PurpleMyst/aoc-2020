use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};

pub fn day01_benchmark(c: &mut Criterion) {
    c.bench_function("day01", |b| b.iter(|| day01::solve()));
}

pub fn day02_benchmark(c: &mut Criterion) {
    c.bench_function("day02", |b| b.iter(|| day02::solve()));
}

pub fn day03_benchmark(c: &mut Criterion) {
    c.bench_function("day03", |b| b.iter(|| day03::solve()));
}

pub fn day04_benchmark(c: &mut Criterion) {
    c.bench_function("day04", |b| b.iter(|| day04::solve()));
}

pub fn day05_benchmark(c: &mut Criterion) {
    c.bench_function("day05", |b| b.iter(|| day05::solve()));
}

pub fn day06_benchmark(c: &mut Criterion) {
    c.bench_function("day06", |b| b.iter(|| day06::solve()));
}

pub fn day07_benchmark(c: &mut Criterion) {
    c.bench_function("day07", |b| b.iter(|| day07::solve()));
}

pub fn day08_benchmark(c: &mut Criterion) {
    c.bench_function("day08", |b| b.iter(|| day08::solve()));
}

pub fn day09_benchmark(c: &mut Criterion) {
    c.bench_function("day09", |b| b.iter(|| day09::solve()));
}

pub fn day10_benchmark(c: &mut Criterion) {
    c.bench_function("day10", |b| b.iter(|| day10::solve()));
}

pub fn day11_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("day11");

    group.bench_function("parsing", |b| b.iter(|| day11::parse_input()));

    let original_cells = day11::parse_input();

    let mut cells = original_cells.clone();

    // We're including the copy in the benchmarks here, but I can't imagine that a simple memcpy() matters much.
    group.bench_function("part1", |b| {
        b.iter(|| {
            cells.copy_from_slice(&original_cells[..]);
            day11::solve_part1(&mut cells[..])
        })
    });

    group.bench_function("part2", |b| {
        b.iter(|| {
            cells.copy_from_slice(&original_cells[..]);
            day11::solve_part2(&mut cells[..])
        })
    });

    group.finish();
}

pub fn day12_benchmark(c: &mut Criterion) {
    c.bench_function("day12", |b| b.iter(|| day12::solve()));
}

pub fn day13_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("day13");

    group.bench_function("parsing", |b| b.iter(|| day13::parse_input()));

    let (earliest, buses) = day13::parse_input();

    group.bench_function("part1", |b| {
        b.iter(|| day13::solve_part1(earliest, buses.iter().map(|&(_, bus_id)| bus_id)));
    });

    group.bench_function("part2", |b| b.iter(|| day13::solve_part2(&buses)));

    group.finish();
}

pub fn day14_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("day14");

    group.bench_function("parsing", |b| b.iter(|| day14::parse_input()));

    let instructions = day14::parse_input();

    group.bench_function("part1", |b| {
        b.iter(|| day14::solve_part1(&instructions));
    });

    group.bench_function("part2", |b| b.iter(|| day14::solve_part2(&instructions)));

    group.finish();
}

pub fn day15_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("day15");
    group.sample_size(10);
    group.measurement_time(Duration::from_secs(60));
    group.bench_function("solve", |b| b.iter(|| day15::solve()));
    group.finish();
}

pub fn day16_benchmark(c: &mut Criterion) {
    c.bench_function("day16", |b| b.iter(|| day16::solve()));
}

pub fn day17_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("day17");

    group.bench_function("parsing", |b| b.iter(|| day17::parse_input()));

    let initial_state = day17::parse_input();

    group.bench_function("part1", |b| {
        b.iter(|| day17::solve_part1(&initial_state));
    });

    group.bench_function("part2", |b| b.iter(|| day17::solve_part2(&initial_state)));

    group.finish();
}

pub fn day18_benchmark(c: &mut Criterion) {
    c.bench_function("day18", |b| b.iter(|| day18::solve()));
}

pub fn day19_benchmark(c: &mut Criterion) {
    c.bench_function("day19", |b| b.iter(|| day19::solve()));
}

pub fn day20_benchmark(c: &mut Criterion) {
    c.bench_function("day20", |b| b.iter(|| day20::solve()));
}

pub fn alldays_benchmark(c: &mut Criterion) {
    c.bench_function("all", |b| {
        b.iter(|| {
            (
                day01::solve(),
                day02::solve(),
                day03::solve(),
                day04::solve(),
                day05::solve(),
                day06::solve(),
                day07::solve(),
                day08::solve(),
                day09::solve(),
                day10::solve(),
                day11::solve(),
                day12::solve(),
                day13::solve(),
                day14::solve(),
                day15::solve(),
                day16::solve(),
                day17::solve(),
                day18::solve(),
                day19::solve(),
                day20::solve(),
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
        day01_benchmark,
        day02_benchmark,
        day03_benchmark,
        day04_benchmark,
        day05_benchmark,
        day06_benchmark,
        day07_benchmark,
        day08_benchmark,
        day09_benchmark,
        day10_benchmark,
        day11_benchmark,
        day12_benchmark,
        day13_benchmark,
        day14_benchmark,
        day15_benchmark,
        day16_benchmark,
        day17_benchmark,
        day18_benchmark,
        day19_benchmark,
        day20_benchmark,
        alldays_benchmark
}

criterion_main!(benches);
