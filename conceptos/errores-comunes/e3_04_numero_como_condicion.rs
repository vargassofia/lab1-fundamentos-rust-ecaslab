// Error común de la sección 3.4. Este archivo NO compila a propósito.
// Error esperado: E0308
// Compruébelo con: rustc --edition 2024 conceptos/errores-comunes/e3_04_numero_como_condicion.rs

fn main() {
    let cantidad = 3;
    if cantidad {
        println!("Hay elementos");
    }
}
