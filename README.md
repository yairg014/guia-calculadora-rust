# Guía: Calculadora Científica con Rust

Este repositorio es una guía para comenzar un proyecto web pequeño en **Rust**. La aplicación sirve una página HTML, recibe un formulario, valida lo escrito y muestra el resultado sin cerrar el servidor. El código está separado por responsabilidad: servidor, cálculo, ecuaciones e interfaz.

## Qué aprenderás

| Parte | Archivo | Idea principal |
|---|---|---|
| Inicio | `src/main.rs` | Declarar módulos y arrancar el programa. |
| Servidor | `src/servidor.rs` | Recibir rutas HTTP, formularios y responder HTML. |
| Calculadora | `src/calculadora.rs` | Interpretar operaciones respetando prioridad matemática. |
| Ecuaciones | `src/ecuaciones.rs` | Resolver ecuaciones lineales de una variable `x`. |
| Interfaz | `static/index.html` | Crear el formulario y los botones. |
| Estilo | `static/estilos.css` | Dar formato a una sola pantalla de calculadora. |

## Estructura

```text
guia-calculadora-rust/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── servidor.rs
│   ├── calculadora.rs
│   └── ecuaciones.rs
├── static/
│   ├── index.html
│   └── estilos.css
└── docs/
    ├── 01-servidor-http.md
    ├── 02-calculadora.md
    ├── 03-ecuaciones.md
    ├── 04-html-css.md
    └── 05-pruebas.md
```

## Ejecutar el proyecto

Instala Rust y, desde la carpeta del repositorio, ejecuta lo siguiente.

```bash
cargo run
```

Después abre [http://localhost:8000](http://localhost:8000). La primera ejecución descarga la única dependencia del proyecto: `tiny_http`.

## Ruta sugerida

Empieza por `src/main.rs` y continúa con la guía de servidor. Después revisa la calculadora, porque el parser es el encargado de evaluar operaciones como `2 + 3 * 4`. Por último, estudia el módulo de ecuaciones y la conexión entre el formulario HTML y el servidor.

> El objetivo no es memorizar cada línea, sino identificar qué responsabilidad tiene cada archivo. Esa separación se puede reutilizar en formularios, conversores, sistemas de registro y proyectos web pequeños.
