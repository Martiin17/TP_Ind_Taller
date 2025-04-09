use crate::{devolucion::Devolucion, stack::Stack};

use super::matchear_devolucion_numero;

pub fn ejecutar_suma(stack: &mut Stack) -> Result<Devolucion, String> {
    let a = stack.pop()?;
    let a = matchear_devolucion_numero(a)?;
    let b = stack.pop()?;
    let b = matchear_devolucion_numero(b)?;
    stack.push(a + b)?;
    Ok(Devolucion::Vacio)
}

pub fn ejecutar_resta(stack: &mut Stack) -> Result<Devolucion, String> {
    let a = stack.pop()?;
    let a = matchear_devolucion_numero(a)?;
    let b = stack.pop()?;
    let b = matchear_devolucion_numero(b)?;
    stack.push(b - a)?;
    Ok(Devolucion::Vacio)
}

pub fn ejecutar_multiplicacion(stack: &mut Stack) -> Result<Devolucion, String> {
    let a = stack.pop()?;
    let a = matchear_devolucion_numero(a)?;
    let b = stack.pop()?;
    let b = matchear_devolucion_numero(b)?;
    stack.push(a * b)?;
    Ok(Devolucion::Vacio)
}

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
