use std::fs;
use tiny_http::{Header, Method, Response, Server};
#[derive(Clone)]
struct Libro {
    codigo: String,
    titulo: String,
    autor: String,
    anio: i32,
    disponibles: i32,
}
#[derive(Clone)]
struct PrestamoLibro {
    libro: String,
    usuario: String,
    prestamo: String,
    devolucion: String,
    activo: bool,
}
fn main() {
    let s = Server::http("0.0.0.0:8103")
        .expect("No se pudo iniciar el servidor en el puerto 8103. ¿Está ocupado?");
    println!("Biblioteca: http://localhost:8103");
    for mut r in s.incoming_requests() {
        let ruta = r.url().to_string();
        let resp = if r.method() == &Method::Get && ruta == "/estilos.css" {
            Response::from_string(archivo("static/estilos.css")).with_header(h("text/css"))
        } else if r.method() == &Method::Post {
            let mut c = String::new();
            let _ = r.as_reader().read_to_string(&mut c);
            let d = f(&c);
            let m = match ruta.as_str() {
                "/libro" => agregar(&d),
                "/prestar" => prestar(&d),
                "/devolver" => devolver(&d),
                _ => String::new(),
            };
            Response::from_string(pagina(&m)).with_header(h("text/html"))
        } else {
            Response::from_string(pagina("")).with_header(h("text/html"))
        };
        let _ = r.respond(resp);
    }
}
fn agregar(d: &[(String, String)]) -> String {
    let codigo = v(d, "codigo");
    let titulo = v(d, "titulo");
    let autor = v(d, "autor");
    let anio = v(d, "anio").parse::<i32>().unwrap_or(0);
    let cantidad = v(d, "cantidad").parse::<i32>().unwrap_or(0);
    if codigo.is_empty() || titulo.is_empty() || autor.is_empty() || anio < 1000 || cantidad < 1 {
        return "Completa datos válidos del libro.".into();
    }
    let mut l = libros();
    if l.iter().any(|x| x.codigo == codigo) {
        return "El código de libro ya existe.".into();
    }
    l.push(Libro {
        codigo: codigo.into(),
        titulo: titulo.into(),
        autor: autor.into(),
        anio,
        disponibles: cantidad,
    });
    guardar_libros(&l);
    "Libro agregado al catálogo.".into()
}
fn prestar(d: &[(String, String)]) -> String {
    let codigo = v(d, "codigo");
    let usuario = v(d, "usuario");
    let fecha = v(d, "fecha");
    if usuario.is_empty() || fecha.is_empty() {
        return "Indica usuario y fecha de préstamo.".into();
    }
    let mut l = libros();
    let Some(x) = l.iter_mut().find(|x| x.codigo == codigo) else {
        return "Libro no encontrado.".into();
    };
    if x.disponibles < 1 {
        return "No hay ejemplares disponibles.".into();
    }
    x.disponibles -= 1;
    guardar_libros(&l);
    let mut p = prestamos();
    p.push(PrestamoLibro {
        libro: codigo.into(),
        usuario: usuario.into(),
        prestamo: fecha.into(),
        devolucion: String::new(),
        activo: true,
    });
    guardar_prestamos(&p);
    "Préstamo registrado.".into()
}
fn devolver(d: &[(String, String)]) -> String {
    let codigo = v(d, "codigo");
    let fecha = v(d, "fecha");
    let mut p = prestamos();
    let Some(x) = p.iter_mut().find(|x| x.libro == codigo && x.activo) else {
        return "No existe préstamo activo para ese libro.".into();
    };
    x.activo = false;
    x.devolucion = fecha.into();
    guardar_prestamos(&p);
    let mut l = libros();
    if let Some(x) = l.iter_mut().find(|x| x.codigo == codigo) {
        x.disponibles += 1;
        guardar_libros(&l)
    }
    "Devolución registrada.".into()
}
fn libros() -> Vec<Libro> {
    fs::read_to_string("libros.txt")
        .unwrap_or_default()
        .lines()
        .filter_map(|x| {
            let p: Vec<_> = x.split('|').collect();
            Some(Libro {
                codigo: p.get(0)?.to_string(),
                titulo: p.get(1)?.to_string(),
                autor: p.get(2)?.to_string(),
                anio: p.get(3)?.parse().ok()?,
                disponibles: p.get(4)?.parse().ok()?,
            })
        })
        .collect()
}
fn prestamos() -> Vec<PrestamoLibro> {
    fs::read_to_string("prestamos.txt")
        .unwrap_or_default()
        .lines()
        .filter_map(|x| {
            let p: Vec<_> = x.split('|').collect();
            Some(PrestamoLibro {
                libro: p.get(0)?.to_string(),
                usuario: p.get(1)?.to_string(),
                prestamo: p.get(2)?.to_string(),
                devolucion: p.get(3)?.to_string(),
                activo: p.get(4)? == &"1",
            })
        })
        .collect()
}
fn guardar_libros(l: &[Libro]) {
    let _ = fs::write(
        "libros.txt",
        l.iter()
            .map(|x| {
                format!(
                    "{}|{}|{}|{}|{}",
                    x.codigo, x.titulo, x.autor, x.anio, x.disponibles
                )
            })
            .collect::<Vec<_>>()
            .join("\n"),
    );
}
fn guardar_prestamos(p: &[PrestamoLibro]) {
    let _ = fs::write(
        "prestamos.txt",
        p.iter()
            .map(|x| {
                format!(
                    "{}|{}|{}|{}|{}",
                    x.libro,
                    x.usuario,
                    x.prestamo,
                    x.devolucion,
                    if x.activo { 1 } else { 0 }
                )
            })
            .collect::<Vec<_>>()
            .join("\n"),
    );
}
fn pagina(m: &str) -> String {
    let l = libros();
    let p = prestamos();
    let libros = l
        .iter()
        .map(|x| {
            format!(
                "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
                x.codigo, x.titulo, x.autor, x.anio, x.disponibles
            )
        })
        .collect::<String>();
    let activos = p
        .iter()
        .filter(|x| x.activo)
        .map(|x| {
            format!(
                "<li>{} prestado a {} desde {}</li>",
                x.libro, x.usuario, x.prestamo
            )
        })
        .collect::<String>();
    archivo("static/index.html")
        .replace("{{MENSAJE}}", m)
        .replace("{{LIBROS}}", &libros)
        .replace("{{PRESTAMOS}}", &activos)
}
fn archivo(r: &str) -> String {
    fs::read_to_string(format!("{}/{}", env!("CARGO_MANIFEST_DIR"), r)).unwrap_or_default()
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
                .map(|(k, v)| (k.into(), v.replace('+', " ")))
        })
        .collect()
}
fn h(t: &str) -> Header {
    Header::from_bytes("Content-Type", format!("{t}; charset=utf-8")).unwrap()
}
