use criterion::{black_box, criterion_group, criterion_main, Criterion};

use pprof::criterion::Output;
use test_criterion::fibonacci_rec;

pub fn rec_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20 rec", |b| b.iter(|| fibonacci_rec(black_box(20))));
}

fn profiled() -> Criterion {
    Criterion::default().with_profiler(pprof::criterion::PProfProfiler::new(
        100,
        Output::Flamegraph(None),
    ))
}

criterion_group! {
    name = benches;
    config = profiled();
    targets = rec_benchmark
}
criterion_main!(benches);
