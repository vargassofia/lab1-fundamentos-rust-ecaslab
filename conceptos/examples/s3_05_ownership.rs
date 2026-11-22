//! 3.5 Ownership (propiedad)

fn imprimir_y_soltar(texto: String) {
    println!("Recibí: {texto}");
} // aquí `texto` sale de alcance y su memoria se libera

fn main() {
    // s1 es dueño del String guardado en el heap.
    let s1 = String::from("Rust");

    // Movimiento: la propiedad pasa a s2; s1 deja de ser válido.
    let s2 = s1;
    println!("s2 = {s2}");

    // clone() crea una copia profunda: ambos son válidos.
    let s3 = s2.clone();
    println!("s2 = {s2}, s3 = {s3}");

    // Pasar un String a una función también lo mueve.
    imprimir_y_soltar(s3);

    // Los tipos simples (i32, bool, char...) implementan Copy:
    // se copian en lugar de moverse.
    let a = 5;
    let b = a;
    println!("a = {a}, b = {b}");
}
