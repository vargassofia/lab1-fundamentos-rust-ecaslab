//! Lógica de la calculadora, separada de la interacción con el usuario.
//!
//! Reemplace cada `todo!()` por su código. Cuando termine,
//! `cargo test -p calculadora` debe pasar completo.

// Punto de partida de la sección 5.3. `pub` es necesario porque el enum
// vive dentro del módulo `calculadora` y se usa desde fuera.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operacion {
    Suma,
    Resta,
    Multiplicacion,
    Division,
}

impl Operacion {
    pub fn desde_simbolo(simbolo: &str) -> Result<Operacion, String> {
        match simbolo {
            "+" => Ok(Operacion::Suma),
            "-" => Ok(Operacion::Resta),
            "*" => Ok(Operacion::Multiplicacion),
            "/" => Ok(Operacion::Division),
            otro => Err(format!("operador desconocido: '{otro}'")),
        }
    }
}

/// Aplica la operación con un `match`. Recuerde: dividir un f64 entre 0.0
/// no falla en Rust (da `inf`), así que debe revisar el divisor usted.
pub fn aplicar(a: f64, op: Operacion, b: f64) -> Result<f64, String> {
    todo!("haga un match sobre op")
}

/// Divide la línea, convierte las partes y llama a `aplicar` usando `?`.
pub fn evaluar(linea: &str) -> Result<f64, String> {
    todo!("interprete «número operador número»")
}

// Pruebas: ejecútelas con `cargo test -p calculadora`.
// Los mensajes exactos pueden variar; ajuste las pruebas si cambia los suyos.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn las_cuatro_operaciones() {
        assert_eq!(evaluar("2 + 3"), Ok(5.0));
        assert_eq!(evaluar("10 - 4.5"), Ok(5.5));
        assert_eq!(evaluar("7.5 * 4"), Ok(30.0));
        assert_eq!(evaluar("9 / 2"), Ok(4.5));
    }

    #[test]
    fn division_entre_cero_es_error() {
        assert!(evaluar("5 / 0").is_err());
    }

    #[test]
    fn numero_invalido_es_error() {
        assert!(evaluar("dos + 2").is_err());
    }

    #[test]
    fn operador_desconocido_es_error() {
        assert!(evaluar("2 ^ 3").is_err());
    }

    #[test]
    fn formato_incorrecto_es_error() {
        assert!(evaluar("2 +").is_err());
        assert!(evaluar("1 + 2 + 3").is_err());
    }
}
