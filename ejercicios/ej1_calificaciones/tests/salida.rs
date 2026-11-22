//! Compara la salida del programa con la "Salida esperada" del documento.

use std::process::Command;

#[test]
fn la_salida_coincide_con_la_esperada() {
    let salida = Command::new(env!("CARGO_BIN_EXE_ej1_calificaciones"))
        .output()
        .expect("no se pudo ejecutar el programa");
    let obtenida = String::from_utf8_lossy(&salida.stdout);
    let esperada = include_str!("../salida_esperada.txt");
    assert_eq!(obtenida, esperada);
}
