//! 3.12 Colecciones: Vec, String y HashMap

use std::collections::HashMap;

fn main() {
    // Vec<T>: lista que crece dinámicamente.
    let mut notas: Vec<u32> = Vec::new();
    notas.push(85);
    notas.push(92);
    notas.push(78);
    println!("Notas: {notas:?} (cantidad: {})", notas.len());
    // get devuelve Option: no provoca pánico si el índice no existe.
    println!("Índice 1: {:?}, índice 9: {:?}", notas.get(1), notas.get(9));

    // String: texto UTF-8 que se puede modificar.
    let mut curso = String::from("Lenguajes");
    curso.push_str(" de Programación");
    println!("{curso} ({} caracteres)", curso.chars().count());

    // HashMap<K, V>: asocia claves con valores.
    let mut inventario: HashMap<String, u32> = HashMap::new();
    inventario.insert(String::from("lapiceros"), 12);
    inventario.insert(String::from("cuadernos"), 5);
    // entry().or_insert() inserta si la clave no existe.
    *inventario.entry(String::from("lapiceros")).or_insert(0) += 3;

    match inventario.get("lapiceros") {
        Some(c) => println!("Lapiceros: {c}"),
        None => println!("No hay lapiceros"),
    }
    // Contar palabras con un HashMap.
    let texto = "rust es rápido y rust es seguro";
    let mut conteo = HashMap::new();
    for palabra in texto.split_whitespace() {
        *conteo.entry(palabra).or_insert(0) += 1;
    }
    println!("'rust' aparece {} veces", conteo["rust"]);
}
