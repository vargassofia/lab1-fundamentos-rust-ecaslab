// Error común de la sección 3.10. Este archivo NO compila a propósito.
// Error esperado: E0308
// Compruébelo con: rustc --edition 2024 conceptos/errores-comunes/e3_10_option_como_valor.rs

fn main() {
    let notas: Vec<u32> = vec![80, 95];
    let primera: u32 = notas.get(0);
    println!("{primera}");
}
