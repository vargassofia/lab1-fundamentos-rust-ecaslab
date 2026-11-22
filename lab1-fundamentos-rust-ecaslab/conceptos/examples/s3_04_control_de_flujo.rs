//! 3.4 Control de flujo

fn main() {
    let nota = 82;

    // `if` es una expresión: puede asignar un valor.
    let resultado = if nota >= 70 { "aprobado" } else { "reprobado" };
    println!("Nota {nota}: {resultado}");

    // `loop` repite hasta un `break`, que puede devolver un valor.
    let mut intentos = 0;
    let total = loop {
        intentos += 1;
        if intentos == 3 {
            break intentos * 10;
        }
    };
    println!("Total tras {intentos} intentos: {total}");

    // `while` repite mientras la condición sea verdadera.
    let mut cuenta = 3;
    while cuenta > 0 {
        println!("{cuenta}...");
        cuenta -= 1;
    }
    println!("¡Despegue!");

    // `for` recorre rangos y colecciones.
    for i in 1..=3 {
        println!("Vuelta {i}");
    }
}
