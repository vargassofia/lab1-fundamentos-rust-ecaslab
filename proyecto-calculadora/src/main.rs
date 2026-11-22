//! Proyecto del Laboratorio 1: calculadora de consola (sección 5).
//! Los requisitos están en README.md.

// Evita advertencias mientras el proyecto está incompleto.
// Borre esta línea cuando termine.
#![allow(dead_code, unused_imports, unused_variables)]

mod calculadora;

use std::io::{self, Write};

fn main() {
    // ... su código aquí: mensaje de bienvenida.

    loop {
        // Fragmento de la sección 5.3 para leer una línea del teclado.
        print!("> ");
        io::stdout().flush().expect("no se pudo escribir");

        let mut linea = String::new();
        let leidos = io::stdin()
            .read_line(&mut linea)
            .expect("no se pudo leer la entrada");
        if leidos == 0 {
            break; // fin de la entrada (Ctrl+D o Ctrl+Z)
        }
        let linea = linea.trim();

        // ... su código aquí: termine con "salir"; si no, llame a
        // calculadora::evaluar y muestre el resultado o "Error: ...".
        todo!("procese la línea")
    }
}
