// Error común de la sección 3.5. Este archivo NO compila a propósito.
// Error esperado: E0382
// Compruébelo con: rustc --edition 2024 conceptos/errores-comunes/e3_05_usar_valor_movido.rs

fn main() {
    let s1 = String::from("Rust");
    let s2 = s1;
    println!("s1 = {s1}, s2 = {s2}");
}
