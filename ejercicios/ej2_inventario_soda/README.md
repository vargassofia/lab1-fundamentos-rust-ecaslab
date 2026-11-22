# Ejercicio 2: Inventario de la soda

**Sección 4.2 del Laboratorio 1** · Temas: `String`, `HashMap`, `Vec`, `Result`, el operador `?` y `match` · Dificultad: intermedia

## Enunciado

La soda de la escuela registra los productos que recibe como líneas de texto con el formato `producto,cantidad`. Algunas líneas vienen con errores. Escriba un programa que:

- Implemente `parsear_linea(linea: &str) -> Result<(String, u32), String>`, que convierta una línea en una tupla `(producto, cantidad)`. El nombre del producto se guarda sin espacios sobrantes y en minúsculas, de modo que «Café» y «café» sean el mismo producto.
- Devuelva un error descriptivo si la línea no tiene exactamente una coma (`formato inválido: '...'`), si el producto está vacío (`producto vacío: '...'`) o si la cantidad no es un número entero positivo (`cantidad inválida: '...'`).
- Use el operador `?` al menos una vez dentro de `parsear_linea`.
- En `main`, acumule las cantidades en un `HashMap<String, u32>` (un mismo producto puede aparecer varias veces) y guarde los mensajes de error en un `Vec<String>`.
- Imprima el inventario ordenado alfabéticamente y luego la lista de errores.

La plantilla `src/main.rs` ya trae los datos de entrada. La salida esperada está en [`salida_esperada.txt`](salida_esperada.txt).

## Pistas

- `linea.split(',').collect::<Vec<&str>>()` le da las partes de la línea; revise que sean exactamente dos.
- `texto.trim().parse::<u32>()` devuelve un `Result<u32, ParseIntError>`. Para usar `?` en una función que retorna `Result<_, String>`, convierta el error con `.map_err(|_| format!(...))`.
- Para acumular, use `*inventario.entry(producto).or_insert(0) += cantidad;`.
- Para imprimir en orden, reúna las claves en un vector (`inventario.keys().collect()`), ordénelo con `sort()` y recórralo.

## Cómo comprobar su solución

```bash
cargo run -p ej2_inventario_soda
cargo test -p ej2_inventario_soda
```

La solución de referencia está en la rama `soluciones` y en el Anexo A.2 del documento.
