use criterion::{Criterion, criterion_group, criterion_main};
use std::{hint::black_box, time::Duration};
use ultra_paralelismo::{i1, i2, i3, i4};

fn configure_criterion() -> Criterion {
    Criterion::default()
        .sample_size(10)
        .measurement_time(Duration::from_secs(10))
}

fn criterion_benchmark(c: &mut Criterion) {
    let datos = (0..1_000_000_000)
        .map(|_| rand::random::<f64>())
        .collect::<Vec<f64>>();
    c.bench_function("i1", |b| {
        b.iter(|| i1::desviacion_estandar(black_box(&datos)))
    });
    c.bench_function("i2", |b| {
        b.iter(|| i2::desviacion_estandar(black_box(&datos)))
    });
    c.bench_function("i3", |b| {
        b.iter(|| i3::desviacion_estandar(black_box(&datos)))
    });
    c.bench_function("i4", |b| {
        b.iter(|| i4::desviacion_estandar(black_box(&datos)))
    });
}

criterion_group!(
    name = benches;
    config = configure_criterion();
    targets = criterion_benchmark
);
criterion_main!(benches);
