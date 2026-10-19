use std::simd::{Simd, StdFloat};

const LANES: usize = 8;
const VECTORES_POR_MICRO_BLOQUE: usize = 128;

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

fn welford_bloque(bloque: &[f64]) -> Estado {
    let (prefijo, vectores, sufijo) = bloque.as_simd::<LANES>();

    let mut n_run = 0.0;
    let mut media_run = Simd::<f64, LANES>::splat(0.0);
    let mut s_run = Simd::<f64, LANES>::splat(0.0);

    for grupo in vectores.chunks(VECTORES_POR_MICRO_BLOQUE) {
        // Desplazamos por el primer vector del grupo: cada carril usa como
        // referencia su propio dato real, lo que evita la cancelación
        // catastrófica al acumular los cuadrados.
        let referencia = grupo[0];
        let mut suma_vec = Simd::<f64, LANES>::splat(0.0);
        let mut suma_cuadrados_vec = Simd::<f64, LANES>::splat(0.0);

        for &vector in grupo {
            let d = vector - referencia;
            suma_vec += d;
            suma_cuadrados_vec = d.mul_add(d, suma_cuadrados_vec);
        }

        let k = grupo.len() as f64;
        let media_d = suma_vec * Simd::splat(1.0 / k);
        let s_bloque = suma_cuadrados_vec - suma_vec * media_d;
        let media_bloque = referencia + media_d;

        if n_run == 0.0 {
            media_run = media_bloque;
            s_run = s_bloque;
            n_run = k;
        } else {
            let n_nuevo = n_run + k;
            let delta = media_bloque - media_run;
            media_run += delta * Simd::splat(k / n_nuevo);
            s_run += s_bloque + delta * delta * Simd::splat(n_run * k / n_nuevo);
            n_run = n_nuevo;
        }
    }

    let medias = media_run.to_array();
    let valores_s = s_run.to_array();

    let mut estado = Estado::default();
    for i in 0..LANES {
        estado = estado.combinar(Estado {
            n: n_run,
            media: medias[i],
            s: valores_s[i],
        });
    }

    for &valor in prefijo.iter().chain(sufijo) {
        estado = estado.welford(valor);
    }

    estado
}

pub fn desviacion_estandar(data: &[f64]) -> f64 {
    let estado = welford_bloque(data);

    if estado.n == 0.0 {
        return 0.0;
    }

    (estado.s / estado.n).max(0.0).sqrt()
}
