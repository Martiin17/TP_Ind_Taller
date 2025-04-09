use crate::{devolucion::Devolucion, stack::Stack};

use super::matchear_devolucion_numero;

pub fn ejecutar_igual(stack: &mut Stack) -> Result<Devolucion, String>{
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

pub fn ejecutar_mayor(stack: &mut Stack) -> Result<Devolucion, String>{
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

pub fn ejecutar_menor(stack: &mut Stack) -> Result<Devolucion, String>{
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

pub fn ejecutar_and(stack: &mut Stack) -> Result<Devolucion, String>{
    let a = stack.pop()?;
    let a = matchear_devolucion_numero(a)?;
    let b = stack.pop()?;
    let b = matchear_devolucion_numero(b)?;
    stack.push(b & a)?;
    Ok(Devolucion::Vacio)
}

pub fn ejecutar_or(stack: &mut Stack) -> Result<Devolucion, String>{
    let a = stack.pop()?;
    let a = matchear_devolucion_numero(a)?;
    let b = stack.pop()?;
    let b = matchear_devolucion_numero(b)?;
    stack.push(b | a)?;
    Ok(Devolucion::Vacio)
}

pub fn ejecutar_not(stack: &mut Stack) -> Result<Devolucion, String>{
    let a = stack.pop()?;
    let a = matchear_devolucion_numero(a)?;
    if a != 0 {
        stack.push(-1)?;
    } else {
        stack.push(0)?;
    }
    Ok(Devolucion::Vacio)
}