use crate::{devolucion::Devolucion, stack::Stack};

use super::matchear_devolucion_numero;

pub fn ejecutar_suma(stack: &mut Stack) -> Result<Devolucion, String> {
    let a = stack.pop()?;
    let a = matchear_devolucion_numero(a)
        .ok_or(String::from("No se pudo extraer el numero del stack"))?;
    let b = stack.pop()?;
    let b = matchear_devolucion_numero(b)
        .ok_or(String::from("No se pudo extraer el numero del stack"))?;
    stack.push(a + b)?;
    Ok(Devolucion::Vacio)
}

pub fn ejecutar_resta(stack: &mut Stack) -> Result<Devolucion, String> {
    let a = stack.pop()?;
    let a = matchear_devolucion_numero(a)
        .ok_or(String::from("No se pudo extraer el numero del stack"))?;
    let b = stack.pop()?;
    let b = matchear_devolucion_numero(b)
        .ok_or(String::from("No se pudo extraer el numero del stack"))?;
    stack.push(b - a)?;
    Ok(Devolucion::Vacio)
}

pub fn ejecutar_multiplicacion(stack: &mut Stack) -> Result<Devolucion, String> {
    let a = stack.pop()?;
    let a = matchear_devolucion_numero(a)
        .ok_or(String::from("No se pudo extraer el numero del stack"))?;
    let b = stack.pop()?;
    let b = matchear_devolucion_numero(b)
        .ok_or(String::from("No se pudo extraer el numero del stack"))?;
    stack.push(a * b)?;
    Ok(Devolucion::Vacio)
}

pub fn ejecutar_division(stack: &mut Stack) -> Result<Devolucion, String> {
    let a = stack.pop()?;
    let a = matchear_devolucion_numero(a)
        .ok_or(String::from("No se pudo extraer el numero del stack"))?;
    let b = stack.pop()?;
    let b = matchear_devolucion_numero(b)
        .ok_or(String::from("No se pudo extraer el numero del stack"))?;
    if b == 0 {
        return Err(String::from("division-by-zero"));
    } else {
        stack.push(b / a)?;
    }
    Ok(Devolucion::Vacio)
}

/* pub fn matchear_devolucion_numero(posible_numero: Devolucion) -> Option<i16> {
    match posible_numero{
        Devolucion::Numero(nro) => Some(nro),
        Devolucion::Vacio => None,
    }
} */
