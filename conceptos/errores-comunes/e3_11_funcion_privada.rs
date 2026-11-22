// Error común de la sección 3.11. Este archivo NO compila a propósito.
// Error esperado: E0603
// Compruébelo con: rustc --edition 2024 conceptos/errores-comunes/e3_11_funcion_privada.rs

mod banco {
    fn saldo() -> u32 {
        1000
    }
}

fn main() {
    println!("Saldo: {}", banco::saldo());
}
