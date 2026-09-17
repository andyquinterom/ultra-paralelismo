#![feature(portable_simd)]

pub mod secuencial;
pub mod secuencial_single_pass;
pub mod simd_single_pass;
pub mod simd_single_pass_par;
pub mod simd_double_pass_2;
pub mod simd_double_pass_4;
pub mod simd_double_pass_8;
pub mod simd_double_pass_8_par;

#[cfg(test)]
mod tests {
    use super::*;

    const IMPLEMENTACIONES: &[(&str, fn(&[f64]) -> f64)] = &[
        ("secuencial", secuencial::desviacion_estandar),
        ("secuencial_single_pass", secuencial_single_pass::desviacion_estandar),
        ("simd_single_pass", simd_single_pass::desviacion_estandar),
        ("simd_single_pass_par", simd_single_pass_par::desviacion_estandar),
        ("simd_double_pass_2", simd_double_pass_2::desviacion_estandar),
        ("simd_double_pass_4", simd_double_pass_4::desviacion_estandar),
        ("simd_double_pass_8", simd_double_pass_8::desviacion_estandar),
        ("simd_double_pass_8_par", simd_double_pass_8_par::desviacion_estandar_8_par),
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
}
