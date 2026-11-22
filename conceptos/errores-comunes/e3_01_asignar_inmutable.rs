// Error común de la sección 3.1. Este archivo NO compila a propósito.
// Error esperado: E0384
// Compruébelo con: rustc --edition 2024 conceptos/errores-comunes/e3_01_asignar_inmutable.rs

fn main() {
    let contador = 0;
    println!("Contador: {contador}");
    contador = 1;
    println!("Contador: {contador}");
}
