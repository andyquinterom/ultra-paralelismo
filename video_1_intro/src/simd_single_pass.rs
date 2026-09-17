use std::simd::{Simd, num::SimdFloat};

const LANES: usize = 8;

pub fn desviacion_estandar(data: &[f64]) -> f64 {
    let n = data.len() as f64;

    let (prefijo, vectores, sufijo) = data.as_simd::<LANES>();

    let mut suma_vec = Simd::<f64, LANES>::splat(0.0);
    let mut suma_cuadrados_vec = Simd::<f64, LANES>::splat(0.0);

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

    let media = suma / n;
    let varianza = (suma_cuadrados / n) - media * media;
    varianza.max(0.0).sqrt()
}
