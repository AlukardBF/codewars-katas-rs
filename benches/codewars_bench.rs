use codewars::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("sum_by_factors", |b| {
        b.iter(|| sum_by_factors::sum_of_divided(black_box(vec![15, 21, 24, 30, -45])))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
