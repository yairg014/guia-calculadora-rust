use std::fs;
use tiny_http::{Header, Method, Response, Server};

#[derive(Clone)]
struct Contacto {
    nombre: String,
    telefono: String,
    correo: String,
    categoria: String,
    favorito: bool,
    notas: String,
}

fn main() {
    let servidor = Server::http("0.0.0.0:8109")
        .expect("No se pudo iniciar el servidor en el puerto 8109. ¿Está ocupado?");
    println!("Agenda de Contactos: http://localhost:8109");
    for mut solicitud in servidor.incoming_requests() {
        let ruta = solicitud.url().to_string();
        if solicitud.method() == &Method::Get && ruta == "/estilos.css" {
            let _ = solicitud.respond(
                Response::from_string(archivo("static/estilos.css")).with_header(tipo("text/css")),
            );
            continue;
        }
        if solicitud.method() == &Method::Get {
            let _ =
                solicitud.respond(Response::from_string(pagina("")).with_header(tipo("text/html")));
            continue;
        }
        let mut cuerpo = String::new();
        let _ = solicitud.as_reader().read_to_string(&mut cuerpo);
        let mensaje = if ruta == "/guardar" {
            guardar(&formulario(&cuerpo))
        } else {
            "Ruta no encontrada.".into()
        };
        let _ = solicitud
            .respond(Response::from_string(pagina(&mensaje)).with_header(tipo("text/html")));
    }
}
fn guardar(d: &[(String, String)]) -> String {
    let c = Contacto {
        nombre: valor(d, "nombre").into(),
        telefono: valor(d, "telefono").into(),
        correo: valor(d, "correo").into(),
        categoria: valor(d, "categoria").into(),
        favorito: valor(d, "favorito") == "si",
        notas: valor(d, "notas").into(),
    };
    if c.nombre.is_empty()
        || c.categoria.is_empty()
        || c.telefono.chars().filter(|x| x.is_ascii_digit()).count() < 8
        || !c.correo.contains('@')
    {
        return "Nombre, categoría, teléfono y correo válidos son obligatorios.".into();
    }
    let _ = fs::write(
        "contactos.txt",
        format!(
            "{}|{}|{}|{}|{}|{}\n",
            c.nombre, c.telefono, c.correo, c.categoria, c.favorito, c.notas
        ),
    );
    "Contacto guardado.".into()
}
fn pagina(mensaje: &str) -> String {
    archivo("static/index.html").replace("{{MENSAJE}}", mensaje)
}
fn archivo(ruta: &str) -> String {
    fs::read_to_string(format!("{}/{}", env!("CARGO_MANIFEST_DIR"), ruta)).unwrap_or_default()
}
fn valor<'a>(d: &'a [(String, String)], n: &str) -> &'a str {
    d.iter()
        .find(|(k, _)| k == n)
        .map(|(_, v)| v.as_str())
        .unwrap_or("")
}
fn formulario(t: &str) -> Vec<(String, String)> {
    t.split('&')
        .filter_map(|x| {
            x.split_once('=')
                .map(|(k, v)| (k.into(), v.replace('+', " ")))
        })
        .collect()
}
fn tipo(valor: &str) -> Header {
    Header::from_bytes("Content-Type", format!("{valor}; charset=utf-8")).unwrap()
}
