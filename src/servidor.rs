//! Módulo 1: servidor HTTP y manejo seguro de formularios.
//! tiny_http recibe la solicitud completa antes de responder, evitando reinicios de conexión.

use std::fs;
use tiny_http::{Header, Method, Response, Server};

use crate::{calculadora, ecuaciones};

pub fn iniciar() {
    let servidor = Server::http("0.0.0.0:8000").expect("No se pudo iniciar el servidor en el puerto 8000");
    println!("Calculadora lista en http://localhost:8000");

    for mut solicitud in servidor.incoming_requests() {
        let ruta = solicitud.url().to_string();

        let respuesta = if solicitud.method() == &Method::Get && ruta == "/estilos.css" {
            Response::from_string(archivo("static/estilos.css")).with_header(tipo("text/css"))
        } else if solicitud.method() == &Method::Post && ruta == "/calcular" {
            let mut cuerpo = String::new();
            let expresion = if solicitud.as_reader().read_to_string(&mut cuerpo).is_ok() {
                valor_formulario(&cuerpo, "expresion")
            } else {
                String::new()
            };

            let resultado = if expresion.contains('=') {
                Ok(ecuaciones::resolver(&expresion))
            } else {
                calculadora::calcular(&expresion)
            };

            Response::from_string(pagina(&expresion, resultado)).with_header(tipo("text/html"))
        } else {
            Response::from_string(pagina("0", Ok("0".to_string()))).with_header(tipo("text/html"))
        };

        // Si el navegador se cierra antes de terminar, el servidor continúa con la siguiente solicitud.
        let _ = solicitud.respond(respuesta);
    }
}

fn archivo(ruta: &str) -> String {
    fs::read_to_string(format!("{}/{}", env!("CARGO_MANIFEST_DIR"), ruta)).unwrap_or_default()
}

fn pagina(expresion: &str, resultado: Result<String, String>) -> String {
    let plantilla = archivo("static/index.html");
    let (valor, error) = match resultado {
        Ok(valor) => (valor, String::new()),
        Err(mensaje) => ("Error".to_string(), mensaje),
    };

    plantilla
        .replace("{{EXPRESION}}", &escapar_html(expresion))
        .replace("{{RESULTADO}}", &escapar_html(&valor))
        .replace("{{ERROR}}", &escapar_html(&error))
}

/// Extrae `expresion=...` de un formulario enviado por el navegador.
fn valor_formulario(cuerpo: &str, nombre: &str) -> String {
    cuerpo
        .split('&')
        .find_map(|campo| campo.strip_prefix(&format!("{nombre}=")))
        .map(decodificar_url)
        .unwrap_or_default()
}

/// Convierte caracteres como `%2B` a `+` y `+` a espacios.
fn decodificar_url(texto: &str) -> String {
    let bytes = texto.as_bytes();
    let mut resultado = Vec::new();
    let mut posicion = 0;

    while posicion < bytes.len() {
        if bytes[posicion] == b'+' {
            resultado.push(b' ');
            posicion += 1;
        } else if bytes[posicion] == b'%' && posicion + 2 < bytes.len() {
            let hex = [bytes[posicion + 1], bytes[posicion + 2]];
            if let Ok(valor) = u8::from_str_radix(&String::from_utf8_lossy(&hex), 16) {
                resultado.push(valor);
                posicion += 3;
            } else {
                resultado.push(bytes[posicion]);
                posicion += 1;
            }
        } else {
            resultado.push(bytes[posicion]);
            posicion += 1;
        }
    }
    String::from_utf8_lossy(&resultado).to_string()
}

/// Evita que el texto escrito por el usuario se interprete como HTML.
fn escapar_html(texto: &str) -> String {
    texto
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn tipo(valor: &str) -> Header {
    Header::from_bytes("Content-Type", format!("{valor}; charset=utf-8")).unwrap()
}

#[cfg(test)]
mod pruebas {
    use super::*;

    #[test]
    fn lee_y_decodifica_un_formulario() {
        assert_eq!(valor_formulario("expresion=2%2B2", "expresion"), "2+2");
        assert_eq!(valor_formulario("otro=1&expresion=sin%2830%29", "expresion"), "sin(30)");
    }

    #[test]
    fn escapa_texto_para_html() {
        assert_eq!(escapar_html("<script>"), "&lt;script&gt;");
    }
}
