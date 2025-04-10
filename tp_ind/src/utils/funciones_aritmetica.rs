//! Contiene la logica de +, -, *, /
use crate::{devolucion::Devolucion, stack::Stack};

use super::matchear_devolucion_numero;
/// Toma los 2 ultimos elementos del stack y devuelve su suma
///
/// #Example
///
/// ```
/// let mut stack = Stack::new(1000);
/// stack.push(5)?;
/// stack.push(7)?;
/// ejecutar_suma(&mut stack)?;
///
/// assert_eq!(stack.vector, vec![12]);
/// ```
///
pub fn ejecutar_suma(stack: &mut Stack) -> Result<Devolucion, String> {
    let a = stack.pop()?;
    let a = matchear_devolucion_numero(a)?;
    let b = stack.pop()?;
    let b = matchear_devolucion_numero(b)?;
    stack.push(a + b)?;
    Ok(Devolucion::Vacio)
}

/// Toma los 2 ultimos elementos del stack y devuelve su resta
///
/// #Example
///
/// ```
/// let mut stack = Stack::new(1000);
/// stack.push(5)?;
/// stack.push(7)?;
/// ejecutar_resta(&mut stack)?;
///
/// assert_eq!(stack.vector, vec![-2]);
/// ```
///
pub fn ejecutar_resta(stack: &mut Stack) -> Result<Devolucion, String> {
    let a = stack.pop()?;
    let a = matchear_devolucion_numero(a)?;
    let b = stack.pop()?;
    let b = matchear_devolucion_numero(b)?;
    stack.push(b - a)?;
    Ok(Devolucion::Vacio)
}

/// Toma los 2 ultimos elementos del stack y devuelve su multipĺicacion
///
/// #Example
///
/// ```
/// let mut stack = Stack::new(1000);
/// stack.push(5)?;
/// stack.push(7)?;
/// ejecutar_multiplicacion(&mut stack)?;
///
/// assert_eq!(stack.vector, vec![35]);
/// ```
///
pub fn ejecutar_multiplicacion(stack: &mut Stack) -> Result<Devolucion, String> {
    let a = stack.pop()?;
    let a = matchear_devolucion_numero(a)?;
    let b = stack.pop()?;
    let b = matchear_devolucion_numero(b)?;
    stack.push(a * b)?;
    Ok(Devolucion::Vacio)
}

/// Toma los 2 ultimos elementos del stack y devuelve su division
///
/// #Example
///
/// ```
/// let mut stack = Stack::new(1000);
/// stack.push(5)?;
/// stack.push(7)?;
/// ejecutar_division(&mut stack)?;
///
/// assert_eq!(stack.vector, vec![0]);
/// ```
///
pub fn ejecutar_division(stack: &mut Stack) -> Result<Devolucion, String> {
    let a = stack.pop()?;
    let a = matchear_devolucion_numero(a)?;
    let b = stack.pop()?;
    let b = matchear_devolucion_numero(b)?;
    if b == 0 {
        return Err(String::from("division-by-zero"));
    } else {
        stack.push(b / a)?;
    }
    Ok(Devolucion::Vacio)
}
