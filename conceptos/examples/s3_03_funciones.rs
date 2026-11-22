//! 3.3 Funciones

// Los parámetros siempre llevan tipo.
// El tipo de retorno se indica con `->`.
fn area_rectangulo(base: f64, altura: f64) -> f64 {
    base * altura // última expresión sin `;`: es el valor retornado
}

fn saludar(nombre: &str) {
    println!("¡Bienvenido al laboratorio, {nombre}!");
}

fn es_par(n: i32) -> bool {
    if n % 2 == 0 {
        return true; // retorno anticipado explícito
    }
    false
}

fn main() {
    saludar("Carlos");
    let area = area_rectangulo(4.0, 2.5);
    println!("Área: {area}");
    println!("¿7 es par? {}", es_par(7));

    // Un bloque también es una expresión con valor.
    let doble = {
        let x = 21;
        x * 2
    };
    println!("Doble: {doble}");
}
