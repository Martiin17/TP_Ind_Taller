use crate::{forth::Forth, stack::Stack};

pub fn ejecutar_if(stack: &mut Stack, texto: &String, forth: &Forth) -> Result<(), String> {
    let valor_bool = stack.pop()?;
    if texto.contains("ELSE"){
        if valor_bool == -1 {
            ejecutar_condicion(stack, texto, forth, String::from("ELSE"))?;
            return Ok(());
        } else{
            ejecutar_condicion(stack, texto, forth, String::from("THEN"))?;
            return Ok(());
        }
    }else{ //Es IF..THEN
        if valor_bool == -1 {
            ejecutar_condicion(stack, texto, forth, String::from("THEN"))?;
            return Ok(());
        }
    }
    Err("error al ejecutar clausula if".to_string())   
}

fn ejecutar_condicion(stack: &mut Stack, texto: &String, forth: &Forth, palabra_freno: String) -> Result<(), String> {
    for palabra in texto.split_whitespace() {
        if palabra != "IF"{
            if palabra == palabra_freno{
                return Ok(());
            }
            let palabra_formateada = palabra.to_string();
            //println!("{:?}", forth.diccionario_usuario);
            forth.ejecutar(&palabra_formateada, stack)?;
        }
    }
    Ok(())
}