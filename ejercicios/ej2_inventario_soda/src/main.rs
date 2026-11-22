//! Ejercicio 2: Inventario de la soda (sección 4.2)

use std::collections::HashMap;

// Convierte "producto,cantidad" en una tupla, o explica el error.
fn parsear_linea(linea: &str) -> Result<(String, u32), String> {
    let partes: Vec<&str> = linea.split(',').collect();
    if partes.len() != 2 {
        return Err(format!("formato inválido: '{linea}'"));
    }
    let producto = partes[0].trim().to_lowercase();
    if producto.is_empty() {
        return Err(format!("producto vacío: '{linea}'"));
    }
    let cantidad: u32 = partes[1]
        .trim()
        .parse()
        .map_err(|_| format!("cantidad inválida: '{linea}'"))?;
    Ok((producto, cantidad))
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

    let mut inventario: HashMap<String, u32> = HashMap::new();
    let mut errores: Vec<String> = Vec::new();

    for linea in entradas {
        match parsear_linea(linea) {
            Ok((producto, cantidad)) => {
                *inventario.entry(producto).or_insert(0) += cantidad;
            }
            Err(e) => errores.push(e),
        }
    }

    // HashMap no garantiza orden: se ordenan las claves para imprimir.
    let mut productos: Vec<&String> = inventario.keys().collect();
    productos.sort();
    println!("Inventario:");
    for p in productos {
        println!("  {p}: {}", inventario[p]);
    }
    println!("Errores ({}):", errores.len());
    for e in &errores {
        println!("  {e}");
    }
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
