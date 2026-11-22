//! 3.6 Borrowing y referencias

// Recibe una referencia: toma prestado, no se adueña.
fn longitud(texto: &String) -> usize {
    texto.len()
}

// Recibe una referencia mutable: puede modificar el original.
fn agregar_signo(texto: &mut String) {
    texto.push('!');
}

fn main() {
    let mut saludo = String::from("Hola");

    let n = longitud(&saludo); // préstamo inmutable
    println!("'{saludo}' tiene {n} bytes");

    agregar_signo(&mut saludo); // préstamo mutable
    println!("Ahora: {saludo}");

    // Varias referencias inmutables al mismo tiempo: permitido.
    let r1 = &saludo;
    let r2 = &saludo;
    println!("r1 = {r1}, r2 = {r2}");
    // r1 y r2 ya no se usan; se puede prestar como mutable.
    let r3 = &mut saludo;
    r3.push_str(" TEC");
    println!("r3 = {r3}");
}
