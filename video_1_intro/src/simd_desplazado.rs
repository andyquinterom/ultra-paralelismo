use rayon::prelude::*;
use std::simd::{Simd, StdFloat, num::SimdFloat};

const LANES: usize = 8;
const TAMAÑO_BLOQUE: usize = 32 * 1024 / size_of::<f64>();

/// Acumula sobre un bloque la suma de las desviaciones respecto a `k` (`s1`) y
/// la suma de sus cuadrados (`s2`), usando SIMD y FMA.
///
/// Restar una constante `k` cercana a la media reduce la cancelación
/// catastrófica: los valores desplazados `d = x - k` son pequeños, de modo que
/// `d * d` conserva precisión aunque `x` sea enorme.
fn acumular(bloque: &[f64], k: f64) -> (f64, f64, f64) {
    let (prefijo, vectores, sufijo) = bloque.as_simd::<LANES>();
    let kv = Simd::<f64, LANES>::splat(k);
    let mut s1 = Simd::<f64, LANES>::splat(0.0);
    let mut s2 = Simd::<f64, LANES>::splat(0.0);

    for &v in vectores {
        let d = v - kv;
        s1 += d;
        s2 = d.mul_add(d, s2); // FMA: StdFloat
    }

    let (mut a, mut b) = (s1.reduce_sum(), s2.reduce_sum());
    for &x in prefijo.iter().chain(sufijo) {
        let d = x - k;
        a += d;
        b += d * d;
    }
    (bloque.len() as f64, a, b)
}

/// Estima `k` como la media de una muestra pequeña y espaciada. El costo es
/// despreciable y basta para acercar `k` a la media real.
fn estimar_k(data: &[f64]) -> f64 {
    let paso = (data.len() / 4096).max(1);
    let (suma, cuenta) = data
        .iter()
        .step_by(paso)
        .fold((0.0, 0.0), |(s, c), &x| (s + x, c + 1.0));
    suma / cuenta
}

/// A partir de `(n, s1, s2)` calcula la desviación estándar corrigiendo el
/// desplazamiento introducido por `k`.
#[inline]
fn cerrar(n: f64, s1: f64, s2: f64) -> f64 {
    let media_d = s1 / n;
    (s2 / n - media_d * media_d).max(0.0).sqrt()
}

/// Versión de un solo hilo.
pub fn desviacion_estandar(data: &[f64]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }

    let k = estimar_k(data);
    let (n, s1, s2) = acumular(data, k);
    cerrar(n, s1, s2)
}

/// Versión paralela: reparte los datos en bloques con rayon.
pub fn desviacion_estandar_par(data: &[f64]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }

    let k = estimar_k(data);
    let (n, s1, s2) = data
        .par_chunks(TAMAÑO_BLOQUE)
        .map(|bloque| acumular(bloque, k))
        .reduce(|| (0.0, 0.0, 0.0), |a, b| (a.0 + b.0, a.1 + b.1, a.2 + b.2));

    cerrar(n, s1, s2)
}
