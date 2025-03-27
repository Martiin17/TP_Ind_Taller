use crate::stack::Stack;

pub fn ejecutar_punto(stack: &mut Stack) -> Result<(), String> {
    let item = stack.pop()?;
    print!("{} ", item);
    Ok(())
}

pub fn ejecutar_cr(stack: &mut Stack) -> Result<(), String> {
    //let _ = stack; saca el warning
    println!("");
    Ok(())
}

pub fn ejecutar_emit(stack: &mut Stack) -> Result<(), String> {
    let item = stack.pop()?;
    if item >= 0 && item <= 255{
        println!("{}", (item as u8 as char).to_string());
    }
    Ok(())
}

pub fn ejecutar_punto_y_coma(stack: &mut Stack, texto: &String) -> Result<(), String> {
    //let _ = stack; saca el warning
    println!("{}", texto);
    Ok(())
}