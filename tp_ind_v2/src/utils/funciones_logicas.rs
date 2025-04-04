use crate::{devolucion::Devolucion, stack::Stack};

use super::matchear_devolucion_numero;

pub fn ejecutar_igual(stack: &mut Stack) -> Result<Devolucion, String>{
    let a = stack.pop()?;
    let a = matchear_devolucion_numero(a)
        .ok_or(String::from("No se pudo extraer el numero del stack"))?;
    let b = stack.pop()?;
    let b = matchear_devolucion_numero(b)
        .ok_or(String::from("No se pudo extraer el numero del stack"))?;
    if a == b {
        stack.push(-1)?;
    } else {
        stack.push(0)?;
    }
    Ok(Devolucion::Vacio)
}

pub fn ejecutar_mayor(stack: &mut Stack) -> Result<Devolucion, String>{
    let a = stack.pop()?;
    let a = matchear_devolucion_numero(a)
        .ok_or(String::from("No se pudo extraer el numero del stack"))?;
    let b = stack.pop()?;
    let b = matchear_devolucion_numero(b)
        .ok_or(String::from("No se pudo extraer el numero del stack"))?;
    if b > a {
        stack.push(-1)?;
    } else {
        stack.push(0)?;
    }
    Ok(Devolucion::Vacio)
}

pub fn ejecutar_menor(stack: &mut Stack) -> Result<Devolucion, String>{
    let a = stack.pop()?;
    let a = matchear_devolucion_numero(a)
        .ok_or(String::from("No se pudo extraer el numero del stack"))?;
    let b = stack.pop()?;
    let b = matchear_devolucion_numero(b)
        .ok_or(String::from("No se pudo extraer el numero del stack"))?;
    if b < a {
        stack.push(-1)?;
    } else {
        stack.push(0)?;
    }
    Ok(Devolucion::Vacio)
}

pub fn ejecutar_and(stack: &mut Stack) -> Result<Devolucion, String>{
    let a = stack.pop()?;
    let a = matchear_devolucion_numero(a)
        .ok_or(String::from("No se pudo extraer el numero del stack"))?;
    let b = stack.pop()?;
    let b = matchear_devolucion_numero(b)
        .ok_or(String::from("No se pudo extraer el numero del stack"))?;
    stack.push(b & a)?;
    Ok(Devolucion::Vacio)
}

pub fn ejecutar_or(stack: &mut Stack) -> Result<Devolucion, String>{
    let a = stack.pop()?;
    let a = matchear_devolucion_numero(a)
        .ok_or(String::from("No se pudo extraer el numero del stack"))?;
    let b = stack.pop()?;
    let b = matchear_devolucion_numero(b)
        .ok_or(String::from("No se pudo extraer el numero del stack"))?;
    stack.push(b | a)?;
    Ok(Devolucion::Vacio)
}

pub fn ejecutar_not(stack: &mut Stack) -> Result<Devolucion, String>{
    let a = stack.pop()?;
    let a = matchear_devolucion_numero(a)
        .ok_or(String::from("No se pudo extraer el numero del stack"))?;
    if a != 0 {
        stack.push(-1)?;
    } else {
        stack.push(0)?;
    }
    Ok(Devolucion::Vacio)
}