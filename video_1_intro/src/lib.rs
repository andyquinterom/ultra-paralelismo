#![feature(portable_simd)]

pub mod secuencial;
pub mod secuencial_single_pass;
pub mod welford;
pub mod welford_simd;
pub mod simd_single_pass;
pub mod simd_single_pass_par;
pub mod modified_welford_chan_simd;
pub mod modified_welford_chan_simd_par;
pub mod simd_double_pass_2;
pub mod simd_double_pass_4;
pub mod simd_double_pass_8;
pub mod simd_double_pass_8_par;
pub mod simd_desplazado;
pub mod simd_dos_pasadas_bloque;

pub use std::simd;

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    type Implementacion = (&'static str, fn(&[f64]) -> f64);

    const IMPLEMENTACIONES: &[Implementacion] = &[
        ("secuencial", secuencial::desviacion_estandar),
        ("secuencial_single_pass", secuencial_single_pass::desviacion_estandar),
        ("welford", welford::desviacion_estandar),
        ("welford_simd", welford_simd::desviacion_estandar),
        ("simd_single_pass", simd_single_pass::desviacion_estandar),
        ("simd_single_pass_par", simd_single_pass_par::desviacion_estandar),
        ("modified_welford_chan_simd", modified_welford_chan_simd::desviacion_estandar),
        ("modified_welford_chan_simd_par", modified_welford_chan_simd_par::desviacion_estandar),
        ("simd_double_pass_2", simd_double_pass_2::desviacion_estandar),
        ("simd_double_pass_4", simd_double_pass_4::desviacion_estandar),
        ("simd_double_pass_8", simd_double_pass_8::desviacion_estandar),
        ("simd_double_pass_8_par", simd_double_pass_8_par::desviacion_estandar_8_par),
        ("simd_desplazado", simd_desplazado::desviacion_estandar),
        ("simd_desplazado_par", simd_desplazado::desviacion_estandar_par),
        ("simd_dos_pasadas_bloque", simd_dos_pasadas_bloque::desviacion_estandar),
        ("simd_dos_pasadas_bloque_par", simd_dos_pasadas_bloque::desviacion_estandar_par),
    ];

    #[test]
    fn test_desviacion_estandar_datos_basicos() {
        let datos = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
        for (nombre, func) in IMPLEMENTACIONES {
            let resultado = func(&datos);
            assert!(
                (resultado - 2.0).abs() < 1e-10,
                "Falló implementación: {}",
                nombre
            );
        }
    }

    #[test]
    fn test_desviacion_estandar_valores_iguales() {
        let datos = vec![5.0, 5.0, 5.0, 5.0];
        for (nombre, func) in IMPLEMENTACIONES {
            let resultado = func(&datos);
            assert!(
                (resultado - 0.0).abs() < 1e-10,
                "Falló implementación: {}",
                nombre
            );
        }
    }

    #[test]
    fn test_desviacion_estandar_un_elemento() {
        let datos = vec![10.0];
        for (nombre, func) in IMPLEMENTACIONES {
            let resultado = func(&datos);
            assert!(
                (resultado - 0.0).abs() < 1e-10,
                "Falló implementación: {}",
                nombre
            );
        }
    }

    #[test]
    fn test_desviacion_estandar_dos_elementos() {
        let datos = vec![3.0, 7.0];
        for (nombre, func) in IMPLEMENTACIONES {
            let resultado = func(&datos);
            assert!(
                (resultado - 2.0).abs() < 1e-10,
                "Falló implementación: {}",
                nombre
            );
        }
    }

    #[test]
    fn test_desviacion_estandar_valores_negativos() {
        let datos = vec![-2.0, -4.0, -4.0, -4.0, -5.0, -5.0, -7.0, -9.0];
        for (nombre, func) in IMPLEMENTACIONES {
            let resultado = func(&datos);
            assert!(
                (resultado - 2.0).abs() < 1e-10,
                "Falló implementación: {}",
                nombre
            );
        }
    }

    #[test]
    fn test_desviacion_estandar_valores_mixtos() {
        let datos = vec![-5.0, 0.0, 5.0];
        let esperado = (50.0_f64 / 3.0_f64).sqrt();
        for (nombre, func) in IMPLEMENTACIONES {
            let resultado = func(&datos);
            assert!(
                (resultado - esperado).abs() < 1e-10,
                "Falló implementación: {}",
                nombre
            );
        }
    }

    /// Test de consistencia: todas las implementaciones deben dar el mismo resultado
    #[test]
    fn test_consistencia_entre_implementaciones() {
        let casos_de_prueba = vec![
            vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0],
            vec![5.0, 5.0, 5.0, 5.0],
            vec![10.0],
            vec![3.0, 7.0],
            vec![-2.0, -4.0, -4.0, -4.0, -5.0, -5.0, -7.0, -9.0],
            vec![-5.0, 0.0, 5.0],
        ];

        for datos in casos_de_prueba {
            let resultados: Vec<f64> = IMPLEMENTACIONES
                .iter()
                .map(|(_, func)| func(&datos))
                .collect();

            // Todos los resultados deben ser iguales
            let primero = resultados[0];
            for (i, resultado) in resultados.iter().enumerate() {
                assert!(
                    (primero - resultado).abs() < 1e-10,
                    "Inconsistencia en datos {:?}: impl 0 = {}, impl {} = {}",
                    datos,
                    primero,
                    i,
                    resultado
                );
            }
        }
    }

    // ------------------------------------------------------------------
    // Helpers para la validación profunda
    // ------------------------------------------------------------------

    /// Referencia estable de doble pasada en `f64`.
    fn referencia_two_pass(data: &[f64]) -> f64 {
        let n = data.len() as f64;
        let media = data.iter().sum::<f64>() / n;
        let varianza = data.iter().map(|v| (v - media) * (v - media)).sum::<f64>() / n;
        varianza.max(0.0).sqrt()
    }

    /// Compara con tolerancia relativa, evitando exigir precisión absoluta
    /// alrededor de cero.
    fn assert_cercano(obtenido: f64, esperado: f64, tol: f64, contexto: &str) {
        let escala = esperado.abs().max(1.0);
        assert!(
            (obtenido - esperado).abs() <= tol * escala,
            "{contexto}: obtenido {obtenido}, esperado {esperado} (tol {tol})"
        );
    }

    /// Generador determinista (LCG) de valores en `[0, 1)`, sin dependencias.
    fn lcg_valores(n: usize, semilla: u64) -> Vec<f64> {
        let mut estado = semilla;
        (0..n)
            .map(|_| {
                estado = estado
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                (estado >> 11) as f64 / (1u64 << 53) as f64
            })
            .collect()
    }

    /// Ejecuta todas las implementaciones contra la referencia estable.
    fn comparar_todas(datos: &[f64], tol: f64, contexto: &str) {
        let esperado = referencia_two_pass(datos);
        for (nombre, func) in IMPLEMENTACIONES {
            let obtenido = func(datos);
            assert_cercano(obtenido, esperado, tol, &format!("{contexto} / {nombre}"));
        }
    }

    // ------------------------------------------------------------------
    // Tests de validación profunda
    // ------------------------------------------------------------------

    /// Recorre todas las longitudes que cruzan los límites de LANES (2, 4, 8),
    /// ejercitando el prefijo y el sufijo SIMD.
    #[test]
    fn test_consistencia_longitudes_frontera() {
        for n in 1..=40usize {
            let datos = lcg_valores(n, n as u64 + 1);
            comparar_todas(&datos, 1e-12, &format!("longitud {n}"));
        }
    }

    /// Las versiones paralelas dividen en bloques de `TAMAÑO_BLOQUE`
    /// (32 KiB / 8 = 4096 elementos). Se cubren ambos lados del límite y un
    /// último bloque parcial.
    #[test]
    fn test_limites_bloque_paralelas() {
        for n in [4095usize, 4096, 4097, 4096 * 3 + 7] {
            let datos = lcg_valores(n, n as u64);
            comparar_todas(&datos, 1e-9, &format!("bloque {n}"));
        }
    }

    /// Compara todas las implementaciones sobre volúmenes crecientes de datos
    /// pseudoaleatorios.
    #[test]
    fn test_datos_aleatorios_grandes() {
        for n in [1_000usize, 100_000, 1_000_000] {
            let datos = lcg_valores(n, 0xDEAD_BEEF ^ n as u64);
            comparar_todas(&datos, 1e-9, &format!("aleatorio {n}"));
        }
    }

    /// Valores con desviación estándar calculable a mano.
    #[test]
    fn test_valores_conocidos() {
        let casos: &[(&[f64], f64)] = &[
            (&[1.0, 2.0, 3.0, 4.0], 1.25_f64.sqrt()),
            (&[1.0, 2.0, 3.0, 4.0, 5.0], 2.0_f64.sqrt()),
            (&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0], (60.0_f64 / 9.0).sqrt()),
            (&[0.0, 0.0, 0.0, 0.0], 0.0),
            (&[1.0, -1.0], 1.0),
        ];
        for (datos, esperado) in casos {
            for (nombre, func) in IMPLEMENTACIONES {
                assert_cercano(
                    func(datos),
                    *esperado,
                    1e-12,
                    &format!("valores conocidos {datos:?} / {nombre}"),
                );
            }
        }
    }

    /// Cancelación catastrófica: valores grandes y muy próximos entre sí.
    /// Las implementaciones de doble pasada deben mantenerse estables; las de
    /// una sola pasada / Welford no se comparan con precisión, pero sí se exige
    /// que devuelvan un valor finito y no negativo (sin `NaN`).
    #[test]
    fn test_estabilidad_cancelacion() {
        let n = 100_000usize;
        let base = 1.0e9_f64;
        let datos: Vec<f64> = (0..n).map(|i| base + (i % 101) as f64).collect();

        let esperado = referencia_two_pass(&datos);
        let estables: &[Implementacion] = &[
            ("secuencial", secuencial::desviacion_estandar),
            ("simd_double_pass_2", simd_double_pass_2::desviacion_estandar),
            ("simd_double_pass_4", simd_double_pass_4::desviacion_estandar),
            ("simd_double_pass_8", simd_double_pass_8::desviacion_estandar),
            (
                "simd_double_pass_8_par",
                simd_double_pass_8_par::desviacion_estandar_8_par,
            ),
            ("simd_desplazado", simd_desplazado::desviacion_estandar),
            (
                "simd_desplazado_par",
                simd_desplazado::desviacion_estandar_par,
            ),
            (
                "simd_dos_pasadas_bloque",
                simd_dos_pasadas_bloque::desviacion_estandar,
            ),
            (
                "simd_dos_pasadas_bloque_par",
                simd_dos_pasadas_bloque::desviacion_estandar_par,
            ),
        ];
        for (nombre, func) in estables {
            assert_cercano(
                func(&datos),
                esperado,
                1e-6,
                &format!("cancelación estable / {nombre}"),
            );
        }

        for (nombre, func) in IMPLEMENTACIONES {
            let obtenido = func(&datos);
            assert!(
                obtenido.is_finite() && obtenido >= 0.0,
                "cancelación / {nombre}: esperado valor finito no negativo, obtenido {obtenido}"
            );
        }
    }

    /// La desviación estándar es invariante ante traslaciones.
    #[test]
    fn test_invariante_traslacion() {
        let datos = lcg_valores(50_000, 7);
        let desplazados: Vec<f64> = datos.iter().map(|v| v + 1_000.0).collect();
        for (nombre, func) in IMPLEMENTACIONES {
            assert_cercano(
                func(&desplazados),
                func(&datos),
                1e-6,
                &format!("traslación / {nombre}"),
            );
        }
    }

    /// La desviación estándar escala linealmente: `f(a·x) = |a|·f(x)`.
    #[test]
    fn test_invariante_escala() {
        let datos = lcg_valores(50_000, 11);
        let a = -3.5_f64;
        let escalados: Vec<f64> = datos.iter().map(|v| v * a).collect();
        for (nombre, func) in IMPLEMENTACIONES {
            assert_cercano(
                func(&escalados),
                a.abs() * func(&datos),
                1e-12,
                &format!("escala / {nombre}"),
            );
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(64))]

        /// Compara todas las implementaciones contra la referencia estable
        /// sobre vectores de datos acotados (evitando `NaN`/`∞`).
        #[test]
        fn test_propiedad_todas_implementaciones(
            datos in proptest::collection::vec(-1.0e6f64..1.0e6f64, 1..10_000)
        ) {
            comparar_todas(&datos, 1e-6, "proptest");
        }
    }
}
