use std::fs;
use tiny_http::{Header, Method, Response, Server};

#[derive(Clone)]
struct Prestamo {
    folio: String,
    cliente: String,
    monto: f64,
    interes: f64,
    saldo: f64,
    fecha: String,
    estado: String,
}
#[derive(Clone)]
struct Abono {
    folio: String,
    monto: f64,
    fecha: String,
}

fn main() {
    // Inicia el servidor web local de préstamos.
    let servidor = Server::http("0.0.0.0:8108")
        .expect("No se pudo iniciar el servidor en el puerto 8108. ¿Está ocupado?");
    println!("Préstamos: http://localhost:8108");

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
        let datos = formulario(&cuerpo);
        let mensaje = match ruta.as_str() {
            "/prestamo" => registrar(&datos),
            "/abono" => abonar(&datos),
            _ => "Ruta no encontrada.".into(),
        };
        let _ = solicitud
            .respond(Response::from_string(pagina(&mensaje)).with_header(tipo("text/html")));
    }
}
fn registrar(d: &[(String, String)]) -> String {
    let p = Prestamo {
        folio: valor(d, "folio").into(),
        cliente: valor(d, "cliente").into(),
        monto: numero(valor(d, "monto")),
        interes: numero(valor(d, "interes")),
        saldo: numero(valor(d, "monto")),
        fecha: valor(d, "fecha").into(),
        estado: "Activo".into(),
    };
    if p.folio.is_empty()
        || p.cliente.is_empty()
        || p.monto <= 0.0
        || p.interes < 0.0
        || p.fecha.is_empty()
    {
        return "Datos de préstamo inválidos.".into();
    }
    let texto = format!(
        "{}|{}|{}|{}|{}|{}|{}\n",
        p.folio, p.cliente, p.monto, p.interes, p.saldo, p.fecha, p.estado
    );
    let _ = fs::write("prestamos.txt", texto);
    "Préstamo activo registrado.".into()
}
fn abonar(d: &[(String, String)]) -> String {
    let a = Abono {
        folio: valor(d, "folio").into(),
        monto: numero(valor(d, "monto")),
        fecha: valor(d, "fecha").into(),
    };
    if a.folio.is_empty() || a.monto <= 0.0 || a.fecha.is_empty() {
        return "Abono inválido.".into();
    }
    let _ = fs::write(
        "abonos.txt",
        format!("{}|{}|{}\n", a.folio, a.monto, a.fecha),
    );
    "Abono registrado.".into()
}
fn pagina(mensaje: &str) -> String {
    archivo("static/index.html").replace("{{MENSAJE}}", mensaje)
}
fn archivo(ruta: &str) -> String {
    fs::read_to_string(format!("{}/{}", env!("CARGO_MANIFEST_DIR"), ruta)).unwrap_or_default()
}
fn valor<'a>(d: &'a [(String, String)], nombre: &str) -> &'a str {
    d.iter()
        .find(|(k, _)| k == nombre)
        .map(|(_, v)| v.as_str())
        .unwrap_or("")
}
fn numero(texto: &str) -> f64 {
    texto.parse().unwrap_or(-1.0)
}
fn formulario(texto: &str) -> Vec<(String, String)> {
    texto
        .split('&')
        .filter_map(|x| {
            x.split_once('=')
                .map(|(k, v)| (k.into(), v.replace('+', " ")))
        })
        .collect()
}
fn tipo(valor: &str) -> Header {
    Header::from_bytes("Content-Type", format!("{valor}; charset=utf-8")).unwrap()
}
