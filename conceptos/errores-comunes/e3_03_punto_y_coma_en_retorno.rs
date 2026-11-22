// Error común de la sección 3.3. Este archivo NO compila a propósito.
// Error esperado: E0308
// Compruébelo con: rustc --edition 2024 conceptos/errores-comunes/e3_03_punto_y_coma_en_retorno.rs

fn cuadrado(x: i32) -> i32 {
    x * x;
}

fn main() {
    println!("{}", cuadrado(4));
}
