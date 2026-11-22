//! 3.1 Variables y mutabilidad

fn main() {
    // Por defecto, las variables son inmutables.
    let nombre = "Ana";
    println!("Hola, {nombre}");

    // Con `mut` se permite cambiar el valor.
    let mut puntos = 10;
    println!("Puntos iniciales: {puntos}");
    puntos += 5;
    println!("Puntos finales: {puntos}");

    // Las constantes llevan tipo explícito y nombre en MAYÚSCULAS.
    const NOTA_MINIMA: f64 = 70.0;
    println!("Nota mínima para aprobar: {NOTA_MINIMA}");

    // Shadowing: una nueva variable con el mismo nombre
    // (incluso puede cambiar de tipo).
    let espacios = "   ";
    let espacios = espacios.len();
    println!("Cantidad de espacios: {espacios}");
}
