use rayon::prelude::*;
use std::{
    ops::{Div, Mul},
    simd::{Simd, StdFloat, num::SimdFloat},
};

use rayon::iter::IntoParallelRefIterator;

const LANES: usize = 8;

pub fn desviacion_estandar(data: &[f64]) -> f64 {
    const CERO: Simd<f64, LANES> = Simd::splat(0.0);
    let n = data.len() as f64;

    let (chunks, residuo) = &data.as_chunks::<LANES>();

    let (suma, suma_cuadrados) = chunks
        .par_iter()
        .with_min_len(1024)
        .copied()
        .map(Simd::from_array)
        .fold(
            || (CERO, CERO),
            |(suma, suma_cuadrados), valor| (suma + valor, suma_cuadrados + valor * valor),
        )
        .reduce(
            || (CERO, CERO),
            |(mut suma_a, mut suma_cuadrados_a), (suma_b, suma_cuadrados_b)| {
                suma_a += suma_b;
                suma_cuadrados_a += suma_cuadrados_b;
                (suma_a, suma_cuadrados_a)
            },
        );

    let suma = suma.reduce_sum();
    let suma_cuadrados = suma_cuadrados.reduce_sum();

    let (suma, suma_cuadrados) = residuo
        .iter()
        .fold((suma, suma_cuadrados), |(suma, suma_cuadrados), &valor| {
            (suma + valor, suma_cuadrados + valor * valor)
        });

    let media = suma / n;
    let varianza = (suma_cuadrados / n) - media * media;
    varianza.max(0.0).sqrt()
}
