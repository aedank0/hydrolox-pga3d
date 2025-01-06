use criterion::{criterion_group, criterion_main, Criterion};
use hydrolox_pga3d::point::Point;
use std::hint::black_box;

fn criterion_benchmark(c: &mut Criterion) {
    let p1 = Point::new(5.0, 6.0, 7.0, 1.0);
    let p2 = Point::new(6.0, 7.0, 8.0, 1.0);

    c.bench_function("join points", |b| {
        b.iter(|| black_box(p1).join(black_box(p2)))
    });

    c.bench_function("dot points", |b| {
        b.iter(|| black_box(p1).dot(black_box(p2)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
