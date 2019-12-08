#![feature(is_sorted)]
use criterion::{black_box, criterion_group, criterion_main, Criterion};
#[path = "../src/day4.rs"]
mod day4;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day4_star2", |b| b.iter(|| day4::star2()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
