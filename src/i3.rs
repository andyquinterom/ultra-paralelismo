use std::{ops::{Div, Mul}, simd::{Simd, StdFloat, num::SimdFloat}};

const LANES: usize = 8;

pub fn desviacion_estandar(data: &[f64]) -> f64 {
    let n = data.len() as f64;
    
    let (chunks, residuo) =  &data.as_chunks::<LANES>();
    
    let (suma, suma_cuadrados) = chunks.iter()
        .copied()
        .map(Simd::from_array)
        .fold((Simd::<f64, LANES>::splat(0.0), Simd::<f64, LANES>::splat(0.0)), |(suma, suma_cuadrados), valor| {
            (suma + valor, suma_cuadrados + valor * valor)
        });
    
    let suma = suma.reduce_sum();
    let suma_cuadrados = suma_cuadrados.reduce_sum();
    
    let (suma, suma_cuadrados) = residuo.iter().fold((suma, suma_cuadrados), |(suma, suma_cuadrados), &valor| {
        (suma + valor, suma_cuadrados + valor * valor)
    });

    let media = suma / n;
    let varianza = (suma_cuadrados / n) - media * media;
    varianza.max(0.0).sqrt()
}
