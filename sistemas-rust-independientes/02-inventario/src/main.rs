use std::fs;
use tiny_http::{Header, Method, Response, Server};
const ARCHIVO: &str = "inventario.txt";
#[derive(Clone)]
struct Producto {
    id: usize,
    codigo: String,
    nombre: String,
    categoria: String,
    stock: i32,
    precio: f64,
    minimo: i32,
}
#[derive(Clone)]
struct Movimiento {
    id: usize,
    producto: usize,
    tipo: String,
    unidades: i32,
    fecha: String,
}
fn main() {
    let servidor = Server::http("0.0.0.0:8102").expect("No se pudo iniciar el servidor");
    println!("Inventario: http://localhost:8102");
    for mut s in servidor.incoming_requests() {
        let ruta = s.url().to_string();
        let r = if s.method() == &Method::Get && ruta == "/estilos.css" {
            Response::from_string(archivo("static/estilos.css")).with_header(tipo("text/css"))
        } else if s.method() == &Method::Post {
            let mut c = String::new();
            let _ = s.as_reader().read_to_string(&mut c);
            let d = formulario(&c);
            let m = match ruta.as_str() {
                "/guardar-producto" => guardar(&d),
                "/movimiento" => movimiento(&d),
                "/eliminar" => eliminar(&d),
                _ => String::new(),
            };
            Response::from_string(pagina(&d, &m)).with_header(tipo("text/html"))
        } else {
            let d = ruta
                .split_once('?')
                .map(|(_, q)| formulario(q))
                .unwrap_or_default();
            Response::from_string(pagina(&d, "")).with_header(tipo("text/html"))
        };
        let _ = s.respond(r);
    }
}
fn guardar(d: &[(String, String)]) -> String {
    let codigo = campo(d, "codigo").trim();
    let nombre = campo(d, "nombre").trim();
    let categoria = campo(d, "categoria").trim();
    let stock = campo(d, "stock").parse::<i32>().unwrap_or(-1);
    let minimo = campo(d, "minimo").parse::<i32>().unwrap_or(-1);
    let precio = campo(d, "precio")
        .replace(',', ".")
        .parse::<f64>()
        .unwrap_or(-1.0);
    if codigo.is_empty() || nombre.is_empty() || categoria.is_empty() {
        return "Completa los datos del producto.".into();
    }
    if !codigo
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-')
        || codigo.len() > 20
    {
        return "El código solo admite letras, números y guiones.".into();
    }
    if stock < 0 || minimo < 0 || precio < 0.0 {
        return "Stock, mínimo y precio no pueden ser negativos.".into();
    }
    let (mut productos, movimientos) = cargar();
    if productos
        .iter()
        .any(|x| x.codigo.eq_ignore_ascii_case(codigo))
    {
        return "Ese código ya está registrado.".into();
    }
    let id = productos.iter().map(|x| x.id).max().unwrap_or(0) + 1;
    productos.push(Producto {
        id,
        codigo: codigo.into(),
        nombre: nombre.into(),
        categoria: categoria.into(),
        stock,
        precio,
        minimo,
    });
    escribir(&productos, &movimientos);
    "Producto registrado.".into()
}
fn movimiento(d: &[(String, String)]) -> String {
    let id = campo(d, "id").parse().unwrap_or(0);
    let unidades = campo(d, "unidades").parse::<i32>().unwrap_or(0);
    let clase = campo(d, "clase");
    let fecha = campo(d, "fecha").trim();
    if unidades < 1 || fecha.is_empty() {
        return "Indica unidades válidas y fecha.".into();
    }
    let (mut productos, mut movimientos) = cargar();
    let Some(producto) = productos.iter_mut().find(|x| x.id == id) else {
        return "Producto no encontrado.".into();
    };
    if clase == "entrada" {
        producto.stock += unidades;
    } else if clase == "salida" && producto.stock >= unidades {
        producto.stock -= unidades;
    } else {
        return "La salida no puede dejar stock negativo.".into();
    }
    let nuevo = movimientos.iter().map(|x| x.id).max().unwrap_or(0) + 1;
    movimientos.push(Movimiento {
        id: nuevo,
        producto: id,
        tipo: clase.into(),
        unidades,
        fecha: fecha.into(),
    });
    escribir(&productos, &movimientos);
    "Movimiento registrado.".into()
}
fn eliminar(d: &[(String, String)]) -> String {
    let id = campo(d, "id").parse().unwrap_or(0);
    let (mut productos, movimientos) = cargar();
    let antes = productos.len();
    productos.retain(|x| x.id != id);
    if antes == productos.len() {
        return "Producto no encontrado.".into();
    }
    escribir(&productos, &movimientos);
    "Producto eliminado.".into()
}
fn cargar() -> (Vec<Producto>, Vec<Movimiento>) {
    let mut p = Vec::new();
    let mut m = Vec::new();
    for l in fs::read_to_string(ARCHIVO).unwrap_or_default().lines() {
        let x: Vec<_> = l.split('|').collect();
        if x.first() == Some(&"P") && x.len() == 8 {
            if let (Ok(id), Ok(stock), Ok(precio), Ok(minimo)) =
                (x[1].parse(), x[5].parse(), x[6].parse(), x[7].parse())
            {
                p.push(Producto {
                    id,
                    codigo: dec(x[2]),
                    nombre: dec(x[3]),
                    categoria: dec(x[4]),
                    stock,
                    precio,
                    minimo,
                })
            }
        } else if x.first() == Some(&"M") && x.len() == 6 {
            if let (Ok(id), Ok(producto), Ok(unidades)) = (x[1].parse(), x[2].parse(), x[4].parse())
            {
                m.push(Movimiento {
                    id,
                    producto,
                    tipo: dec(x[3]),
                    unidades,
                    fecha: dec(x[5]),
                })
            }
        }
    }
    (p, m)
}
fn escribir(p: &[Producto], m: &[Movimiento]) {
    let a = p.iter().map(|x| {
        format!(
            "P|{}|{}|{}|{}|{}|{}|{}",
            x.id,
            cod(&x.codigo),
            cod(&x.nombre),
            cod(&x.categoria),
            x.stock,
            x.precio,
            x.minimo
        )
    });
    let b = m.iter().map(|x| {
        format!(
            "M|{}|{}|{}|{}|{}",
            x.id,
            x.producto,
            cod(&x.tipo),
            x.unidades,
            cod(&x.fecha)
        )
    });
    let _ = fs::write(ARCHIVO, a.chain(b).collect::<Vec<_>>().join("\n"));
}
fn pagina(d: &[(String, String)], msg: &str) -> String {
    let buscar = campo(d, "buscar").to_lowercase();
    let (p, m) = cargar();
    let lista: Vec<_> = p
        .into_iter()
        .filter(|x| {
            buscar.is_empty()
                || x.codigo.to_lowercase().contains(&buscar)
                || x.nombre.to_lowercase().contains(&buscar)
                || x.categoria.to_lowercase().contains(&buscar)
        })
        .collect();
    let total = lista.iter().map(|x| x.stock as f64 * x.precio).sum::<f64>();
    let bajo = lista.iter().filter(|x| x.stock <= x.minimo).count();
    let filas = lista.iter().map(fila).collect::<String>();
    let historial = m
        .iter()
        .rev()
        .take(8)
        .map(|x| {
            format!(
                "<li>{}: {} unidades el {}</li>",
                html(&x.tipo),
                x.unidades,
                html(&x.fecha)
            )
        })
        .collect::<String>();
    archivo("static/index.html")
        .replace("{{MENSAJE}}", &html(msg))
        .replace("{{BUSCAR}}", &html(campo(d, "buscar")))
        .replace("{{TOTAL}}", &format!("{total:.2}"))
        .replace("{{BAJO}}", &bajo.to_string())
        .replace("{{PRODUCTOS}}", &filas)
        .replace("{{MOVIMIENTOS}}", &historial)
}
fn fila(p: &Producto) -> String {
    format!("<tr class=\"{}\"><td>{}</td><td><b>{}</b><br><small>{}</small></td><td>{}</td><td>$ {:.2}</td><td><form action=/movimiento method=post><input type=hidden name=id value={}><input type=hidden name=clase value=entrada><input name=unidades type=number min=1 placeholder=Cant. required><input name=fecha type=date required><button>Entrada</button></form><form action=/movimiento method=post><input type=hidden name=id value={}><input type=hidden name=clase value=salida><input name=unidades type=number min=1 placeholder=Cant. required><input name=fecha type=date required><button>Salida</button></form><form action=/eliminar method=post><input type=hidden name=id value={}><button class=eliminar>Eliminar</button></form></td></tr>",if p.stock<=p.minimo{"bajo"}else{""},html(&p.codigo),html(&p.nombre),html(&p.categoria),p.stock,p.precio,p.id,p.id,p.id)
}
fn archivo(r: &str) -> String {
    fs::read_to_string(format!("{}/{}", env!("CARGO_MANIFEST_DIR"), r)).unwrap_or_default()
}
fn campo<'a>(d: &'a [(String, String)], n: &str) -> &'a str {
    d.iter()
        .find(|(k, _)| k == n)
        .map(|(_, v)| v.as_str())
        .unwrap_or("")
}
fn formulario(t: &str) -> Vec<(String, String)> {
    t.split('&')
        .filter_map(|x| x.split_once('=').map(|(k, v)| (url(k), url(v))))
        .collect()
}
fn url(t: &str) -> String {
    let b = t.as_bytes();
    let mut r = Vec::new();
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'+' {
            r.push(b' ');
            i += 1
        } else if b[i] == b'%' && i + 2 < b.len() {
            let h = [b[i + 1], b[i + 2]];
            if let Ok(v) = u8::from_str_radix(&String::from_utf8_lossy(&h), 16) {
                r.push(v);
                i += 3
            } else {
                r.push(b[i]);
                i += 1
            }
        } else {
            r.push(b[i]);
            i += 1
        }
    }
    String::from_utf8_lossy(&r).to_string()
}
fn html(t: &str) -> String {
    t.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
fn cod(t: &str) -> String {
    t.replace('\\', "\\\\").replace('|', "\\p")
}
fn dec(t: &str) -> String {
    t.replace("\\p", "|").replace("\\\\", "\\")
}
fn tipo(t: &str) -> Header {
    Header::from_bytes("Content-Type", format!("{t}; charset=utf-8")).unwrap()
}
