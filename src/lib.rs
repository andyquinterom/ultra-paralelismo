#![feature(portable_simd)]

pub mod i1;
pub mod i2;
pub mod i3;
pub mod i4;
pub mod i5;

#[cfg(test)]
mod tests {
    use super::*;

    const IMPLEMENTACIONES: &[(&str, fn(&[f64]) -> f64)] = &[
        ("i1_secuencial", i1::desviacion_estandar),
        ("i2_paralelo", i2::desviacion_estandar),
        ("i3_paralelo_min_len", i3::desviacion_estandar),
        ("i4_paralelo_simd", i4::desviacion_estandar),
        ("i5_paralelo_simd_unsafe", i5::desviacion_estandar),
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
