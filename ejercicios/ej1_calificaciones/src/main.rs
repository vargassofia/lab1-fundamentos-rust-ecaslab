//! Ejercicio 1: Registro de calificaciones (sección 4.1)

struct Estudiante {
    nombre: String,
    notas: Vec<f64>,
}

impl Estudiante {
    fn new(nombre: &str, notas: Vec<f64>) -> Estudiante {
        Estudiante {
            nombre: String::from(nombre),
            notas,
        }
    }

    // None si el estudiante no tiene notas (evita dividir entre cero).
    fn promedio(&self) -> Option<f64> {
        if self.notas.is_empty() {
            return None;
        }
        let suma: f64 = self.notas.iter().sum();
        Some(suma / self.notas.len() as f64)
    }
}

// Devuelve una referencia al estudiante con mayor promedio.
fn mejor_estudiante(grupo: &[Estudiante]) -> Option<&Estudiante> {
    let mut mejor: Option<&Estudiante> = None;
    let mut mejor_promedio = 0.0;
    for est in grupo {
        if let Some(p) = est.promedio() {
            if mejor.is_none() || p > mejor_promedio {
                mejor = Some(est);
                mejor_promedio = p;
            }
        }
    }
    mejor
}

fn main() {
    let grupo = vec![
        Estudiante::new("Ana", vec![90.0, 85.0, 100.0]),
        Estudiante::new("Luis", vec![70.0, 65.0]),
        Estudiante::new("Valeria", vec![]),
        Estudiante::new("Diego", vec![95.0, 98.0]),
    ];

    for est in &grupo {
        match est.promedio() {
            Some(p) => println!("{}: {:.2}", est.nombre, p),
            None => println!("{}: sin notas", est.nombre),
        }
    }

    match mejor_estudiante(&grupo) {
        Some(est) => println!("Mejor promedio: {}", est.nombre),
        None => println!("No hay estudiantes con notas"),
    }
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
