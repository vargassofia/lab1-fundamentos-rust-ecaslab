# Laboratorio 1: Fundamentos de Rust

**ECAS Lab** · Autora: Sofía Vargas González · Estudiante de Ingeniería en Computadores, TEC

Este repositorio acompaña la guía *Laboratorio 1: Fundamentos de Rust*. Contiene:

- Los ejemplos de la sección 3, listos para ejecutar.
- Los errores comunes del compilador, uno por archivo.
- Los dos ejercicios de la sección 4, con pruebas automáticas.
- El proyecto final de la sección 5: una calculadora de consola.

La meta es la misma que la de la guía: que usted pueda escribir programas pequeños sin pelear con el compilador y que entienda **por qué** rechaza su código cuando lo hace.

## Ramas

| Rama | Contenido |
|---|---|
| `main` | Punto de partida. Los ejercicios y el proyecto tienen `todo!()` donde va su código. |
| `soluciones` | Implementación de referencia completa, incluidos los retos extra de la calculadora. |

Intente resolver todo en `main` antes de mirar `soluciones`: pelear un poco con el compilador es parte del aprendizaje.

## Requisitos

- Rust instalado con `rustup` (sección 2.1 de la guía). Los ejemplos se probaron con **Rust 1.99.1** y la **edición 2024**.
- Git, para clonar el repositorio y cambiar de rama.

Compruebe la instalación:

```bash
rustc --version
cargo --version
```

## Inicio rápido

```bash
git clone https://github.com/vargassofia/lab1-fundamentos-rust-ecaslab.git
cd lab1-fundamentos-rust-ecaslab
cargo build --workspace                      # compila todo
cargo run -p conceptos --example s3_01_variables   # ejecuta el primer ejemplo
```

Todos los comandos de este manual se ejecutan desde la **raíz del repositorio**, donde está el `Cargo.toml` principal.

## Estructura de archivos

El repositorio es un *workspace* de Cargo: un solo `Cargo.toml` en la raíz agrupa varios paquetes, que comparten la carpeta `target/` y el archivo `Cargo.lock`.

```
lab1-fundamentos-rust-ecaslab/
├── Cargo.toml                  ← workspace: declara los paquetes del laboratorio
├── Cargo.lock                  ← versiones exactas de las dependencias
├── LICENSE
├── README.md                   ← este manual
│
├── conceptos/                  ← sección 3: fundamentos del lenguaje
│   ├── Cargo.toml              ← depende de rand (sección 3.11)
│   ├── examples/               ← un programa por tema (s3_01 … s3_13)
│   └── errores-comunes/        ← un error del compilador por tema (e3_01 … e3_13)
│       └── verificar.sh        ← comprueba que cada archivo falle con su error
│
├── ejercicios/                 ← sección 4
│   ├── ej1_calificaciones/
│   │   ├── README.md           ← enunciado y pistas
│   │   ├── salida_esperada.txt
│   │   ├── src/main.rs         ← su código va aquí
│   │   └── tests/salida.rs     ← compara la salida con salida_esperada.txt
│   └── ej2_inventario_soda/    ← misma estructura que el ejercicio 1
│
├── proyecto-calculadora/       ← sección 5
│   ├── README.md               ← requisitos, retos y ejemplo de sesión
│   ├── src/main.rs             ← ciclo de lectura e impresión
│   ├── src/calculadora.rs      ← enum Operacion, aplicar y evaluar
│   └── tests/sesion.rs         ← ejecuta una sesión completa y revisa la salida
│
└── target/                     ← lo genera Cargo al compilar (ignorado por git)
```

## Mapa: de la guía al archivo

Use esta tabla para encontrar el archivo de cada sección del documento.

### Sección 3: fundamentos del lenguaje

Cada tema tiene un **ejemplo** que compila y un **error común** que no compila a propósito.

| Sección | Tema | Ejemplo (`conceptos/examples/`) | Error común (`conceptos/errores-comunes/`) | Error esperado |
|---|---|---|---|---|
| 3.1 | Variables y mutabilidad | `s3_01_variables.rs` | `e3_01_asignar_inmutable.rs` | E0384 |
| 3.2 | Tipos básicos | `s3_02_tipos_basicos.rs` | `e3_02_literal_fuera_de_rango.rs` | literal out of range |
| 3.3 | Funciones | `s3_03_funciones.rs` | `e3_03_punto_y_coma_en_retorno.rs` | E0308 |
| 3.4 | Control de flujo | `s3_04_control_de_flujo.rs` | `e3_04_numero_como_condicion.rs` | E0308 |
| 3.5 | Ownership | `s3_05_ownership.rs` | `e3_05_usar_valor_movido.rs` | E0382 |
| 3.6 | Borrowing y referencias | `s3_06_borrowing.rs` | `e3_06_prestamo_mutable.rs` | E0502 |
| 3.7 | Slices | `s3_07_slices.rs` | `e3_07_string_en_vez_de_str.rs` | E0308 |
| 3.8 | Structs | `s3_08_structs.rs` | `e3_08_campo_de_struct_inmutable.rs` | E0594 |
| 3.9 | Enums y match | `s3_09_enums_y_match.rs` | `e3_09_match_no_exhaustivo.rs` | E0004 |
| 3.10 | Option y Result | `s3_10_option_y_result.rs` | `e3_10_option_como_valor.rs` | E0308 |
| 3.11 | Módulos y crates | `s3_11_modulos.rs`, `s3_11_crate_rand.rs` | `e3_11_funcion_privada.rs` | E0603 |
| 3.12 | Vec, String y HashMap | `s3_12_colecciones.rs` | `e3_12_tipo_no_deducible.rs` | E0282 |
| 3.13 | Operador `?` | `s3_13_operador_interrogacion.rs` | `e3_13_interrogacion_en_main.rs` | E0277 |

### Secciones 4 y 5: ejercicios y proyecto

| Sección | Qué es | Paquete (`-p`) | Archivos que usted edita |
|---|---|---|---|
| 4.1 | Ejercicio 1: Registro de calificaciones | `ej1_calificaciones` | `ejercicios/ej1_calificaciones/src/main.rs` |
| 4.2 | Ejercicio 2: Inventario de la soda | `ej2_inventario_soda` | `ejercicios/ej2_inventario_soda/src/main.rs` |
| 5 | Proyecto: calculadora de consola | `calculadora` | `proyecto-calculadora/src/main.rs` y `src/calculadora.rs` |
| Anexo A | Soluciones de los ejercicios | — | rama `soluciones` |

## Comandos

### Compilar

```bash
cargo check --workspace                 # revisa que todo compile, sin generar ejecutables (rápido)
cargo build --workspace                 # compila todo; los ejecutables quedan en target/debug/
cargo build --workspace --all-targets   # compila también ejemplos y pruebas
cargo clippy --workspace --all-targets  # sugerencias de estilo y errores frecuentes
```

### Ejecutar

```bash
cargo run -p conceptos --example s3_05_ownership   # un ejemplo de la sección 3
cargo run -p ej1_calificaciones                    # ejercicio 1
cargo run -p ej2_inventario_soda                   # ejercicio 2
cargo run -p calculadora                           # proyecto (escriba 'salir' para terminar)
```

### Probar

```bash
cargo test -p ej1_calificaciones                # pruebas de un solo paquete
cargo test -p ej1_calificaciones promedio       # solo las pruebas cuyo nombre contiene "promedio"
cargo test --workspace --no-fail-fast           # todas las pruebas de todos los paquetes
```

> **Importante:** sin `--no-fail-fast`, Cargo se detiene en el primer paquete cuyas pruebas fallan, así que puede parecer que «solo corrieron 5 pruebas». En la rama `main` es normal que las pruebas fallen con `not yet implemented` hasta que usted reemplace cada `todo!()`.

### Comprobar los errores comunes

Los archivos de `conceptos/errores-comunes/` **no compilan a propósito**, por eso no forman parte del workspace. Puede compilarlos uno por uno con `rustc` y leer el error, o comprobarlos todos con el script:

```bash
rustc --edition 2024 conceptos/errores-comunes/e3_05_usar_valor_movido.rs   # muestra el error E0382
rustc --explain E0382                                                       # explicación detallada del error
bash conceptos/errores-comunes/verificar.sh                                 # revisa los 13 archivos
```

El script compila en una carpeta temporal y no deja archivos sueltos.

## Cómo trabajar un ejercicio

1. Lea el enunciado en el `README.md` del ejercicio o en la sección 4 de la guía.
2. Abra `src/main.rs` y reemplace cada `todo!()` por su código.
3. Ejecute `cargo run -p <paquete>` y compare con `salida_esperada.txt`.
4. Ejecute `cargo test -p <paquete>` hasta que todas las pruebas pasen.
5. Cuando termine, borre la línea `#![allow(...)]` del inicio del archivo y confirme que `cargo build` no muestra advertencias.
6. Compare su solución con la rama `soluciones` o con el Anexo A de la guía:

   ```bash
   git diff origin/soluciones -- ejercicios/ej1_calificaciones/src/main.rs   # ver la diferencia sin cambiar de rama
   git switch soluciones                                                    # o cambiar de rama
   ```

   `git switch` exige que no tenga cambios sin guardar. Si los tiene, haga commit de su trabajo antes de cambiar de rama.

## Recursos

- [The Rust Programming Language](https://doc.rust-lang.org/book/) ([en español](https://book.rustlang-es.org/))
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://rustlings.rust-lang.org/)
- [Rust Playground](https://play.rust-lang.org/)

## Licencia

El código está bajo la licencia [MIT](LICENSE).
