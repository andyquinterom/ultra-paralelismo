use rayon::prelude::*;
use std::simd::{Simd, num::SimdFloat};

const LANES: usize = 8;
const TAMAÑO_BLOQUE: usize = 16 * 1024 / size_of::<f64>();

#[derive(Clone, Copy, Default)]
struct Estado {
    n: f64,
    media: f64,
    s: f64,
}

impl Estado {
    #[inline]
    fn combinar(self, otro: Self) -> Self {
        if self.n == 0.0 {
            return otro;
        }
        if otro.n == 0.0 {
            return self;
        }
        let n = self.n + otro.n;
        let delta = otro.media - self.media;
        let media = self.media + delta * (otro.n / n);
        let s = self.s + otro.s + delta * delta * (self.n * otro.n / n);
        Self { n, media, s }
    }
    fn cerrar(self) -> f64 {
        if self.n == 0.0 {
            return 0.0;
        }
        (self.s / self.n).max(0.0).sqrt()
    }
}

fn bloque_dos_pasadas(data: &[f64]) -> Estado {
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

    Estado {
        n,
        media,
        s: suma_cuadrados,
    }
}


pub fn desviacion_estandar(data: &[f64]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }

    bloque_dos_pasadas(data).cerrar()
}

pub fn desviacion_estandar_par(data: &[f64]) -> f64 {
    data
        .par_chunks(TAMAÑO_BLOQUE)
        .map(bloque_dos_pasadas)
        .reduce(Estado::default, Estado::combinar)
        .cerrar()
}
