//! 3.10 Option y Result

// Option: el valor puede existir (Some) o no (None).
fn buscar_nota(notas: &[f64], indice: usize) -> Option<f64> {
    if indice < notas.len() {
        Some(notas[indice])
    } else {
        None
    }
}

// Result: la operación puede salir bien (Ok) o fallar (Err).
fn dividir(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("no se puede dividir entre cero"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    let notas = [85.0, 92.5, 78.0];

    match buscar_nota(&notas, 1) {
        Some(n) => println!("Nota encontrada: {n}"),
        None => println!("No existe esa nota"),
    }
    // `if let` sirve cuando solo interesa un caso.
    if let Some(n) = buscar_nota(&notas, 2) {
        println!("Tercera nota: {n}");
    }
    // unwrap_or da un valor por defecto si es None.
    println!("Con defecto: {}", buscar_nota(&notas, 5).unwrap_or(0.0));

    match dividir(10.0, 4.0) {
        Ok(r) => println!("10 / 4 = {r}"),
        Err(e) => println!("Error: {e}"),
    }
    match dividir(1.0, 0.0) {
        Ok(r) => println!("1 / 0 = {r}"),
        Err(e) => println!("Error: {e}"),
    }
}
