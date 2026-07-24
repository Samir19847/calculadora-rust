# Calculadora Rust

Calculadora de consola escrita en Rust. Permite realizar las cuatro operaciones básicas mediante un menú interactivo, con validación de entrada y manejo de errores.

## Funcionalidades

- Suma, resta, multiplicación y división
- En la división, además del resultado muestra el residuo, y controla la división entre cero
- Validación de entradas: si el usuario ingresa algo que no es un número, el programa lo avisa y vuelve a pedir el dato en lugar de cerrarse

## Conceptos de Rust aplicados

- **Funciones puras** para cada operación (`sumar`, `restar`, `multiplicar`, `dividir`, `residuo`)
- **Manejo de errores con `Result`**: el `match` sobre `parse()` evita que el programa entre en pánico si la entrada no es válida
- **Control de flujo con `loop` y `continue`**: el menú se repite hasta que el usuario elige salir

## Cómo ejecutarlo

Necesitás tener [Rust y Cargo](https://www.rust-lang.org/tools/install) instalados.

```bash
git clone https://github.com/Samir19847/calculadora-rust.git
cd calculadora-rust
cargo run
```

## Estado del proyecto

Proyecto simple hecho como práctica de fundamentos de Rust: tipos, funciones y manejo de errores con `Result`.
