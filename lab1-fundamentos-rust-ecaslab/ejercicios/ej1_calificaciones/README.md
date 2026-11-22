# Ejercicio 1: Registro de calificaciones

**Sección 4.1 del Laboratorio 1** · Temas: structs, `Vec`, `Option`, `match`, slices y referencias · Dificultad: fácil

## Enunciado

Escriba un programa que registre las notas de un grupo de estudiantes y determine quién tiene el mejor promedio. Debe:

- Definir un struct `Estudiante` con los campos `nombre: String` y `notas: Vec<f64>`.
- Implementar una función asociada `new(nombre: &str, notas: Vec<f64>) -> Estudiante`.
- Implementar el método `promedio(&self) -> Option<f64>`, que devuelva `None` si el estudiante no tiene notas.
- Implementar la función `mejor_estudiante(grupo: &[Estudiante]) -> Option<&Estudiante>`, que devuelva el estudiante con mayor promedio, ignorando a quienes no tienen notas. Si hay empate, debe quedar el primero de la lista.
- En `main`, imprimir el promedio de cada estudiante con dos decimales (o «sin notas») y luego el nombre del mejor.

La plantilla `src/main.rs` ya trae los datos de entrada. La salida esperada está en [`salida_esperada.txt`](salida_esperada.txt).

## Pistas

- Para sumar un vector de `f64`, puede usar `let suma: f64 = self.notas.iter().sum();`. La anotación de tipo es necesaria.
- `self.notas.len()` es un `usize`: conviértalo con `as f64` antes de dividir.
- `mejor_estudiante` devuelve una referencia porque no debe adueñarse de los estudiantes del grupo.
- Para imprimir con dos decimales, use `{:.2}`.

## Cómo comprobar su solución

```bash
cargo run -p ej1_calificaciones    # ejecuta el programa
cargo test -p ej1_calificaciones   # pruebas unitarias + comparación con la salida esperada
```

Mientras haya un `todo!()` sin reemplazar, las pruebas fallan: es lo esperado. La solución de referencia está en la rama `soluciones` (`git switch soluciones`) y en el Anexo A.1 del documento.
