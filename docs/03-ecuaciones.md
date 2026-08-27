# 03. Ecuaciones lineales y sus validaciones

El archivo `src/ecuaciones.rs` resuelve ecuaciones lineales de una sola variable, `x`. En esta guía una ecuación lineal es una combinación de términos como `2x`, `x/2`, `+3` y `-5`; no se admiten productos como `x*x` ni potencias de `x`.

## Idea matemática

Cada lado se reduce a esta forma:

> `a·x + b`

La función `lado_lineal` devuelve una tupla `(a, b)`. Por ejemplo, `2x - 5` se convierte en `(2, -5)`. Si la ecuación es `a·x + b = c·x + d`, la solución se calcula con:

```text
x = (d - b) / (a - c)
```

## Casos que se controlan

| Entrada | Resultado |
|---|---|
| `2x - 5 = 11` | `x = 8` |
| `x/2 + 7 = 12` | `x = 10` |
| `3x + 2 = x + 10` | `x = 4` |
| `2x + 3 = 2x + 3` | Infinitas soluciones |
| `2x + 3 = 2x + 4` | Sin solución |
| `x*x = 9` | Formato no válido |
| `x/0 = 2` | Formato no válido |

## Validación clave

Antes de aceptar un término con variable, se cuenta cuántas veces aparece `x`.

```rust
if termino.matches('x').count() != 1 {
    return Err("No es lineal".to_string());
}
```

Esta pequeña validación evita tratar por error a `x*x` como si fuera una ecuación de primer grado.

## Cómo extender este módulo

Para admitir paréntesis o ecuaciones cuadráticas, conviene crear un parser más completo. No mezcles esa lógica con el servidor: crea otro módulo, por ejemplo `src/cuadraticas.rs`, y llámalo desde `servidor.rs`. Mantener cada tema en su archivo hace que el cambio sea más fácil de probar.
