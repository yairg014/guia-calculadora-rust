//! Módulo 2: cálculo normal y científico.
//! Parser es una estructura que lee la expresión en orden y respeta la prioridad matemática.

pub fn calcular(expresion: &str) -> Result<String, String> {
    let texto = expresion
        .trim()
        .replace('×', "*")
        .replace('÷', "/")
        .replace(',', ".");

    if texto.is_empty() {
        return Err("Escribe una operación.".to_string());
    }
    if texto.len() > 120 || !texto.chars().all(caracter_permitido) {
        return Err("Solo se permiten números, operadores, paréntesis y funciones válidas.".to_string());
    }

    let mut parser = Parser::nuevo(&texto);
    let valor = parser.expresion()?;
    parser.espacios();

    if parser.posicion != parser.texto.len() {
        return Err("Revisa la operación.".to_string());
    }
    if !valor.is_finite() {
        return Err("El resultado no es válido.".to_string());
    }
    Ok(formatear_numero(valor))
}

fn caracter_permitido(caracter: char) -> bool {
    caracter.is_ascii_alphanumeric() || "+-*/^(). ".contains(caracter)
}

struct Parser {
    texto: Vec<char>,
    posicion: usize,
}

impl Parser {
    fn nuevo(texto: &str) -> Self {
        Self { texto: texto.chars().collect(), posicion: 0 }
    }

    fn espacios(&mut self) {
        while self.texto.get(self.posicion) == Some(&' ') {
            self.posicion += 1;
        }
    }

    // Nivel 1: suma y resta.
    fn expresion(&mut self) -> Result<f64, String> {
        let mut valor = self.termino()?;
        loop {
            self.espacios();
            match self.texto.get(self.posicion) {
                Some('+') => { self.posicion += 1; valor += self.termino()?; }
                Some('-') => { self.posicion += 1; valor -= self.termino()?; }
                _ => return Ok(valor),
            }
        }
    }

    // Nivel 2: multiplicación y división.
    fn termino(&mut self) -> Result<f64, String> {
        let mut valor = self.potencia()?;
        loop {
            self.espacios();
            match self.texto.get(self.posicion) {
                Some('*') => { self.posicion += 1; valor *= self.potencia()?; }
                Some('/') => {
                    self.posicion += 1;
                    let divisor = self.potencia()?;
                    if divisor == 0.0 {
                        return Err("No se puede dividir entre cero.".to_string());
                    }
                    valor /= divisor;
                }
                _ => return Ok(valor),
            }
        }
    }

    // Nivel 3: potencias. Ejemplo: 2^3^2 se procesa de derecha a izquierda.
    fn potencia(&mut self) -> Result<f64, String> {
        let mut valor = self.valor()?;
        self.espacios();
        if self.texto.get(self.posicion) == Some(&'^') {
            self.posicion += 1;
            valor = valor.powf(self.potencia()?);
        }
        Ok(valor)
    }

    // Nivel 4: números, paréntesis y funciones.
    fn valor(&mut self) -> Result<f64, String> {
        self.espacios();
        if self.texto.get(self.posicion) == Some(&'-') {
            self.posicion += 1;
            return Ok(-self.valor()?);
        }
        if self.texto.get(self.posicion) == Some(&'(') {
            self.posicion += 1;
            let valor = self.expresion()?;
            self.espacios();
            if self.texto.get(self.posicion) != Some(&')') {
                return Err("Falta cerrar un paréntesis.".to_string());
            }
            self.posicion += 1;
            return Ok(valor);
        }

        let inicio = self.posicion;
        while self.texto.get(self.posicion).is_some_and(|c| c.is_ascii_alphabetic()) {
            self.posicion += 1;
        }
        if inicio != self.posicion {
            return self.funcion(inicio);
        }

        let inicio = self.posicion;
        while self.texto.get(self.posicion).is_some_and(|c| c.is_ascii_digit() || *c == '.') {
            self.posicion += 1;
        }
        self.texto[inicio..self.posicion]
            .iter()
            .collect::<String>()
            .parse()
            .map_err(|_| "Número inválido.".to_string())
    }

    fn funcion(&mut self, inicio: usize) -> Result<f64, String> {
        let nombre: String = self.texto[inicio..self.posicion].iter().collect();
        if nombre == "pi" {
            return Ok(std::f64::consts::PI);
        }
        self.espacios();
        if self.texto.get(self.posicion) != Some(&'(') {
            return Err("Función inválida.".to_string());
        }
        self.posicion += 1;
        let numero = self.expresion()?;
        self.espacios();
        if self.texto.get(self.posicion) != Some(&')') {
            return Err("Falta cerrar la función.".to_string());
        }
        self.posicion += 1;

        match nombre.as_str() {
            "sqrt" if numero >= 0.0 => Ok(numero.sqrt()),
            "sqrt" => Err("No existe raíz real de un número negativo.".to_string()),
            "sin" => Ok(numero.to_radians().sin()),
            "cos" => Ok(numero.to_radians().cos()),
            "tan" => Ok(numero.to_radians().tan()),
            "ln" if numero > 0.0 => Ok(numero.ln()),
            "ln" => Err("ln necesita un número mayor que cero.".to_string()),
            "log" if numero > 0.0 => Ok(numero.log10()),
            "log" => Err("log necesita un número mayor que cero.".to_string()),
            _ => Err("Función no disponible.".to_string()),
        }
    }
}

pub fn formatear_numero(valor: f64) -> String {
    let texto = format!("{valor:.8}");
    texto.trim_end_matches('0').trim_end_matches('.').to_string()
}

#[cfg(test)]
mod pruebas {
    use super::*;

    #[test]
    fn respeta_la_prioridad_de_operaciones() {
        assert_eq!(calcular("2 + 3 * 4"), Ok("14".to_string()));
        assert_eq!(calcular("(2 + 3) * 4"), Ok("20".to_string()));
    }

    #[test]
    fn calcula_funciones_cientificas() {
        assert_eq!(calcular("sqrt(81) + sin(30)"), Ok("9.5".to_string()));
    }

    #[test]
    fn controla_los_errores_de_usuario() {
        assert!(calcular("8 / 0").is_err());
        assert!(calcular("sqrt(-1)").is_err());
        assert!(calcular("sin(30").is_err());
        assert!(calcular("2 @ 3").is_err());
    }
}
