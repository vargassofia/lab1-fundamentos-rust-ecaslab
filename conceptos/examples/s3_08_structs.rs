//! 3.8 Structs

// `derive(Debug)` permite imprimir con {:?}.
#[derive(Debug)]
struct Estudiante {
    nombre: String,
    carnet: u32,
    nota: f64,
}

impl Estudiante {
    // Función asociada que actúa como constructor.
    fn new(nombre: &str, carnet: u32) -> Estudiante {
        Estudiante {
            nombre: String::from(nombre),
            carnet,
            nota: 0.0,
        }
    }

    // Método que solo lee: recibe &self.
    fn aprobo(&self) -> bool {
        self.nota >= 70.0
    }

    // Método que modifica: recibe &mut self.
    fn asignar_nota(&mut self, nota: f64) {
        self.nota = nota;
    }
}

fn main() {
    let mut est = Estudiante::new("María", 2024123456);
    est.asignar_nota(91.5);
    println!("{} ({}) aprobó: {}", est.nombre, est.carnet, est.aprobo());
    println!("{est:?}");
}
