use std::ops::Div;

pub fn desviacion_estandar(data: &[f64]) -> f64 {
    let n = data.len() as f64;
    let suma = data.iter().sum::<f64>();
    let media = suma / n;

    data.iter()
        .map(|value| (value - media).powf(2.0))
        .sum::<f64>()
        .div(n)
        .sqrt()
}
