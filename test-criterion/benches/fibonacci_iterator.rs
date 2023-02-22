use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use pprof::criterion::Output;
use test_criterion::{fibonacci_iter, Fibonacci};

pub fn iterator_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| {
        b.iter(|| Fibonacci::default().nth(black_box(20)).unwrap())
    });
}

fn _iterator_benchmark_group(c: &mut Criterion) {
    let mut group = c.benchmark_group("Fibonacci Sizes");

    for s in &[1, 10] {
        group.bench_with_input(BenchmarkId::from_parameter(s), s, |b, s| {
            b.iter(|| {
                Fibonacci::default().nth(black_box(*s));
                fibonacci_iter(black_box(*s));
            })
        });
    }
}

pub fn iterator_benchmark_fn(c: &mut Criterion) {
    c.bench_function("fib 20fn", |b| b.iter(|| fibonacci_iter(black_box(20))));
}

fn _profiled() -> Criterion {
    Criterion::default().with_profiler(pprof::criterion::PProfProfiler::new(
        100,
        Output::Flamegraph(None),
    ))
}
/*
criterion_group! {
    name = benches;
    config = profiled();
    targets = iterator_benchmark, iterator_benchmark_fn, iterator_benchmark_group
}*/
criterion_group!(benches, iterator_benchmark, iterator_benchmark_fn);
criterion_main!(benches);

// https://github.com/rust-lang/rust/issues/107617
