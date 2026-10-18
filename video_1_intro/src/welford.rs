#[derive(Clone, Copy, Default)]
struct Estado {
    n: f64,
    media: f64,
    s: f64,
}

impl Estado {
    #[inline]
    fn welford(self, valor: f64) -> Self {
        let n = self.n + 1.0;
        let delta = valor - self.media;
        let media = self.media + delta / n;
        let delta2 = valor - media;
        Self {
            n,
            media,
            s: self.s + delta * delta2,
        }
    }
}

pub fn desviacion_estandar(data: &[f64]) -> f64 {
    let mut estado = Estado::default();
    for &valor in data {
        estado = estado.welford(valor);
    }

    if estado.n == 0.0 {
        return 0.0;
    }

    (estado.s / estado.n).max(0.0).sqrt()
}
