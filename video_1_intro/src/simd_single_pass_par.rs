use rayon::prelude::*;
use std::simd::{Simd, num::SimdFloat};

const LANES: usize = 8;
const TAMAÑO_BLOQUE: usize = 32 * 1024 / size_of::<f64>();


pub fn desviacion_estandar(data: &[f64]) -> f64 {
    let n = data.len() as f64;

    let (suma, suma_cuadrados) = data
        .par_chunks(TAMAÑO_BLOQUE)
        .map(|bloque| {
            let (prefijo, vectores, sufijo) = bloque.as_simd::<LANES>();

            let mut suma_vec = Simd::splat(0.0);
            let mut suma_cuadrados_vec = Simd::splat(0.0);

            for &vector in vectores {
                suma_vec += vector;
                suma_cuadrados_vec += vector * vector;
            }

            let mut suma = suma_vec.reduce_sum();
            let mut suma_cuadrados = suma_cuadrados_vec.reduce_sum();

            for &valor in prefijo.iter().chain(sufijo) {
                suma += valor;
                suma_cuadrados += valor * valor;
            }

            (suma, suma_cuadrados)
        })
        .reduce(
            || (0.0, 0.0),
            |(suma_a, suma_cuadrados_a), (suma_b, suma_cuadrados_b)| {
                (suma_a + suma_b, suma_cuadrados_a + suma_cuadrados_b)
            },
        );

    let media = suma / n;
    let varianza = (suma_cuadrados / n) - media * media;
    varianza.max(0.0).sqrt()
}
