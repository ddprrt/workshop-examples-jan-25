use criterion::{black_box, criterion_group, criterion_main, Criterion};

use test_criterion::fibonacci;

pub fn fn_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(benches, fn_benchmark);
criterion_main!(benches);
