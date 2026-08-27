//! Módulo 3: solución de ecuaciones lineales sencillas de una variable x.
//! Formatos admitidos: 2x - 5 = 11, x/2 + 7 = 12 y 3x + 2 = x + 10.

use crate::calculadora::formatear_numero;

pub fn resolver(ecuacion: &str) -> String {
    let texto = ecuacion.trim().replace(' ', "").replace('X', "x").replace('−', "-");
    if texto.matches('=').count() != 1 || texto.len() > 100 {
        return "Formato no válido. Ejemplo: 2x - 5 = 11".to_string();
    }

    let partes: Vec<&str> = texto.split('=').collect();
    let izquierda = lado_lineal(partes[0]);
    let derecha = lado_lineal(partes[1]);

    match (izquierda, derecha) {
        (Ok((a, b)), Ok((c, d))) if casi_cero(a - c) && casi_cero(b - d) => "Infinitas soluciones.".to_string(),
        (Ok((a, _)), Ok((c, _))) if casi_cero(a - c) => "La ecuación no tiene solución.".to_string(),
        (Ok((a, b)), Ok((c, d))) => format!("x = {}", formatear_numero((d - b) / (a - c))),
        _ => "Formato no válido. Usa términos como 2x, x/2, +3 o -5.".to_string(),
    }
}

/// Devuelve (coeficiente de x, número fijo). Ejemplo: 2x - 5 se convierte en (2, -5).
fn lado_lineal(lado: &str) -> Result<(f64, f64), String> {
    if lado.is_empty() || !lado.chars().all(|c| c.is_ascii_digit() || "x+-*/.".contains(c)) {
        return Err("Caracteres no válidos".to_string());
    }

    let lado = lado.replace('-', "+-");
    let mut coeficiente_x = 0.0;
    let mut termino_fijo = 0.0;

    for termino in lado.split('+').filter(|termino| !termino.is_empty()) {
        if termino.contains('x') {
            if termino.matches('x').count() != 1 {
                return Err("No es lineal".to_string());
            }
            coeficiente_x += coeficiente(termino)?;
        } else {
            termino_fijo += termino.parse::<f64>().map_err(|_| "Número inválido".to_string())?;
        }
    }
    Ok((coeficiente_x, termino_fijo))
}

fn coeficiente(termino: &str) -> Result<f64, String> {
    let texto = termino.replace('*', "").replace('x', "");
    if texto.is_empty() || texto == "+" { return Ok(1.0); }
    if texto == "-" { return Ok(-1.0); }

    if let Some((numerador, denominador)) = texto.split_once('/') {
        let numerador = if numerador.is_empty() || numerador == "+" { 1.0 }
            else if numerador == "-" { -1.0 }
            else { numerador.parse().map_err(|_| "Número inválido".to_string())? };
        let denominador: f64 = denominador.parse().map_err(|_| "Número inválido".to_string())?;
        if casi_cero(denominador) { return Err("División entre cero".to_string()); }
        return Ok(numerador / denominador);
    }
    texto.parse().map_err(|_| "Número inválido".to_string())
}

fn casi_cero(numero: f64) -> bool {
    numero.abs() < 0.0000001
}

#[cfg(test)]
mod pruebas {
    use super::*;

    #[test]
    fn resuelve_ecuaciones_con_una_x() {
        assert_eq!(resolver("2x - 5 = 11"), "x = 8");
        assert_eq!(resolver("x/2 + 7 = 12"), "x = 10");
        assert_eq!(resolver("3x + 2 = x + 10"), "x = 4");
    }

    #[test]
    fn detecta_casos_especiales_y_formato_incorrecto() {
        assert_eq!(resolver("2x + 3 = 2x + 3"), "Infinitas soluciones.");
        assert_eq!(resolver("2x + 3 = 2x + 4"), "La ecuación no tiene solución.");
        assert!(resolver("x*x = 9").contains("Formato no válido"));
        assert!(resolver("x/0 = 2").contains("Formato no válido"));
    }
}
