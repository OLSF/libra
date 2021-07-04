//! Benchmarks

use criterion::{criterion_group, criterion_main, Criterion};
use miner::delay;

// exammple code
// fn fibonacci(n: u64) -> u64 {
//     match n {
//         0 => 1,
//         1 => 1,
//         n => fibonacci(n-1) + fibonacci(n-2),
//     }
// }

fn bench_delay(c: &mut Criterion) {
    c.bench_function("do_delay_100", |b| {
        b.iter(|| delay::do_delay(b"test preimage"))
    });

    // example code
    // c.bench_function("fib", |b| b.iter(|| fibonacci(1)));
}

// sample size configs not documented. Found here: https://github.com/bheisler/criterion.rs/issues/407
criterion_group! {
    name = ol_benches;
    config = Criterion::default().sample_size(10);  // sampling size
    targets = bench_delay
}

criterion_main!(ol_benches);
