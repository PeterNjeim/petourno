use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn reverse_fractal(_n: i32, k: i32) -> i32 {
    k / 4
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("rfr 20", |b| b.iter(|| reverse_fractal(-6, black_box(21))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
