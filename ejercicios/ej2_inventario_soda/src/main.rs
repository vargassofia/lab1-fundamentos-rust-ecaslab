//! Ejercicio 2: Inventario de la soda (sección 4.2)
//!
//! Lea el enunciado en README.md y reemplace cada `todo!()` por su código.
//! Cuando termine, `cargo test -p ej2_inventario_soda` debe pasar completo.

// Evita advertencias mientras el ejercicio está incompleto.
// Borre esta línea cuando termine.
#![allow(dead_code, unused_variables)]

// Convierte "producto,cantidad" en una tupla, o explica el error.
// Use el operador ? al menos una vez.
fn parsear_linea(linea: &str) -> Result<(String, u32), String> {
    todo!("divida la línea, valide sus partes y convierta la cantidad")
}

fn main() {
    let entradas = [
        "empanadas, 12",
        "Café,30",
        "empanadas,8",
        "gallo pinto",
        "jugo,muchos",
        "café, 5",
    ];
    // ... su código aquí: acumule las cantidades en un HashMap<String, u32>,
    // guarde los errores en un Vec<String> e imprima ambos.
}

// Pruebas: ejecútelas con `cargo test -p ej2_inventario_soda`.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linea_valida_se_normaliza() {
        assert_eq!(parsear_linea(" Café , 30"), Ok((String::from("café"), 30)));
    }

    #[test]
    fn sin_coma_es_formato_invalido() {
        assert_eq!(
            parsear_linea("gallo pinto"),
            Err(String::from("formato inválido: 'gallo pinto'"))
        );
    }

    #[test]
    fn dos_comas_es_formato_invalido() {
        assert!(
            parsear_linea("a,1,2")
                .unwrap_err()
                .starts_with("formato inválido")
        );
    }

    #[test]
    fn producto_vacio() {
        assert_eq!(
            parsear_linea("  ,4"),
            Err(String::from("producto vacío: '  ,4'"))
        );
    }

    #[test]
    fn cantidad_no_numerica_o_negativa() {
        assert_eq!(
            parsear_linea("jugo,muchos"),
            Err(String::from("cantidad inválida: 'jugo,muchos'"))
        );
        assert!(
            parsear_linea("jugo,-3")
                .unwrap_err()
                .starts_with("cantidad inválida")
        );
    }
}
