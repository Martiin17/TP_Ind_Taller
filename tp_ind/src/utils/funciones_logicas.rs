//! Contiene la logica de =, <, >, AND, OR, NOT
use crate::{devolucion::Devolucion, stack::Stack};

use super::matchear_devolucion_numero;

/// Toma los 2 ultimos elementos del stack y los compara
///
/// Si son iguales pushea verdadero (-1), en caso contrario pushea (0)
pub fn ejecutar_igual(stack: &mut Stack) -> Result<Devolucion, String> {
    let a = stack.pop()?;
    let a = matchear_devolucion_numero(a)?;
    let b = stack.pop()?;
    let b = matchear_devolucion_numero(b)?;
    if a == b {
        stack.push(-1)?;
    } else {
        stack.push(0)?;
    }
    Ok(Devolucion::Vacio)
}

/// Toma los 2 ultimos elementos del stack y los compara
///
/// Siendo a el primer elemento que sale del stack y b el segundo
///
/// Se realiza b > a
///
/// Si la condicion es verdadera se pushea (-1), en caso contrario pushea (0)
pub fn ejecutar_mayor(stack: &mut Stack) -> Result<Devolucion, String> {
    let a = stack.pop()?;
    let a = matchear_devolucion_numero(a)?;
    let b = stack.pop()?;
    let b = matchear_devolucion_numero(b)?;
    if b > a {
        stack.push(-1)?;
    } else {
        stack.push(0)?;
    }
    Ok(Devolucion::Vacio)
}

/// Toma los 2 ultimos elementos del stack y los compara
///
/// Siendo a el primer elemento que sale del stack y b el segundo
///
/// Se realiza b < a
///
/// Si la condicion es verdadera se pushea (-1), en caso contrario pushea (0)
pub fn ejecutar_menor(stack: &mut Stack) -> Result<Devolucion, String> {
    let a = stack.pop()?;
    let a = matchear_devolucion_numero(a)?;
    let b = stack.pop()?;
    let b = matchear_devolucion_numero(b)?;
    if b < a {
        stack.push(-1)?;
    } else {
        stack.push(0)?;
    }
    Ok(Devolucion::Vacio)
}

/// Toma los 2 ultimos elementos del stack y se realiza un and logico bit a bit
///
/// Si la condicion es verdadera se pushea (-1), en caso contrario pushea (0)
pub fn ejecutar_and(stack: &mut Stack) -> Result<Devolucion, String> {
    let a = stack.pop()?;
    let a = matchear_devolucion_numero(a)?;
    let b = stack.pop()?;
    let b = matchear_devolucion_numero(b)?;
    stack.push(b & a)?;
    Ok(Devolucion::Vacio)
}

/// Toma los 2 ultimos elementos del stack y se realiza un or logico inclusivo bit a bit
///
/// Si la condicion es verdadera se pushea (-1), en caso contrario pushea (0)
pub fn ejecutar_or(stack: &mut Stack) -> Result<Devolucion, String> {
    let a = stack.pop()?;
    let a = matchear_devolucion_numero(a)?;
    let b = stack.pop()?;
    let b = matchear_devolucion_numero(b)?;
    stack.push(b | a)?;
    Ok(Devolucion::Vacio)
}

/// Toma el ultimo elemento del stack e invierte su valor logico
///
/// #Example
///
/// -1 --> 0
///
/// Numero distinto de -1 --> -1
///
/// Se pushea el valor invertido
pub fn ejecutar_not(stack: &mut Stack) -> Result<Devolucion, String> {
    let a = stack.pop()?;
    let a = matchear_devolucion_numero(a)?;
    if a == 0 {
        stack.push(-1)?;
    } else {
        stack.push(0)?;
    }
    Ok(Devolucion::Vacio)
}
