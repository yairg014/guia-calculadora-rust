# 01. Servidor HTTP y validación de solicitudes

El archivo `src/servidor.rs` concentra la comunicación entre el navegador y Rust. Usa `tiny_http` para evitar leer manualmente un socket: la librería espera la solicitud completa y responde con encabezados HTTP correctos. Esto evita errores de conexión al enviar formularios.

## Flujo de una solicitud

| Paso | Qué sucede | Dónde verlo |
|---:|---|---|
| 1 | Rust escucha el puerto `8000`. | `Server::http("0.0.0.0:8000")` |
| 2 | El navegador pide una ruta o envía un formulario. | `incoming_requests()` |
| 3 | El programa reconoce método y ruta. | `Method::Get` o `Method::Post` |
| 4 | Si llega un formulario, se lee el cuerpo completo. | `as_reader().read_to_string(...)` |
| 5 | Se calcula o se resuelve la ecuación. | Módulos `calculadora` y `ecuaciones` |
| 6 | Rust devuelve HTML con el resultado. | `solicitud.respond(...)` |

## Conexión principal

```rust
let servidor = Server::http("0.0.0.0:8000")?;

for mut solicitud in servidor.incoming_requests() {
    if solicitud.method() == &Method::Post && solicitud.url() == "/calcular" {
        let mut cuerpo = String::new();
        solicitud.as_reader().read_to_string(&mut cuerpo)?;
        // Procesar cuerpo y devolver respuesta.
    }
}
```

El valor `0.0.0.0` permite abrir la página desde la misma computadora o desde otro dispositivo de la red local usando la IP del equipo. Si solo quieres trabajar en tu propia máquina, puedes sustituirlo por `127.0.0.1:8000`.

## Validaciones del servidor

La validación se realiza en Rust, no solo en el navegador. El servidor limita el tamaño de la expresión, filtra caracteres antes de calcular, controla división entre cero, revisa paréntesis y devuelve un mensaje si una ecuación no es lineal. Esto evita que una entrada equivocada cierre el programa.

También se usa `escapar_html` al colocar una operación escrita por el usuario dentro de la página. Por ejemplo, los símbolos `<` y `>` se convierten en texto antes de construir la respuesta HTML.

## Cómo reutilizarlo

Para crear otro formulario, conserva la estructura `GET` para archivos estáticos y `POST` para datos. Cambia la ruta `/calcular`, el nombre del campo y la función que procesa su contenido. Mantén siempre la lectura completa del cuerpo y la respuesta mediante `tiny_http`.
