//! Lógica de la calculadora, separada de la interacción con el usuario.

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

/// Aplica la operación. Dividir un f64 entre 0.0 no falla en Rust (da `inf`),
/// por eso el divisor se revisa a mano.
pub fn aplicar(a: f64, op: Operacion, b: f64) -> Result<f64, String> {
    match op {
        Operacion::Suma => Ok(a + b),
        Operacion::Resta => Ok(a - b),
        Operacion::Multiplicacion => Ok(a * b),
        Operacion::Division if b == 0.0 => Err(String::from("no se puede dividir entre cero")),
        Operacion::Division => Ok(a / b),
    }
}

/// Convierte un número escrito por el usuario, o la palabra `ans` (reto 2).
fn parsear_numero(texto: &str, ans: Option<f64>) -> Result<f64, String> {
    if texto == "ans" {
        return ans.ok_or_else(|| String::from("todavía no hay un resultado guardado en ans"));
    }
    let numero: f64 = texto
        .parse()
        .map_err(|_| format!("número inválido: '{texto}'"))?;
    // "inf" y "NaN" son f64 válidos para parse, pero no para una calculadora.
    if !numero.is_finite() {
        return Err(format!("número inválido: '{texto}'"));
    }
    Ok(numero)
}

/// Interpreta una línea con el formato `número operador número`.
///
/// El parámetro `ans` corresponde al reto 2: sin el reto, la firma
/// sugerida en la sección 5.3 es `evaluar(linea: &str)`.
pub fn evaluar(linea: &str, ans: Option<f64>) -> Result<f64, String> {
    let partes: Vec<&str> = linea.split_whitespace().collect();
    if partes.len() != 3 {
        return Err(String::from(
            "formato incorrecto; use número operador número (por ejemplo, 7.5 * 4)",
        ));
    }
    let a = parsear_numero(partes[0], ans)?;
    let op = Operacion::desde_simbolo(partes[1])?;
    let b = parsear_numero(partes[2], ans)?;
    aplicar(a, op, b)
}

// Pruebas: ejecútelas con `cargo test -p calculadora`.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn las_cuatro_operaciones() {
        assert_eq!(evaluar("2 + 3", None), Ok(5.0));
        assert_eq!(evaluar("10 - 4.5", None), Ok(5.5));
        assert_eq!(evaluar("7.5 * 4", None), Ok(30.0));
        assert_eq!(evaluar("9 / 2", None), Ok(4.5));
    }

    #[test]
    fn numeros_negativos() {
        assert_eq!(evaluar("-3 * -2", None), Ok(6.0));
    }

    #[test]
    fn division_entre_cero() {
        assert_eq!(
            evaluar("5 / 0", None),
            Err(String::from("no se puede dividir entre cero"))
        );
    }

    #[test]
    fn numero_invalido() {
        assert_eq!(
            evaluar("dos + 2", None),
            Err(String::from("número inválido: 'dos'"))
        );
        assert!(evaluar("inf + 1", None).is_err());
    }

    #[test]
    fn operador_desconocido() {
        assert_eq!(
            evaluar("2 ^ 3", None),
            Err(String::from("operador desconocido: '^'"))
        );
    }

    #[test]
    fn formato_incorrecto() {
        assert!(
            evaluar("2 +", None)
                .unwrap_err()
                .starts_with("formato incorrecto")
        );
        assert!(
            evaluar("1 + 2 + 3", None)
                .unwrap_err()
                .starts_with("formato incorrecto")
        );
        assert!(
            evaluar("2+3", None)
                .unwrap_err()
                .starts_with("formato incorrecto")
        );
    }

    #[test]
    fn ans_usa_el_ultimo_resultado() {
        assert_eq!(evaluar("ans * 2", Some(15.0)), Ok(30.0));
        assert_eq!(evaluar("ans - ans", Some(4.0)), Ok(0.0));
    }

    #[test]
    fn ans_sin_resultado_previo_es_error() {
        assert!(evaluar("ans + 1", None).is_err());
    }
}
