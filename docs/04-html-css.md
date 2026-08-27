# 04. HTML, CSS y envío de datos a Rust

Los archivos de `static/` no necesitan un framework. `index.html` define una sola calculadora y `estilos.css` se encarga del diseño. Esto permite concentrarse en cómo viaja el dato desde un formulario hasta Rust.

## El formulario

```html
<form action="/calcular" method="post">
  <input id="expresion" name="expresion" value="{{EXPRESION}}">
  <button type="submit" class="igual">=</button>
</form>
```

El atributo `action="/calcular"` coincide con la ruta que revisa `src/servidor.rs`. El atributo `name="expresion"` define el nombre que Rust busca en el cuerpo del formulario. Al pulsar el botón de tipo `submit`, el navegador envía la expresión mediante una solicitud `POST`.

## Botones sin recargar la página

Los botones numéricos son de tipo `button`; por eso solo agregan texto al campo y no envían el formulario.

```html
<button type="button" onclick="agregar('7')">7</button>
```

Solo el botón verde `=` es `submit`. La función `agregar` evita que una calculadora recién iniciada muestre `07` al pulsar el primer número.

## CSS en una sola pantalla

La clase `.calculadora` limita el ancho, centra el panel y aplica una sombra. La clase `.teclas` usa `grid` con cuatro columnas para ordenar los botones. Las clases `.operador`, `.limpiar` e `.igual` sirven para que el usuario distinga cada grupo sin añadir elementos extra.

## Conexión completa

| Capa | Elemento | Trabajo |
|---|---|---|
| HTML | `name="expresion"` | Nombra el dato enviado. |
| Navegador | `POST /calcular` | Envía el formulario. |
| Rust | `valor_formulario` | Lee y decodifica el dato. |
| Rust | `calcular` o `resolver` | Valida y produce el resultado. |
| HTML | `{{RESULTADO}}` | Muestra la respuesta del servidor. |
