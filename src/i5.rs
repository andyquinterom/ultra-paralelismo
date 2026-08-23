use rayon::prelude::*;
use std::{
    ops::Div,
    simd::{Simd, num::SimdFloat},
};

const LANES: usize = 4;
const TAMAÑO_GRUPO: usize = 4096;

#[inline]
fn suma_chunk(chunk: &[f64]) -> f64 {
    let mut suma = Simd::<f64, LANES>::splat(0.0);
    let n = chunk.len();
    let mut i = 0;
    while i < (n / LANES) * LANES {
        suma += Simd::from_slice(unsafe { &chunk.get_unchecked(i..i + LANES) });
        i += LANES;
    }
    let mut suma = suma.reduce_sum();
    while i < n {
        suma += unsafe { chunk.get_unchecked(i) };
        i += 1;
    }
    suma
}

#[inline]
fn suma_diff_cuadrado_chunk(chunk: &[f64], media: f64, media_splat: Simd<f64, LANES>) -> f64 {
    let mut suma = Simd::<f64, LANES>::splat(0.0);
    let n = chunk.len();
    let mut i = 0;
    while i < (n / LANES) * LANES {
        let valores = Simd::from_slice(unsafe { &chunk.get_unchecked(i..i + LANES) });
        let diff = valores - media_splat;
        suma += diff * diff;
        i += LANES;
    }
    let mut suma = suma.reduce_sum();
    while i < n {
        let diff = unsafe { chunk.get_unchecked(i) } - media;
        suma += diff * diff;
        i += 1;
    }
    suma
}

pub fn desviacion_estandar(data: &[f64]) -> f64 {
    let n = data.len() as f64;
    let suma = data.par_chunks(TAMAÑO_GRUPO).map(suma_chunk).sum::<f64>();
    let media = suma / n;
    let media_splat = Simd::<f64, LANES>::splat(media);

    data.par_chunks(TAMAÑO_GRUPO)
        .map(|chunk| suma_diff_cuadrado_chunk(chunk, media, media_splat))
        .sum::<f64>()
        .div(n)
        .sqrt()
}
