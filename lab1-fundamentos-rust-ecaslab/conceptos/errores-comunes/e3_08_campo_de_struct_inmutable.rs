// Error común de la sección 3.8. Este archivo NO compila a propósito.
// Error esperado: E0594
// Compruébelo con: rustc --edition 2024 conceptos/errores-comunes/e3_08_campo_de_struct_inmutable.rs

struct Punto {
    x: i32,
    y: i32,
}

fn main() {
    let p = Punto { x: 0, y: 0 };
    p.x = 5;
    println!("({}, {})", p.x, p.y);
}
