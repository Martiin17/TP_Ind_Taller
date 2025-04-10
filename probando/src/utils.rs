//! Este modulo contiene utils.
use std::fs::File;
use std::io::{self, Write, Read};
/// # Parametros
/// - `a`: primer número.
/// - `b`: segundo número.
///
/// # Retorna
/// Vector con suma, div, mul y resta.

pub fn operar(a: &i16, b:&i16) -> Result<Vec<i16>, String> {
    if *b == 0{
        return Err(String::from("division-by-zero"));
    }
    let resultados = vec![a+b,a-b,a/b,a*b];

    Ok(resultados)
}

/// Suma dos números.
///
/// # Parámetros
/// - `a`: primer número.
/// - `b`: segundo número.
///
/// # Retorna
/// La suma de `a` y `b`.
pub fn escribir_resultados(vec: &Vec<i16>) -> io::Result<()> {
    let mut archivo = File::create("resultados_esperados.fth")?;

    for (i, valor) in vec.iter().enumerate() {
        if i > 0 {
            write!(archivo, " ")?; // Agrega un espacio antes del número (excepto el primero)
        }
        write!(archivo, "{}", valor)?;
    }

    Ok(())
}

pub fn leer_resultados() -> io::Result<Vec<i16>> {
    let mut archivo = File::open("resultados_esperados.fth")?;
    let mut contenido = String::new();
    archivo.read_to_string(&mut contenido)?;

    let leido = contenido
        .split_whitespace()
        .filter_map(|s| s.parse::<i16>().ok())
        .collect::<Vec<i16>>();

    
    Ok(leido)
}