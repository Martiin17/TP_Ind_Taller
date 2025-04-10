//! Contiene la logica de DUP, DROP, SWAP, OVER, ROT
use crate::{devolucion::Devolucion, stack::Stack};

use super::matchear_devolucion_numero;

/// Toma el ultimo elemento del stack y los duplica
pub fn ejecutar_dup(stack: &mut Stack) -> Result<Devolucion, String> {
    let a = stack.pop()?;
    let a = matchear_devolucion_numero(a)?;
    stack.push(a)?;
    stack.push(a)?;
    Ok(Devolucion::Vacio)
}

/// Toma el ultimo elemento del stack y lo elimina
pub fn ejecutar_drop(stack: &mut Stack) -> Result<Devolucion, String> {
    stack.pop()?;
    Ok(Devolucion::Vacio)
}

/// Toma los 2 ultimos elementos del stack y los cambia de lugar entre si
pub fn ejecutar_swap(stack: &mut Stack) -> Result<Devolucion, String> {
    let a = stack.pop()?;
    let b = stack.pop()?;
    let a = matchear_devolucion_numero(a)?;
    let b = matchear_devolucion_numero(b)?;
    stack.push(a)?;
    stack.push(b)?;
    Ok(Devolucion::Vacio)
}

/// Toma los 2 ultimos elementos del stack y duplica el 2do elemento
pub fn ejecutar_over(stack: &mut Stack) -> Result<Devolucion, String> {
    let a = stack.pop()?;
    let b = stack.pop()?;
    let a = matchear_devolucion_numero(a)?;
    let b = matchear_devolucion_numero(b)?;
    stack.push(b)?;
    stack.push(a)?;
    stack.push(b)?;
    Ok(Devolucion::Vacio)
}

/// Toma los 3 ultimos elementos del stack y mueve todos los elementos en el siguiente orden
/// 
/// {a,b,c} --> {b,a,c}
pub fn ejecutar_rot(stack: &mut Stack) -> Result<Devolucion, String> {
    let a = stack.pop()?;
    let b = stack.pop()?;
    let c = stack.pop()?;
    let a = matchear_devolucion_numero(a)?;
    let b = matchear_devolucion_numero(b)?;
    let c = matchear_devolucion_numero(c)?;
    stack.push(b)?;
    stack.push(a)?;
    stack.push(c)?;
    Ok(Devolucion::Vacio)
}

/// Toma un i16 y lo pushea al stack
pub fn ejecutar_int(stack: &mut Stack, nro: i16) -> Result<Devolucion, String> {
    stack.push(nro)?;
    Ok(Devolucion::Vacio)
}
