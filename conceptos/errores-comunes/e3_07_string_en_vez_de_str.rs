// Error común de la sección 3.7. Este archivo NO compila a propósito.
// Error esperado: E0308
// Compruébelo con: rustc --edition 2024 conceptos/errores-comunes/e3_07_string_en_vez_de_str.rs

fn contar_letras(texto: &str) -> usize {
    texto.chars().count()
}

fn main() {
    let nombre = String::from("Sofía");
    println!("{}", contar_letras(nombre));
}
