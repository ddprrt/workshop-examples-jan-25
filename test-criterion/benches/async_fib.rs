use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tokio::runtime::Builder;

pub async fn fibonacci(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;

    match n {
        0 => b,
        _ => {
            for _ in 0..n {
                let c = a + b;
                a = b;
                b = c;
            }
            b
        }
    }
}

pub fn fn_benchmark(c: &mut Criterion) {
    let runtime = Builder::new_current_thread().build().unwrap();
    c.bench_function("fib 20", |b| {
        b.to_async(&runtime).iter(|| fibonacci(black_box(20)))
    });
}

criterion_group!(benches, fn_benchmark);
criterion_main!(benches);
