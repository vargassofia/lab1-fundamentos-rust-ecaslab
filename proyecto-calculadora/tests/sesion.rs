//! Ejecuta la calculadora con una sesión completa escrita en la entrada
//! estándar y revisa lo que imprime.

use std::io::Write;
use std::process::{Command, Stdio};

fn ejecutar(entrada: &str) -> String {
    let mut proceso = Command::new(env!("CARGO_BIN_EXE_calculadora"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("no se pudo ejecutar la calculadora");
    proceso
        .stdin
        .take()
        .unwrap()
        .write_all(entrada.as_bytes())
        .unwrap();
    let salida = proceso.wait_with_output().unwrap();
    assert!(
        salida.status.success(),
        "la calculadora terminó con un pánico"
    );
    String::from_utf8_lossy(&salida.stdout).into_owned()
}

#[test]
fn sesion_con_errores_no_detiene_el_programa() {
    let salida = ejecutar("7.5 * 4\n10 / 0\ndos + 2\n3 ^ 2\n5 +\n9 - 2.5\nsalir\n");
    assert!(salida.contains("30"));
    assert_eq!(salida.matches("Error:").count(), 4);
    assert!(salida.contains("6.5"));
}
