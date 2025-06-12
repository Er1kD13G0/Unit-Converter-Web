use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Conversão não suportada: de {0} para {1}")]
    ConversaoNaoSuportada(String, String),
}

pub fn converter_distancia(valor: f64, de: &str, para: &str) -> Result<f64, Error> {
    match (de, para) {
        ("Milimetro", "Centimetro") => Ok(valor / 10.0),
        ("Milimetro", "Metro") => Ok(valor / 1000.0),
        ("Milimetro", "Quilometro") => Ok(valor / 1_000_000.0),
        ("Centimetro", "Milimetro") => Ok(valor * 10.0),
        ("Centimetro", "Metro") => Ok(valor * 100.0),
        ("Centimetro", "Quilometro") => Ok(valor * 100_000.0),
        ("Metro", "Milimetro") => Ok(valor * 1000.0),
        ("Metro", "Centimetro") => Ok(valor * 100.0),
        ("Metro", "Quilometro") => Ok(valor / 1000.0),
        ("Metro", "Polegada") => Ok(valor * 39.3701),
        ("Metro", "Pe") => Ok(valor * 3.28084),
        ("Metro", "Jarda") => Ok(valor * 1.09361),
        ("Quilometro", "Metro") => Ok(valor * 1000.0),
        ("Quilometro", "Milha") => Ok(valor / 1.60934),
        ("Polegada", "Metro") => Ok(valor / 39.3701),
        ("Pe", "Metro") => Ok(valor / 3.28084),
        ("Jarda", "Metro") => Ok(valor / 1.09361),
        ("Milha", "Quilometro") => Ok(valor * 1.60934),
        _ => Err(Error::ConversaoNaoSuportada(
            de.to_string(),
            para.to_string(),
        )),
    }
}

pub fn converter_peso(valor: f64, de: &str, para: &str) -> Result<f64, Error> {
    match (de, para) {
        ("Miligrama", "Grama") => Ok(valor / 1000.0),
        ("Grama", "Kilograma") => Ok(valor / 1000.0),
        ("Kilograma", "Grama") => Ok(valor * 1000.0),
        ("Grama", "Miligrama") => Ok(valor * 1000.0),
        ("Kilograma", "Libra") => Ok(valor * 2.20462),
        ("Libra", "Kilograma") => Ok(valor / 2.20462),
        ("Grama", "Onça") => Ok(valor / 28.34955),
        ("Onça", "Grama") => Ok(valor * 28.3495),
        _ => Err(Error::ConversaoNaoSuportada(
            de.to_string(),
            para.to_string(),
        )),
    }
}

pub fn converter_temperatura(valor: f64, de: &str, para: &str) -> Result<f64, Error> {
    match (de, para) {
        ("Celsius", "Fahrenheit") => Ok((valor * 9.0 / 5.0) + 32.0),
        ("Celsius", "Kelvin") => Ok(valor + 273.15),
        ("Fahrenheit", "Celsius") => Ok((valor - 32.0) * 5.0 / 9.0),
        ("Fahrenheit", "Kelvin") => Ok((valor - 32.0) * 5.0 / 9.0 + 273.15),
        ("Kelvin", "Celsius") => Ok(valor - 273.15),
        ("Kelvin", "Fahrenheit") => Ok((valor - 273.15) * 9.0 / 5.0 + 32.0),
        _ => Err(Error::ConversaoNaoSuportada(
            de.to_string(),
            para.to_string(),
        )),
    }
}
