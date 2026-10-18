use std::simd::Simd;

const LANES: usize = 8;

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
}

pub fn desviacion_estandar(data: &[f64]) -> f64 {
    let (prefijo, vectores, sufijo) = data.as_simd::<LANES>();

    // Cada carril mantiene su propio estado de Welford, con la misma
    // recurrencia que la versión pura pero vectorizada: el elemento `i` del
    // vector actualiza al carril `i`. Así cada carril acumula la subsecuencia
    // `i, i + LANES, i + 2 * LANES, ...`.
    let mut n = Simd::splat(0.0);
    let mut media = Simd::<f64, LANES>::splat(0.0);
    let mut s = Simd::<f64, LANES>::splat(0.0);

    for &vector in vectores {
        n += Simd::splat(1.0);
        let delta = vector - media;
        media += delta / n;
        let delta2 = vector - media;
        s += delta * delta2;
    }

    // Los carriles se combinan con la fórmula de Chan, igual que en la versión
    // paralela.
    let medias = media.to_array();
    let valores_s = s.to_array();
    let n = n.to_array()[0];

    let mut estado = Estado::default();
    for i in 0..LANES {
        estado = estado.combinar(Estado {
            n,
            media: medias[i],
            s: valores_s[i],
        });
    }

    // Los elementos que no completan un vector se procesan con la recurrencia
    // escalar.
    for &valor in prefijo.iter().chain(sufijo) {
        estado = estado.welford(valor);
    }

    if estado.n == 0.0 {
        return 0.0;
    }

    (estado.s / estado.n).max(0.0).sqrt()
}
