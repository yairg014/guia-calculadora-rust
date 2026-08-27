use std::fs;
use tiny_http::{Header, Method, Response, Server};

const ARCHIVO: &str = "notas.txt";

#[derive(Clone)]
struct Nota {
    id: usize,
    titulo: String,
    contenido: String,
    categoria: String,
    fecha: String,
    destacada: bool,
}

fn main() {
    let servidor = Server::http("0.0.0.0:8101").expect("No se pudo iniciar el servidor");
    println!("Bloc de Notas: http://localhost:8101");
    for mut solicitud in servidor.incoming_requests() {
        let ruta = solicitud.url().to_string();
        let respuesta = if solicitud.method() == &Method::Get && ruta == "/estilos.css" {
            Response::from_string(archivo("static/estilos.css")).with_header(tipo("text/css"))
        } else if solicitud.method() == &Method::Post {
            let mut cuerpo = String::new();
            let _ = solicitud.as_reader().read_to_string(&mut cuerpo);
            let datos = formulario(&cuerpo);
            let mensaje = match ruta.as_str() {
                "/guardar" => guardar(&datos),
                "/eliminar" => eliminar(&datos),
                "/destacar" => destacar(&datos),
                _ => String::new(),
            };
            Response::from_string(pagina(&datos, &mensaje)).with_header(tipo("text/html"))
        } else {
            let datos = ruta
                .split_once('?')
                .map(|(_, q)| formulario(q))
                .unwrap_or_default();
            Response::from_string(pagina(&datos, "")).with_header(tipo("text/html"))
        };
        let _ = solicitud.respond(respuesta);
    }
}

fn guardar(datos: &[(String, String)]) -> String {
    let titulo = campo(datos, "titulo").trim();
    let contenido = campo(datos, "contenido").trim();
    let categoria = campo(datos, "categoria").trim();
    let fecha = campo(datos, "fecha").trim();
    let id = campo(datos, "id").parse().unwrap_or(0);
    let destacada = campo(datos, "destacada") == "si";
    if titulo.is_empty() || contenido.is_empty() || categoria.is_empty() || fecha.is_empty() {
        return "Completa título, contenido, categoría y fecha.".into();
    }
    if titulo.len() > 60 || contenido.len() > 1800 || categoria.len() > 30 {
        return "Se superó el límite de caracteres permitido.".into();
    }
    let mut notas = cargar();
    if id == 0 {
        let nuevo = notas.iter().map(|n| n.id).max().unwrap_or(0) + 1;
        notas.push(Nota {
            id: nuevo,
            titulo: titulo.into(),
            contenido: contenido.into(),
            categoria: categoria.into(),
            fecha: fecha.into(),
            destacada,
        });
    } else if let Some(n) = notas.iter_mut().find(|n| n.id == id) {
        n.titulo = titulo.into();
        n.contenido = contenido.into();
        n.categoria = categoria.into();
        n.fecha = fecha.into();
        n.destacada = destacada;
    } else {
        return "No se encontró la nota a editar.".into();
    }
    guardar_archivo(&notas);
    "Nota guardada.".into()
}
fn eliminar(datos: &[(String, String)]) -> String {
    let id = campo(datos, "id").parse().unwrap_or(0);
    let mut notas = cargar();
    let antes = notas.len();
    notas.retain(|n| n.id != id);
    if antes == notas.len() {
        return "La nota no existe.".into();
    }
    guardar_archivo(&notas);
    "Nota eliminada.".into()
}
fn destacar(datos: &[(String, String)]) -> String {
    let id = campo(datos, "id").parse().unwrap_or(0);
    let mut notas = cargar();
    if let Some(n) = notas.iter_mut().find(|n| n.id == id) {
        n.destacada = !n.destacada;
        let estado = n.destacada;
        guardar_archivo(&notas);
        return if estado {
            "Nota destacada."
        } else {
            "Nota sin destacar."
        }
        .into();
    }
    "La nota no existe.".into()
}
fn cargar() -> Vec<Nota> {
    fs::read_to_string(ARCHIVO)
        .unwrap_or_default()
        .lines()
        .filter_map(|l| {
            let p: Vec<_> = l.split('|').collect();
            if p.len() != 6 {
                return None;
            }
            Some(Nota {
                id: p[0].parse().ok()?,
                titulo: dec(p[1]),
                contenido: dec(p[2]),
                categoria: dec(p[3]),
                fecha: dec(p[4]),
                destacada: p[5] == "1",
            })
        })
        .collect()
}
fn guardar_archivo(notas: &[Nota]) {
    let texto = notas
        .iter()
        .map(|n| {
            format!(
                "{}|{}|{}|{}|{}|{}",
                n.id,
                cod(&n.titulo),
                cod(&n.contenido),
                cod(&n.categoria),
                cod(&n.fecha),
                if n.destacada { 1 } else { 0 }
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let _ = fs::write(ARCHIVO, texto);
}
fn pagina(datos: &[(String, String)], mensaje: &str) -> String {
    let buscar = campo(datos, "buscar").to_lowercase();
    let categoria = campo(datos, "categoria_filtro").to_lowercase();
    let editar = campo(datos, "editar").parse().unwrap_or(0);
    let notas: Vec<_> = cargar()
        .into_iter()
        .filter(|n| {
            (buscar.is_empty()
                || n.titulo.to_lowercase().contains(&buscar)
                || n.contenido.to_lowercase().contains(&buscar))
                && (categoria.is_empty() || n.categoria.to_lowercase().contains(&categoria))
        })
        .collect();
    let actual = notas
        .iter()
        .find(|n| n.id == editar)
        .cloned()
        .unwrap_or(Nota {
            id: 0,
            titulo: String::new(),
            contenido: String::new(),
            categoria: String::new(),
            fecha: String::new(),
            destacada: false,
        });
    let lista = notas.iter().map(tarjeta).collect::<String>();
    archivo("static/index.html")
        .replace("{{MENSAJE}}", &html(mensaje))
        .replace("{{BUSCAR}}", &html(campo(datos, "buscar")))
        .replace("{{FILTRO}}", &html(campo(datos, "categoria_filtro")))
        .replace("{{ID}}", &actual.id.to_string())
        .replace("{{TITULO}}", &html(&actual.titulo))
        .replace("{{CONTENIDO}}", &html(&actual.contenido))
        .replace("{{CATEGORIA}}", &html(&actual.categoria))
        .replace("{{FECHA}}", &actual.fecha)
        .replace(
            "{{ESTRELLA}}",
            if actual.destacada { "checked" } else { "" },
        )
        .replace("{{NOTAS}}", &lista)
        .replace("{{TOTAL}}", &notas.len().to_string())
}
fn tarjeta(n: &Nota) -> String {
    let palabras = n.contenido.split_whitespace().count();
    format!("<article class=\"nota\"><small>{} · {} · {} palabras {}</small><h3>{}</h3><p>{}</p><footer><a href=\"/?editar={}\">Editar</a><form action=\"/destacar\" method=\"post\"><input type=hidden name=id value={}><button>{}</button></form><form action=\"/eliminar\" method=\"post\"><input type=hidden name=id value={}><button class=eliminar>Eliminar</button></form></footer></article>",html(&n.categoria),html(&n.fecha),palabras,if n.destacada{"★"}else{""},html(&n.titulo),html(&n.contenido).replace('\n',"<br>"),n.id,n.id,if n.destacada{"Quitar estrella"}else{"Destacar"},n.id)
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
    t.replace('\\', "\\\\")
        .replace('|', "\\p")
        .replace('\n', "\\n")
}
fn dec(t: &str) -> String {
    let mut r = String::new();
    let mut e = false;
    for c in t.chars() {
        if e {
            r.push(match c {
                'n' => '\n',
                'p' => '|',
                x => x,
            });
            e = false
        } else if c == '\\' {
            e = true
        } else {
            r.push(c)
        }
    }
    r
}
fn tipo(t: &str) -> Header {
    Header::from_bytes("Content-Type", format!("{t}; charset=utf-8")).unwrap()
}
