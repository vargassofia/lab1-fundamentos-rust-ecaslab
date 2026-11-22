// Error común de la sección 3.12. Este archivo NO compila a propósito.
// Error esperado: E0282
// Compruébelo con: rustc --edition 2024 conceptos/errores-comunes/e3_12_tipo_no_deducible.rs

fn main() {
    let lista = Vec::new();
    println!("Elementos: {}", lista.len());
}
