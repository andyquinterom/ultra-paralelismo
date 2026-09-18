use rayon::prelude::*;
use std::simd::{Simd, num::SimdFloat};

const LANES: usize = 8;
const TAMAÑO_BLOQUE: usize = 32 * 1024 / size_of::<f64>();

pub fn desviacion_estandar_8_par(data: &[f64]) -> f64 {
    let n = data.len() as f64;

    let suma = data
        .par_chunks(TAMAÑO_BLOQUE)
        .map(|bloque| {
            let (prefijo, vectores, sufijo) = bloque.as_simd::<LANES>();

            let mut suma_vec = Simd::splat(0.0);
            for &vector in vectores {
                suma_vec += vector;
            }

            let mut suma = suma_vec.reduce_sum();
            for &valor in prefijo.iter().chain(sufijo) {
                suma += valor;
            }

            suma
        })
        .reduce(|| 0.0, |a, b| a + b);

    let media = suma / n;

    let media_vec = Simd::<f64, LANES>::splat(media);
    let suma_cuadrados = data
        .par_chunks(TAMAÑO_BLOQUE)
        .map(|bloque| {
            let (prefijo, vectores, sufijo) = bloque.as_simd::<LANES>();

            let mut suma_cuadrados_vec = Simd::splat(0.0);
            for &vector in vectores {
                let diff = vector - media_vec;
                suma_cuadrados_vec += diff * diff;
            }

            let mut suma_cuadrados = suma_cuadrados_vec.reduce_sum();
            for &valor in prefijo.iter().chain(sufijo) {
                let diff = valor - media;
                suma_cuadrados += diff * diff;
            }

            suma_cuadrados
        })
        .reduce(|| 0.0, |a, b| a + b);

    (suma_cuadrados / n).sqrt()
}
