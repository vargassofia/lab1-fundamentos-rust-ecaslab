//! Ejercicio 1: Registro de calificaciones (sección 4.1)
//!
//! Lea el enunciado en README.md y reemplace cada `todo!()` por su código.
//! Cuando termine, `cargo test -p ej1_calificaciones` debe pasar completo.

// Evita advertencias mientras el ejercicio está incompleto.
// Borre esta línea cuando termine.
#![allow(dead_code, unused_variables, clippy::useless_vec)]

struct Estudiante {
    nombre: String,
    notas: Vec<f64>,
}

impl Estudiante {
    fn new(nombre: &str, notas: Vec<f64>) -> Estudiante {
        todo!("cree un Estudiante con el nombre y las notas recibidos")
    }

    // Debe devolver None si el estudiante no tiene notas.
    fn promedio(&self) -> Option<f64> {
        todo!("calcule el promedio de self.notas")
    }
}

// Devuelve el estudiante con mayor promedio, ignorando a quienes no tienen
// notas. Si hay empate, debe quedar el primero de la lista.
fn mejor_estudiante(grupo: &[Estudiante]) -> Option<&Estudiante> {
    todo!("recorra el grupo y quédese con el mejor promedio")
}

fn main() {
    let grupo = vec![
        Estudiante::new("Ana", vec![90.0, 85.0, 100.0]),
        Estudiante::new("Luis", vec![70.0, 65.0]),
        Estudiante::new("Valeria", vec![]),
        Estudiante::new("Diego", vec![95.0, 98.0]),
    ];
    // ... su código aquí: imprima el promedio de cada estudiante con dos
    // decimales (o «sin notas») y luego el nombre del mejor.
}

// Pruebas: ejecútelas con `cargo test -p ej1_calificaciones`.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn promedio_con_notas() {
        let est = Estudiante::new("Ana", vec![90.0, 85.0, 100.0]);
        assert_eq!(est.promedio(), Some(275.0 / 3.0));
    }

    #[test]
    fn promedio_sin_notas_es_none() {
        let est = Estudiante::new("Valeria", vec![]);
        assert_eq!(est.promedio(), None);
    }

    #[test]
    fn mejor_estudiante_ignora_a_quien_no_tiene_notas() {
        let grupo = vec![
            Estudiante::new("Valeria", vec![]),
            Estudiante::new("Luis", vec![70.0, 65.0]),
        ];
        assert_eq!(
            mejor_estudiante(&grupo).map(|e| e.nombre.as_str()),
            Some("Luis")
        );
    }

    #[test]
    fn en_empate_queda_el_primero() {
        let grupo = vec![
            Estudiante::new("Ana", vec![80.0]),
            Estudiante::new("Diego", vec![80.0]),
        ];
        assert_eq!(
            mejor_estudiante(&grupo).map(|e| e.nombre.as_str()),
            Some("Ana")
        );
    }

    #[test]
    fn grupo_vacio_no_tiene_mejor() {
        assert!(mejor_estudiante(&[]).is_none());
    }
}
