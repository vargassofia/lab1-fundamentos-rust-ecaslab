// Error común de la sección 3.13. Este archivo NO compila a propósito.
// Error esperado: E0277
// Compruébelo con: rustc --edition 2024 conceptos/errores-comunes/e3_13_interrogacion_en_main.rs

fn main() {
    let numero: i32 = "42".parse()?;
    println!("{numero}");
}
