//! 3.11 Módulos y crates

// Un módulo agrupa código relacionado.
mod geometria {
    // Sin `pub`, todo es privado al módulo.
    pub fn area_circulo(radio: f64) -> f64 {
        PI_APROX * radio * radio
    }

    const PI_APROX: f64 = 3.14159;

    // Los módulos se pueden anidar.
    pub mod conversiones {
        pub fn cm_a_m(cm: f64) -> f64 {
            cm / 100.0
        }
    }
}

// `use` trae nombres al alcance actual.
use geometria::conversiones::cm_a_m;
// También funciona con la biblioteca estándar.
use std::cmp::max;

fn main() {
    println!("Área: {:.2}", geometria::area_circulo(2.0));
    println!("150 cm = {} m", cm_a_m(150.0));
    println!("Mayor: {}", max(17, 42));
}
