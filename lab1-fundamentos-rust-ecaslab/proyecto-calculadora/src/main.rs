//! Proyecto del Laboratorio 1: calculadora de consola (sección 5).
//! Incluye los dos retos extra: `historial` y la memoria `ans`.

mod calculadora;

use calculadora::evaluar;
use std::io::{self, Write};

fn main() {
    println!("Calculadora de consola - ECAS Lab");
    println!("Escriba una operación (por ejemplo, 7.5 * 4), 'historial' o 'salir'.");

    let mut historial: Vec<String> = Vec::new(); // reto 1
    let mut ans: Option<f64> = None; // reto 2

    loop {
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

        match linea {
            "" => continue,
            "salir" => break,
            "historial" => {
                if historial.is_empty() {
                    println!("El historial está vacío.");
                }
                for (i, entrada) in historial.iter().enumerate() {
                    println!("{}. {entrada}", i + 1);
                }
            }
            _ => match evaluar(linea, ans) {
                Ok(resultado) => {
                    println!("= {resultado}");
                    historial.push(format!("{linea} = {resultado}"));
                    ans = Some(resultado);
                }
                Err(e) => println!("Error: {e}"),
            },
        }
    }
    println!("¡Hasta luego!");
}
