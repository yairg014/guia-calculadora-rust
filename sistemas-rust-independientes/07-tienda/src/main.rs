use std::fs;
use tiny_http::{Header, Method, Response, Server};
#[derive(Clone)]
struct ProductoTienda {
    codigo: String,
    nombre: String,
    precio: f64,
    stock: i32,
}
#[derive(Clone)]
struct Venta {
    producto: String,
    unidades: i32,
    total: f64,
    fecha: String,
}
fn main() {
    let s = Server::http("0.0.0.0:8107")
        .expect("No se pudo iniciar el servidor en el puerto 8107. ¿Está ocupado?");
    println!("Tienda: http://localhost:8107");
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
        let m = if r.method() == &Method::Post && ruta == "/producto" {
            producto(&d)
        } else if r.method() == &Method::Post && ruta == "/venta" {
            venta(&d)
        } else {
            String::new()
        };
        let _ = r.respond(Response::from_string(pagina(&m)).with_header(tipo("text/html")));
    }
}
fn producto(d: &[(String, String)]) -> String {
    let x = ProductoTienda {
        codigo: v(d, "codigo").into(),
        nombre: v(d, "nombre").into(),
        precio: num(v(d, "precio")),
        stock: v(d, "stock").parse().unwrap_or(-1),
    };
    if x.codigo.is_empty() || x.nombre.is_empty() || x.precio < 0. || x.stock < 0 {
        return "Datos de producto inválidos.".into();
    }
    fs::write(
        "productos.txt",
        format!("{}|{}|{}|{}\n", x.codigo, x.nombre, x.precio, x.stock),
    )
    .ok();
    "Producto agregado.".into()
}
fn venta(d: &[(String, String)]) -> String {
    let c = v(d, "codigo");
    let u = v(d, "unidades").parse::<i32>().unwrap_or(0);
    if u < 1 {
        return "Cantidad inválida.".into();
    }
    let mut p = cargar();
    let Some(x) = p.iter_mut().find(|x| x.codigo == c) else {
        return "Producto no encontrado.".into();
    };
    if u > x.stock {
        return "Stock insuficiente.".into();
    }
    x.stock -= u;
    let t = x.precio * u as f64;
    let ticket = Venta {
        producto: x.nombre.clone(),
        unidades: u,
        total: t,
        fecha: v(d, "fecha").into(),
    };
    fs::write(
        "productos.txt",
        p.iter()
            .map(|x| format!("{}|{}|{}|{}", x.codigo, x.nombre, x.precio, x.stock))
            .collect::<Vec<_>>()
            .join("\n"),
    )
    .ok();
    fs::write(
        "ventas.txt",
        format!(
            "{}|{}|{}|{}\n",
            ticket.producto, ticket.unidades, ticket.total, ticket.fecha
        ),
    )
    .ok();
    format!(
        "Venta registrada. Ticket: {} x{} = ${:.2}",
        ticket.producto, u, t
    )
}
fn cargar() -> Vec<ProductoTienda> {
    fs::read_to_string("productos.txt")
        .unwrap_or_default()
        .lines()
        .filter_map(|l| {
            let p: Vec<_> = l.split('|').collect();
            Some(ProductoTienda {
                codigo: p.get(0)?.to_string(),
                nombre: p.get(1)?.to_string(),
                precio: num(p.get(2)?),
                stock: p.get(3)?.parse().ok()?,
            })
        })
        .collect()
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
fn archivo(ruta: &str) -> String {
    fs::read_to_string(format!("{}/{}", env!("CARGO_MANIFEST_DIR"), ruta)).unwrap_or_default()
}
fn tipo(valor: &str) -> Header {
    Header::from_bytes("Content-Type", format!("{valor}; charset=utf-8")).unwrap()
}
