# 05. Pruebas y comprobaciones

Las pruebas están junto al código que validan. Rust solo compila esos bloques cuando se ejecuta `cargo test`; por eso no afectan la ejecución normal de la calculadora.

## Ejecutar pruebas

```bash
cargo test
```

El proyecto comprueba siete situaciones: lectura de formularios, escape de HTML, prioridad de operadores, funciones científicas, entradas incorrectas, ecuaciones lineales y casos especiales de ecuaciones.

## Patrón básico

```rust
#[test]
fn respeta_la_prioridad_de_operaciones() {
    assert_eq!(calcular("2 + 3 * 4"), Ok("14".to_string()));
}
```

La prueba entrega una entrada concreta y compara el valor recibido con el resultado esperado. Si más adelante cambias el parser y rompes la prioridad matemática, `cargo test` lo detectará.

## Lista de comprobación manual

| Prueba | Entrada | Resultado esperado |
|---|---|---|
| Inicio | Abrir `localhost:8000` | Campo y resultado en `0` |
| Operación | `2+2` | `4` |
| Científica | `sqrt(81)+sin(30)` | `9.5` |
| Error | `8/0` | Mensaje de división entre cero, sin cerrar la página |
| Ecuación | `2x-5=11` | `x = 8` |

La idea principal es que los errores sean respuestas normales de la aplicación. Una entrada inválida nunca debe detener el servidor ni cerrar la página.
