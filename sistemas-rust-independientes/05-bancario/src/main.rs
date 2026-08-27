use std::fs;
use tiny_http::{Header, Method, Response, Server};
#[derive(Clone)]
struct Cuenta {
    numero: String,
    titular: String,
    saldo: f64,
}
#[derive(Clone)]
struct Movimiento {
    cuenta: String,
    tipo: String,
    monto: f64,
    fecha: String,
    saldo: f64,
}
fn main() {
    let servidor = Server::http("0.0.0.0:8105")
        .expect("No se pudo iniciar el servidor en el puerto 8105. ¿Está ocupado?");
    println!("Bancario: http://localhost:8105");

    for mut r in servidor.incoming_requests() {
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
        let m = if r.method() == &Method::Post && ruta == "/cuenta" {
            crear(&d)
        } else if r.method() == &Method::Post && ruta == "/movimiento" {
            mover(&d)
        } else {
            String::new()
        };
        let _ = r.respond(Response::from_string(pagina(&m)).with_header(tipo("text/html")));
    }
}
fn crear(d: &[(String, String)]) -> String {
    let n = v(d, "numero");
    let t = v(d, "titular");
    if n.is_empty() || t.is_empty() {
        return "Número y titular son obligatorios.".into();
    }
    let mut x = cargar();
    if x.iter().any(|a| a.numero == n) {
        return "La cuenta ya existe.".into();
    }
    x.push(Cuenta {
        numero: n.into(),
        titular: t.into(),
        saldo: 0.,
    });
    guardar(&x);
    "Cuenta creada con saldo inicial de $0.00.".into()
}
fn mover(d: &[(String, String)]) -> String {
    let n = v(d, "numero");
    let monto = v(d, "monto").parse::<f64>().unwrap_or(0.);
    let tipo = v(d, "tipo");
    let fecha = v(d, "fecha");
    let mut x = cargar();
    let Some(c) = x.iter_mut().find(|a| a.numero == n) else {
        return "Cuenta no encontrada.".into();
    };
    if monto <= 0. {
        return "Monto inválido.".into();
    }
    if tipo == "retiro" && monto > c.saldo {
        return "Saldo insuficiente.".into();
    }
    if tipo == "deposito" {
        c.saldo += monto
    } else {
        c.saldo -= monto
    }
    let mov = Movimiento {
        cuenta: n.into(),
        tipo: tipo.into(),
        monto,
        fecha: fecha.into(),
        saldo: c.saldo,
    };
    guardar(&x);
    fs::write(
        "movimientos.txt",
        format!(
            "{}|{}|{}|{}|{}\n",
            mov.cuenta, mov.tipo, mov.monto, mov.fecha, mov.saldo
        ),
    )
    .ok();
    "Movimiento registrado.".into()
}
fn cargar() -> Vec<Cuenta> {
    fs::read_to_string("cuentas.txt")
        .unwrap_or_default()
        .lines()
        .filter_map(|l| {
            let p: Vec<_> = l.split('|').collect();
            Some(Cuenta {
                numero: p.get(0)?.to_string(),
                titular: p.get(1)?.to_string(),
                saldo: p.get(2)?.parse().ok()?,
            })
        })
        .collect()
}
fn guardar(x: &[Cuenta]) {
    fs::write(
        "cuentas.txt",
        x.iter()
            .map(|a| format!("{}|{}|{}", a.numero, a.titular, a.saldo))
            .collect::<Vec<_>>()
            .join("\n"),
    )
    .ok();
}
fn pagina(m: &str) -> String {
    let x = cargar();
    let total = x.iter().map(|a| a.saldo).sum::<f64>();
    let filas = x
        .iter()
        .map(|a| {
            format!(
                "<tr><td>{}</td><td>{}</td><td>$ {:.2}</td></tr>",
                a.numero, a.titular, a.saldo
            )
        })
        .collect::<String>();
    archivo("static/index.html")
        .replace("{{MENSAJE}}", m)
        .replace("{{TOTAL}}", &format!("{total:.2}"))
        .replace("{{CUENTAS}}", &filas)
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
