# Proyecto: calculadora de consola

**Sección 5 del Laboratorio 1.** Una calculadora interactiva que integra enums, `match`, `Result`, el operador `?`, slices, módulos, `Vec` y `Option`.

## Requisitos

- Mostrar un mensaje de bienvenida y pedir operaciones en un ciclo, con el indicador `> `.
- Aceptar operaciones con el formato `número operador número`, separados por espacios (por ejemplo, `7.5 * 4`). Los números pueden tener decimales.
- Soportar los operadores `+`, `-`, `*` y `/`, representados con un `enum Operacion`.
- Las funciones que pueden fallar devuelven `Result<_, String>`, y la función que interpreta una línea usa el operador `?`.
- Manejar sin pánico (sin `unwrap` sobre datos del usuario) estos errores: número inválido, operador desconocido, formato incorrecto y división entre cero. Cada error se informa con un mensaje que empieza con `Error:` y el programa continúa.
- Terminar cuando el usuario escriba `salir`.
- Ubicar la lógica de cálculo en un módulo `calculadora`, separada de `main`.
- Compilar sin advertencias con `cargo build`.

### Retos extra opcionales

1. **Historial:** el comando `historial` muestra todas las operaciones válidas, numeradas (`Vec<String>`).
2. **Memoria con `ans`:** `ans` reemplaza a cualquiera de los dos números con el último resultado (`Option<f64>`). Usarlo antes de la primera operación muestra un error.

## Estructura

```
proyecto-calculadora/
├── src/
│   ├── main.rs          ← ciclo de lectura e impresión
│   └── calculadora.rs   ← enum Operacion, aplicar y evaluar
└── tests/
    └── sesion.rs        ← ejecuta una sesión completa y revisa la salida
```

## Cómo ejecutarla

```bash
cargo run -p calculadora
cargo test -p calculadora
```

Ejemplo de sesión:

```
Calculadora de consola - ECAS Lab
Escriba una operación (por ejemplo, 7.5 * 4), 'historial' o 'salir'.
> 7.5 * 4
= 30
> 10 / 0
Error: no se puede dividir entre cero
> ans - 0.5
= 29.5
> historial
1. 7.5 * 4 = 30
2. ans - 0.5 = 29.5
> salir
¡Hasta luego!
```

## Ramas

- `main`: punto de partida con el `enum Operacion` de la sección 5.3 y `todo!()` donde va su código.
- `soluciones`: implementación de referencia completa, con los dos retos.

## Créditos

La primera versión de esta calculadora partió del tutorial de Shivam Ojha, [*100 Days of Rust — Day 3: Building a simple calculator CLI in Rust*](https://medium.com/@shivamojha2419/100-days-of-rust-day-3-building-a-simple-calculator-cli-in-rust-dd974265313b). El historial de commits de ese repositorio original ([vargassofia/Calculator](https://github.com/vargassofia/Calculator)) se conserva aquí. La versión actual se reescribió para cumplir los requisitos del laboratorio.
