use criterion::{criterion_group, criterion_main, Criterion};
use hydrolox_pga3d::{motor::Motor, point::Point};
use rand::Rng;
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

    let mut rng = rand::rng();

    c.bench_function("random combine and transform", |b| {
        b.iter(|| {
            let axis: (f32, f32, f32) = rng.random();
            let magnitude = (axis.0 * axis.0 + axis.1 * axis.1 + axis.2 * axis.2).sqrt();
            Motor::from_translation(rng.random(), rng.random(), rng.random())
                .combine(Motor::from_rotation_around_axis(
                    axis.0 / magnitude,
                    axis.1 / magnitude,
                    axis.2 / magnitude,
                    rng.random(),
                ))
                .transform(Point::from_position(
                    rng.random(),
                    rng.random(),
                    rng.random(),
                ));
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
