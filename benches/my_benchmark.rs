use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hanoi::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("hanoi 14", |b| b.iter(|| run_hanoi(black_box(14))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
