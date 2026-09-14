use std::ops::Div;

pub fn desviacion_estandar(data: &[f64]) -> f64 {
    let n = data.len() as f64;
    let suma = data.iter().sum::<f64>();
    let media = suma / n;

    data.iter()
        .map(|value| {
            let diff = value - media;
            diff * diff
        })
        .sum::<f64>()
        .div(n)
        .sqrt()
}
