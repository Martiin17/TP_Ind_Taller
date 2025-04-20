//! Contiene la logica de ., CR, EMIT, ." "
use crate::{devolucion::Devolucion, stack::Stack, utils::matchear_devolucion_numero};
use std::io::Write;

/// Quita el ultimo elemento del stack y lo imprime
///
pub fn ejecutar_punto<W: Write>(writer: &mut W, stack: &mut Stack) -> Result<Devolucion, String> {
    let numero = stack.pop()?;
    let numero = matchear_devolucion_numero(numero)?;
    write!(writer, "{} ", numero).map_err(|e| e.to_string())?;
    Ok(Devolucion::Vacio)
}

/// Imprime un salto de linea
///
pub fn ejecutar_cr<W: Write>(writer: &mut W, _stack: &mut Stack) -> Result<Devolucion, String> {
    writeln!(writer).map_err(|e| e.to_string())?;
    Ok(Devolucion::Vacio)
}

/// Quita el ultimo elemento del stack y lo transforma en su representacion ASCII
pub fn ejecutar_emit<W: Write>(writer: &mut W, stack: &mut Stack) -> Result<Devolucion, String> {
    let numero = stack.pop()?;
    let numero = matchear_devolucion_numero(numero)?;
    if (0..=255).contains(&numero) {
        write!(writer, "{} ", numero as u8 as char).map_err(|e| e.to_string())?;
    }
    Ok(Devolucion::Vacio)
}

/// Imprime el texto que hay entre ." "
///
/// #Example
///
/// " Hola mundo!"
///
/// Se imprime --> Hola mundo!
///
pub fn ejecutar_punto_y_coma<W: Write>(
    writer: &mut W,
    _stack: &mut Stack,
    texto: &str,
) -> Result<Devolucion, String> {
    write!(writer, "{} ", texto).map_err(|e| e.to_string())?;
    Ok(Devolucion::Vacio)
}
