use rayon::prelude::*;
use std::simd::{Simd, num::SimdFloat};

use rayon::iter::IntoParallelRefIterator;

const LANES: usize = 8;

pub fn desviacion_estandar(data: &[f64]) -> f64 {
    const CERO: Simd<f64, LANES> = Simd::splat(0.0);
    let n = data.len() as f64;

    let (chunks, residuo) = &data.as_chunks::<LANES>();

    let mut suma = chunks
        .par_iter()
        .with_min_len(1024)
        .copied()
        .map(Simd::from_array)
        .fold(|| CERO, |acc, valor| acc + valor)
        .reduce(|| CERO, |a, b| a + b)
        .reduce_sum();

    for &valor in residuo.iter() {
        suma += valor;
    }

    let media = suma / n;

    let media_vec = Simd::<f64, LANES>::splat(media);
    let mut suma_cuadrados = chunks
        .par_iter()
        .with_min_len(1024)
        .copied()
        .map(Simd::from_array)
        .fold(
            || CERO,
            |acc, valor| {
                let diff = valor - media_vec;
                acc + diff * diff
            },
        )
        .reduce(|| CERO, |a, b| a + b)
        .reduce_sum();

    for &valor in residuo.iter() {
        let diff = valor - media;
        suma_cuadrados += diff * diff;
    }

    (suma_cuadrados / n).sqrt()
}
