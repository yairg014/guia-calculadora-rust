use std::fs;
use tiny_http::{Header, Method, Response, Server};
#[derive(Clone)]
struct Estudiante {
    matricula: String,
    nombre: String,
    carrera: String,
    p1: f64,
    p2: f64,
    p3: f64,
}
fn main() {
    let s = Server::http("0.0.0.0:8104")
        .expect("No se pudo iniciar el servidor en el puerto 8104. ¿Está ocupado?");
    println!("Estudiantes: http://localhost:8104");
    for mut r in s.incoming_requests() {
        let ruta = r.url();
        let resp = if ruta == "/estilos.css" {
            Response::from_string(a("static/estilos.css")).with_header(h("text/css"))
        } else if r.method() == &Method::Post {
            let mut c = String::new();
            let _ = r.as_reader().read_to_string(&mut c);
            Response::from_string(pagina(&guardar(&f(&c)))).with_header(h("text/html"))
        } else {
            Response::from_string(pagina("")).with_header(h("text/html"))
        };
        let _ = r.respond(resp);
    }
}
fn guardar(d: &[(String, String)]) -> String {
    let e = Estudiante {
        matricula: v(d, "matricula").into(),
        nombre: v(d, "nombre").into(),
        carrera: v(d, "carrera").into(),
        p1: num(v(d, "p1")),
        p2: num(v(d, "p2")),
        p3: num(v(d, "p3")),
    };
    if e.matricula.is_empty()
        || e.nombre.is_empty()
        || e.carrera.is_empty()
        || [e.p1, e.p2, e.p3].iter().any(|x| *x < 0. || *x > 10.)
    {
        return "Completa datos y calificaciones de 0 a 10.".into();
    }
    let mut x = cargar();
    if x.iter().any(|a| a.matricula == e.matricula) {
        return "La matrícula ya existe.".into();
    }
    x.push(e);
    let _ = fs::write(
        "estudiantes.txt",
        x.iter()
            .map(|a| {
                format!(
                    "{}|{}|{}|{}|{}|{}",
                    a.matricula, a.nombre, a.carrera, a.p1, a.p2, a.p3
                )
            })
            .collect::<Vec<_>>()
            .join("\n"),
    );
    "Estudiante registrado; promedio calculado.".into()
}
fn cargar() -> Vec<Estudiante> {
    fs::read_to_string("estudiantes.txt")
        .unwrap_or_default()
        .lines()
        .filter_map(|x| {
            let p: Vec<_> = x.split('|').collect();
            Some(Estudiante {
                matricula: (*p.get(0)?).into(),
                nombre: (*p.get(1)?).into(),
                carrera: (*p.get(2)?).into(),
                p1: num(p.get(3)?),
                p2: num(p.get(4)?),
                p3: num(p.get(5)?),
            })
        })
        .collect()
}
fn promedio(e: &Estudiante) -> f64 {
    (e.p1 + e.p2 + e.p3) / 3.
}
fn estado(x: f64) -> &'static str {
    if x >= 8.0 {
        "Aprobado"
    } else if x >= 6.0 {
        "Riesgo"
    } else {
        "Reprobado"
    }
}
fn pagina(m: &str) -> String {
    let mut e = cargar();
    e.sort_by(|a, b| promedio(b).total_cmp(&promedio(a)));
    let filas = e
        .iter()
        .map(|x| {
            format!(
                "<tr><td>{}</td><td>{}</td><td>{}</td><td>{:.1}</td><td>{}</td></tr>",
                x.matricula,
                x.nombre,
                x.carrera,
                promedio(x),
                estado(promedio(x))
            )
        })
        .collect::<String>();
    a("static/index.html")
        .replace("{{MENSAJE}}", m)
        .replace("{{ALUMNOS}}", &filas)
}
fn a(r: &str) -> String {
    fs::read_to_string(format!("{}/{}", env!("CARGO_MANIFEST_DIR"), r)).unwrap_or_default()
}
fn v<'a>(d: &'a [(String, String)], n: &str) -> &'a str {
    d.iter()
        .find(|(k, _)| k == n)
        .map(|(_, x)| x.as_str())
        .unwrap_or("")
}
fn num(x: &str) -> f64 {
    x.parse().unwrap_or(-1.)
}
fn f(t: &str) -> Vec<(String, String)> {
    t.split('&')
        .filter_map(|x| {
            x.split_once('=')
                .map(|(a, b)| (a.into(), b.replace('+', " ")))
        })
        .collect()
}
fn h(t: &str) -> Header {
    Header::from_bytes("Content-Type", format!("{t}; charset=utf-8")).unwrap()
}
