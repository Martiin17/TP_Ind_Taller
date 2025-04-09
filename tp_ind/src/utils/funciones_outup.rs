use crate::{devolucion::Devolucion, stack::Stack, utils::matchear_devolucion_numero};

pub fn ejecutar_punto(stack: &mut Stack) -> Result<Devolucion, String> {
    let numero = stack.pop()?;
    let numero = matchear_devolucion_numero(numero)?;
    print!("{} ", numero);
    Ok(Devolucion::Vacio)
}

pub fn ejecutar_cr(stack: &mut Stack) -> Result<Devolucion, String> {
    let _ = stack;
    println!("");
    Ok(Devolucion::Vacio)
}

pub fn ejecutar_emit(stack: &mut Stack) -> Result<Devolucion, String> {
    let numero = stack.pop()?;
    let numero = matchear_devolucion_numero(numero)?;
    if (0..=255).contains(&numero) {
        println!("{}", (numero as u8 as char));
    }
    Ok(Devolucion::Vacio)
}

pub fn ejecutar_punto_y_coma(stack: &mut Stack, texto: &String) -> Result<Devolucion, String> {
    let _ = stack;
    println!("{}", texto);
    Ok(Devolucion::Vacio)
}
