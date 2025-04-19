//! Contiene la logica de +, -, *, /
use crate::{devolucion::Devolucion, stack::Stack};

use super::matchear_devolucion_numero;
/// Toma los 2 ultimos elementos del stack y devuelve su suma
pub fn ejecutar_suma(stack: &mut Stack) -> Result<Devolucion, String> {
    let a = stack.pop()?;
    let a = matchear_devolucion_numero(a)?;
    let b = stack.pop()?;
    let b = matchear_devolucion_numero(b)?;
    stack.push(a + b)?;
    Ok(Devolucion::Vacio)
}

/// Toma los 2 ultimos elementos del stack y devuelve su resta
pub fn ejecutar_resta(stack: &mut Stack) -> Result<Devolucion, String> {
    let a = stack.pop()?;
    let a = matchear_devolucion_numero(a)?;
    let b = stack.pop()?;
    let b = matchear_devolucion_numero(b)?;
    stack.push(b - a)?;
    Ok(Devolucion::Vacio)
}

/// Toma los 2 ultimos elementos del stack y devuelve su multipĺicacion
pub fn ejecutar_multiplicacion(stack: &mut Stack) -> Result<Devolucion, String> {
    let a = stack.pop()?;
    let a = matchear_devolucion_numero(a)?;
    let b = stack.pop()?;
    let b = matchear_devolucion_numero(b)?;
    stack.push(a * b)?;
    Ok(Devolucion::Vacio)
}

/// Toma los 2 ultimos elementos del stack y devuelve su division
pub fn ejecutar_division(stack: &mut Stack) -> Result<Devolucion, String> {
    let a = stack.pop()?;
    let a = matchear_devolucion_numero(a)?;
    let b = stack.pop()?;
    let b = matchear_devolucion_numero(b)?;
    if a == 0 {
        return Err(String::from("division-by-zero\n"));
    } else {
        stack.push(b / a)?;
    }
    Ok(Devolucion::Vacio)
}
