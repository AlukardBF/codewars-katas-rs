use codewars::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::time::Duration;
use codewars::integers_recreating_one::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("integers_recreating_one");
    group.sample_size(10);
    group.measurement_time(Duration::new(60, 0));

    let mut divisors = Divisors::with_capacity(200);
    let mut primes = Divisors::with_capacity(100);
    // let mut primes = Primes::with_capacity(100);
    // group.bench_function("list_squares", |b| {
    //     b.iter(|| {
    //         list_squared(black_box(1), black_box(200000));
    //     })
    // });
    group.bench_function("get_divisors", |b| {
        b.iter(|| {
            let range = black_box(1..200000);
            range.for_each(|num| {
                // trial_prime(num, &mut primes);
                // get_divisors(0, 1, &mut primes);
                get_divisors_old(num, &mut primes, &mut divisors);
            })
        })
    });
    group.bench_function("trial_prime", |b| {
        b.iter(|| {
            let range = black_box(1..200000);
            range.for_each(|num| {
                // trial_prime(num, &mut primes);
                trial_prime_old(num, &mut primes);
            })
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
