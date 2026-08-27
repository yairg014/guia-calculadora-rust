# 02. Calculadora y parser de expresiones

El archivo `src/calculadora.rs` evalúa números, operadores, paréntesis y funciones científicas. No usa `eval` ni ejecuta texto como código. En su lugar, un parser recorre la expresión carácter por carácter.

## Prioridad matemática

El parser está separado en cuatro niveles. Cada función llama al siguiente nivel, por lo que la prioridad se respeta de manera natural.

| Nivel | Función | Ejemplo |
|---:|---|---|
| 1 | `expresion` | Suma y resta: `2 + 3` |
| 2 | `termino` | Multiplicación y división: `3 * 4` |
| 3 | `potencia` | Potencias: `2 ^ 3` |
| 4 | `valor` | Números, paréntesis y funciones: `sqrt(81)` |

Por eso `2 + 3 * 4` devuelve `14`: primero se resuelve la multiplicación y luego la suma.

## Ejemplo de validación

Antes de crear el parser, la función `calcular` revisa la entrada.

```rust
if texto.is_empty() {
    return Err("Escribe una operación.".to_string());
}

if texto.len() > 120 || !texto.chars().all(caracter_permitido) {
    return Err("Solo se permiten números, operadores, paréntesis y funciones válidas.".to_string());
}
```

La función devuelve `Result<String, String>`. `Ok(...)` contiene un resultado listo para mostrar y `Err(...)` contiene un mensaje de error. El módulo de servidor transforma ambos casos en una página HTML que el usuario puede seguir usando.

## Funciones científicas disponibles

Las funciones se identifican con su nombre y después reciben el valor dentro de paréntesis. Los valores trigonométricos usan grados.

| Entrada | Resultado esperado |
|---|---|
| `sqrt(81)` | `9` |
| `sin(30)` | `0.5` |
| `cos(60)` | `0.5` |
| `tan(45)` | `1` |
| `ln(1)` | `0` |
| `log(100)` | `2` |
| `pi * 2` | Aproximación de `2π` |

## Cómo añadir otra función

En la función `funcion`, agrega un nuevo caso dentro de `match nombre.as_str()`. Por ejemplo, para una función de valor absoluto se usaría `"abs" => Ok(numero.abs())`. Después añade un botón en `static/index.html` con `agregar('abs(')` y una prueba en el bloque `#[cfg(test)]`.
