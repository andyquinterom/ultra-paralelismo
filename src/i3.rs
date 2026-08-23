use rayon::prelude::*;
use std::ops::Div;

pub fn desviacion_estandar(data: &[f64]) -> f64 {
    const TAMAÑO_GRUPO: usize = 4096;
    let n = data.len() as f64;
    let suma = data.par_iter().with_min_len(TAMAÑO_GRUPO).sum::<f64>();
    let media = suma / n;

    data.par_iter()
        .with_min_len(TAMAÑO_GRUPO)
        .map(|value| {
            let diff = value - media;
            diff * diff
        })
        .sum::<f64>()
        .div(n)
        .sqrt()
}
