use crate::{devolucion::Devolucion, stack::Stack};

use super::matchear_devolucion_numero;

pub fn ejecutar_dup(stack: &mut Stack) -> Result<Devolucion, String> {
    let a = stack.pop()?;
    let a = matchear_devolucion_numero(a)
        .ok_or(String::from("No se pudo extraer el numero del stack"))?;
    stack.push(a)?;
    stack.push(a)?;
    Ok(Devolucion::Vacio)
}

//REVISAR
pub fn ejecutar_drop(stack: &mut Stack) -> Result<Devolucion, String> {
    //No devuelve el nro, ver si causa problemas
    stack.pop()?;
    Ok(Devolucion::Vacio)
}

pub fn ejecutar_swap(stack: &mut Stack) -> Result<Devolucion, String> {
    let a = stack.pop()?;
    let b = stack.pop()?;
    let a = matchear_devolucion_numero(a)
        .ok_or(String::from("No se pudo extraer el numero del stack"))?;
    let b = matchear_devolucion_numero(b)
        .ok_or(String::from("No se pudo extraer el numero del stack"))?;
    stack.push(a)?;
    stack.push(b)?;
    Ok(Devolucion::Vacio)
}

pub fn ejecutar_over(stack: &mut Stack) -> Result<Devolucion, String> {
    let a = stack.pop()?;
    let b = stack.pop()?;
    let a = matchear_devolucion_numero(a)
        .ok_or(String::from("No se pudo extraer el numero del stack"))?;
    let b = matchear_devolucion_numero(b)
        .ok_or(String::from("No se pudo extraer el numero del stack"))?;
    stack.push(b)?;
    stack.push(a)?;
    stack.push(b)?;
    Ok(Devolucion::Vacio)
}

pub fn ejecutar_rot(stack: &mut Stack) -> Result<Devolucion, String> {
    let a = stack.pop()?;
    let b = stack.pop()?;
    let c = stack.pop()?;
    let a = matchear_devolucion_numero(a)
        .ok_or(String::from("No se pudo extraer el numero del stack"))?;
    let b = matchear_devolucion_numero(b)
        .ok_or(String::from("No se pudo extraer el numero del stack"))?;
    let c = matchear_devolucion_numero(c)
        .ok_or(String::from("No se pudo extraer el numero del stack"))?;
    stack.push(b)?;
    stack.push(a)?;
    stack.push(c)?;
    Ok(Devolucion::Vacio)
}

pub fn ejecutar_int(stack: &mut Stack, nro: i16) -> Result<Devolucion, String> {
    stack.push(nro)?;
    Ok(Devolucion::Vacio)
}
