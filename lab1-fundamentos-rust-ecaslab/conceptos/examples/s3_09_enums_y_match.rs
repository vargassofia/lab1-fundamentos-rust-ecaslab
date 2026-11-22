//! 3.9 Enums y match

// Cada variante puede guardar datos distintos.
enum Figura {
    Circulo(f64),         // radio
    Rectangulo(f64, f64), // base, altura
    Triangulo { base: f64, altura: f64 },
}

fn area(figura: &Figura) -> f64 {
    // `match` obliga a cubrir todas las variantes.
    match figura {
        Figura::Circulo(r) => 3.14159 * r * r,
        Figura::Rectangulo(b, h) => b * h,
        Figura::Triangulo { base, altura } => base * altura / 2.0,
    }
}

fn describir(n: i32) -> &'static str {
    match n {
        0 => "cero",
        1..=9 => "un dígito",
        x if x < 0 => "negativo",
        _ => "dos o más dígitos", // `_` captura el resto
    }
}

fn main() {
    let figuras = [
        Figura::Circulo(1.0),
        Figura::Rectangulo(3.0, 4.0),
        Figura::Triangulo {
            base: 6.0,
            altura: 2.0,
        },
    ];
    for f in &figuras {
        println!("Área: {:.2}", area(f));
    }
    println!("7 es {}; -3 es {}", describir(7), describir(-3));
}
