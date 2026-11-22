// Error común de la sección 3.9. Este archivo NO compila a propósito.
// Error esperado: E0004
// Compruébelo con: rustc --edition 2024 conceptos/errores-comunes/e3_09_match_no_exhaustivo.rs

enum Semaforo {
    Verde,
    Amarillo,
    Rojo,
}

fn accion(s: Semaforo) -> &'static str {
    match s {
        Semaforo::Verde => "avanzar",
        Semaforo::Rojo => "detenerse",
    }
}

fn main() {
    println!("{}", accion(Semaforo::Amarillo));
}
