//! 3.7 Slices

// Devuelve un slice (&str) con la primera palabra.
fn primera_palabra(texto: &str) -> &str {
    for (i, &byte) in texto.as_bytes().iter().enumerate() {
        if byte == b' ' {
            return &texto[..i];
        }
    }
    &texto[..]
}

fn suma(numeros: &[i32]) -> i32 {
    let mut total = 0;
    for n in numeros {
        total += n;
    }
    total
}

fn main() {
    let frase = String::from("Instituto Tecnológico");
    let palabra = primera_palabra(&frase);
    println!("Primera palabra: {palabra}");

    // Rango de bytes [inicio..fin): el fin no se incluye.
    println!("Fragmento: {}", &frase[0..5]);

    // Slices de arreglos.
    let datos = [4, 8, 15, 16, 23, 42];
    let medio = &datos[1..4];
    println!("Medio: {medio:?}");
    println!("Suma del medio: {}", suma(medio));
    println!("Suma total: {}", suma(&datos));
}
