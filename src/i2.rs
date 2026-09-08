pub fn desviacion_estandar(data: &[f64]) -> f64 {
    let n = data.len() as f64;

    let (suma, suma_cuadrados) = data
        .iter()
        .fold((0.0, 0.0), |(suma, suma_cuadrados), &valor| {
            (suma + valor, suma_cuadrados + valor * valor)
        });

    let media = suma / n;
    let varianza = (suma_cuadrados / n) - media * media;
    varianza.max(0.0).sqrt()
}
