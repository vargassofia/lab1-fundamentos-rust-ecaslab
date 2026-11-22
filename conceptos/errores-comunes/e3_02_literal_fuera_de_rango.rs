// Error común de la sección 3.2. Este archivo NO compila a propósito.
// Error esperado: literal out of range for `u8`
// Compruébelo con: rustc --edition 2024 conceptos/errores-comunes/e3_02_literal_fuera_de_rango.rs

fn main() {
    let edad: u8 = 300;
    println!("Edad: {edad}");
}
