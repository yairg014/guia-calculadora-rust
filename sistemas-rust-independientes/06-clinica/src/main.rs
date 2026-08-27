use std::fs;
use tiny_http::{Header, Method, Response, Server};
#[derive(Clone)]
struct Paciente {
    expediente: String,
    nombre: String,
    telefono: String,
}
#[derive(Clone)]
struct Cita {
    paciente: String,
    doctor: String,
    fecha: String,
    motivo: String,
    estado: String,
}
fn main() {
    // Inicia el servidor web local de la clínica.
    let s = Server::http("0.0.0.0:8106")
        .expect("No se pudo iniciar el servidor en el puerto 8106. ¿Está ocupado?");
    println!("Clínica: http://localhost:8106");
    for mut r in s.incoming_requests() {
        let ruta = r.url().to_string();
        if r.method() == &Method::Get && ruta == "/estilos.css" {
            let _ = r.respond(
                Response::from_string(archivo("static/estilos.css")).with_header(tipo("text/css")),
            );
            continue;
        }
        if r.method() == &Method::Get {
            let _ = r.respond(Response::from_string(pagina("")).with_header(tipo("text/html")));
            continue;
        }
        let mut c = String::new();
        let _ = r.as_reader().read_to_string(&mut c);
        let d = f(&c);
        let m = if r.method() == &Method::Post && ruta == "/paciente" {
            paciente(&d)
        } else if r.method() == &Method::Post && ruta == "/cita" {
            cita(&d)
        } else {
            String::new()
        };
        let _ = r.respond(Response::from_string(pagina(&m)).with_header(tipo("text/html")));
    }
}
fn paciente(d: &[(String, String)]) -> String {
    let e = v(d, "expediente");
    let n = v(d, "nombre");
    let t = v(d, "telefono");
    if e.is_empty() || n.is_empty() || t.chars().filter(|c| c.is_ascii_digit()).count() < 8 {
        return "Datos de paciente inválidos.".into();
    }
    fs::write("pacientes.txt", format!("{}|{}|{}\n", e, n, t)).ok();
    "Paciente registrado.".into()
}
fn cita(d: &[(String, String)]) -> String {
    let p = v(d, "paciente");
    let doc = v(d, "doctor");
    let fecha = v(d, "fecha");
    let motivo = v(d, "motivo");
    if p.is_empty() || doc.is_empty() || fecha.is_empty() || motivo.is_empty() {
        return "Completa los datos de la cita.".into();
    }
    fs::write(
        "citas.txt",
        format!("{}|{}|{}|{}|Pendiente\n", p, doc, fecha, motivo),
    )
    .ok();
    "Cita agendada como pendiente.".into()
}
fn pagina(m: &str) -> String {
    archivo("static/index.html").replace("{{MENSAJE}}", m)
}
fn v<'a>(d: &'a [(String, String)], n: &str) -> &'a str {
    d.iter()
        .find(|(k, _)| k == n)
        .map(|(_, x)| x.as_str())
        .unwrap_or("")
}
fn f(t: &str) -> Vec<(String, String)> {
    t.split('&')
        .filter_map(|x| {
            x.split_once('=')
                .map(|(a, b)| (a.into(), b.replace('+', " ")))
        })
        .collect()
}
fn archivo(ruta: &str) -> String {
    fs::read_to_string(format!("{}/{}", env!("CARGO_MANIFEST_DIR"), ruta)).unwrap_or_default()
}
fn tipo(valor: &str) -> Header {
    Header::from_bytes("Content-Type", format!("{valor}; charset=utf-8")).unwrap()
}
