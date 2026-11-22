// Error común de la sección 3.6. Este archivo NO compila a propósito.
// Error esperado: E0502
// Compruébelo con: rustc --edition 2024 conceptos/errores-comunes/e3_06_prestamo_mutable.rs

fn main() {
    let mut lista = vec![1, 2, 3];
    let primero = &lista[0];
    lista.push(4);
    println!("Primero: {primero}");
}
