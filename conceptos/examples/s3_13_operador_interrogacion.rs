//! 3.13 Manejo de errores con el operador ?

use std::num::ParseIntError;

// `?` devuelve el error de inmediato si lo hay;
// si es Ok, extrae el valor y continúa.
fn sumar_textos(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let x: i32 = a.trim().parse()?;
    let y: i32 = b.trim().parse()?;
    Ok(x + y)
}

// Con un error propio de tipo String usando map_err.
fn edad_valida(texto: &str) -> Result<u8, String> {
    let edad: u8 = texto
        .parse()
        .map_err(|_| format!("'{texto}' no es una edad válida"))?;
    if edad < 15 {
        return Err(format!("{edad} es menor que la edad mínima"));
    }
    Ok(edad)
}

fn main() {
    println!("{:?}", sumar_textos("40", " 2 "));
    println!("{:?}", sumar_textos("40", "dos"));
    println!("{:?}", edad_valida("19"));
    println!("{:?}", edad_valida("12"));
    println!("{:?}", edad_valida("veinte"));
}
