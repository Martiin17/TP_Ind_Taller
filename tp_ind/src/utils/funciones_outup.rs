//! Contiene la logica de ., CR, EMIT, ." "
use crate::{devolucion::Devolucion, stack::Stack, utils::matchear_devolucion_numero};

/// Quita el ultimo elemento del stack y lo imprime
/// 
pub fn ejecutar_punto(stack: &mut Stack) -> Result<Devolucion, String> {
    let numero = stack.pop()?;
    let numero = matchear_devolucion_numero(numero)?;
    print!("{} ", numero);
    Ok(Devolucion::Vacio)
}

/// Imprime un salto de linea
///
pub fn ejecutar_cr(stack: &mut Stack) -> Result<Devolucion, String> {
    let _ = stack;
    println!("");
    Ok(Devolucion::Vacio)
}

/// Quita el ultimo elemento del stack y lo transforma en su representacion ASCII
pub fn ejecutar_emit(stack: &mut Stack) -> Result<Devolucion, String> {
    let numero = stack.pop()?;
    let numero = matchear_devolucion_numero(numero)?;
    if (0..=255).contains(&numero) {
        println!("{}", (numero as u8 as char));
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
pub fn ejecutar_punto_y_coma(stack: &mut Stack, texto: &String) -> Result<Devolucion, String> {
    let _ = stack;
    println!("{}", texto);
    Ok(Devolucion::Vacio)
}
