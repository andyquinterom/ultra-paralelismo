use criterion::{Criterion, criterion_group, criterion_main};
use std::{hint::black_box, time::Duration};
use ultra_paralelismo::{
    secuencial, secuencial_single_pass, welford, welford_simd, simd_single_pass, simd_single_pass_par,
    modified_welford_chan_simd, modified_welford_chan_simd_par, simd_double_pass_2, simd_double_pass_4, simd_double_pass_8,
    simd_double_pass_8_par, simd_desplazado, simd_dos_pasadas_bloque,
};

fn configure_criterion() -> Criterion {
    Criterion::default()
        .sample_size(10)
        .measurement_time(Duration::from_secs(10))
}

fn criterion_benchmark(c: &mut Criterion) {
    rayon::ThreadPoolBuilder::new().num_threads(12).build_global().unwrap();
    let datos = (0..1_000_000_000)
        .map(|_| rand::random::<f64>())
        .collect::<Vec<f64>>();
    c.bench_function("secuencial", |b| {
        b.iter(|| secuencial::desviacion_estandar(black_box(&datos)))
    });
    c.bench_function("secuencial_single_pass", |b| {
        b.iter(|| secuencial_single_pass::desviacion_estandar(black_box(&datos)))
    });
    c.bench_function("welford", |b| {
        b.iter(|| welford::desviacion_estandar(black_box(&datos)))
    });
    c.bench_function("welford_simd", |b| {
        b.iter(|| welford_simd::desviacion_estandar(black_box(&datos)))
    });
    c.bench_function("simd_single_pass", |b| {
        b.iter(|| simd_single_pass::desviacion_estandar(black_box(&datos)))
    });
    c.bench_function("simd_single_pass_par", |b| {
        b.iter(|| simd_single_pass_par::desviacion_estandar(black_box(&datos)))
    });
    c.bench_function("modified_welford_chan_simd", |b| {
        b.iter(|| modified_welford_chan_simd::desviacion_estandar(black_box(&datos)))
    });
    c.bench_function("modified_welford_chan_simd_par", |b| {
        b.iter(|| modified_welford_chan_simd_par::desviacion_estandar(black_box(&datos)))
    });
    c.bench_function("simd_double_pass_2", |b| {
        b.iter(|| simd_double_pass_2::desviacion_estandar(black_box(&datos)))
    });
    c.bench_function("simd_double_pass_4", |b| {
        b.iter(|| simd_double_pass_4::desviacion_estandar(black_box(&datos)))
    });
    c.bench_function("simd_double_pass_8", |b| {
        b.iter(|| simd_double_pass_8::desviacion_estandar(black_box(&datos)))
    });
    c.bench_function("simd_double_pass_8_par", |b| {
        b.iter(|| simd_double_pass_8_par::desviacion_estandar_8_par(black_box(&datos)))
    });
    c.bench_function("simd_desplazado", |b| {
        b.iter(|| simd_desplazado::desviacion_estandar(black_box(&datos)))
    });
    c.bench_function("simd_desplazado_par", |b| {
        b.iter(|| simd_desplazado::desviacion_estandar_par(black_box(&datos)))
    });
    c.bench_function("simd_dos_pasadas_bloque", |b| {
        b.iter(|| simd_dos_pasadas_bloque::desviacion_estandar(black_box(&datos)))
    });
    c.bench_function("simd_dos_pasadas_bloque_par", |b| {
        b.iter(|| simd_dos_pasadas_bloque::desviacion_estandar_par(black_box(&datos)))
    });
}

criterion_group!(
    name = benches;
    config = configure_criterion();
    targets = criterion_benchmark
);
criterion_main!(benches);
