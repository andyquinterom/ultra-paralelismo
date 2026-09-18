use std::simd::{Simd, num::SimdFloat};

const LANES: usize = 4;

pub fn desviacion_estandar(data: &[f64]) -> f64 {
    let n = data.len() as f64;

    let (prefijo, vectores, sufijo) = data.as_simd::<LANES>();

    let mut suma_vec = Simd::splat(0.0);
    for &vector in vectores {
        suma_vec += vector;
    }

    let mut suma = suma_vec.reduce_sum();
    for &valor in prefijo.iter().chain(sufijo) {
        suma += valor;
    }

    let media = suma / n;

    let media_vec = Simd::<f64, LANES>::splat(media);
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

    (suma_cuadrados / n).sqrt()
}
