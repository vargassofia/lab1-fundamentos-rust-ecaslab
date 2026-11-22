//! 3.2 Tipos básicos

fn main() {
    // Enteros con y sin signo.
    let temperatura: i32 = -5;
    let estudiantes: u8 = 35;

    // Punto flotante (f64 es el tipo por defecto).
    let promedio: f64 = 87.5;

    // Booleanos y caracteres (char usa comillas simples).
    let aprobado: bool = promedio >= 70.0;
    let letra: char = 'ñ';

    println!("Temperatura: {temperatura} °C");
    println!("Estudiantes: {estudiantes}");
    println!("Promedio: {promedio}, ¿aprobado? {aprobado}");
    println!("Letra: {letra}");

    // Las conversiones entre tipos numéricos son explícitas.
    let total = estudiantes as f64 * promedio;
    println!("Total de puntos: {total}");

    // Tuplas y arreglos de tamaño fijo.
    let punto: (i32, i32) = (3, -2);
    let notas: [u32; 3] = [90, 75, 88];
    println!("Punto: ({}, {})", punto.0, punto.1);
    println!("Primera nota: {}, cantidad: {}", notas[0], notas.len());
}
