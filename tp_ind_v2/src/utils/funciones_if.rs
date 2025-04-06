/* use crate::{devolucion::Devolucion, interprete::{self, Interprete}, stack::Stack};

use super::matchear_devolucion_numero;

pub fn ejecutar_if(stack: &mut Stack, texto: &String, forth: &Forth) -> Result<Devolucion, String> {
    let valor_bool = stack.pop()?;
    let valor_bool = matchear_devolucion_numero(valor_bool).ok_or(String::from("No se pudo extraer el numero del stack"))?;
    if texto.contains("ELSE"){
        if valor_bool == -1 {
            ejecutar_condicion(stack, texto, forth, String::from("ELSE"))?;
            return Ok(Devolucion::Vacio);
        } else{
            ejecutar_condicion(stack, texto, forth, String::from("THEN"))?;
            return Ok(Devolucion::Vacio);
        }
    }else{ //Es IF..THEN
        if valor_bool == -1 {
            ejecutar_condicion(stack, texto, forth, String::from("THEN"))?;
            return Ok(Devolucion::Vacio);
        }
    }
    Err("error al ejecutar clausula if".to_string())
}

fn ejecutar_condicion(stack: &mut Stack, texto: &String, interprete: &Interprete, palabra_freno: String) -> Result<Devolucion, String> {
    for palabra in texto.split_whitespace() {
        if palabra != "IF"{
            if palabra == palabra_freno{
                return Ok(());
            }
            let palabra_formateada = palabra.to_string();
            interprete.parsear_tokens(&palabra_formateada, stack)?;
        }
    }
    Ok(Devolucion::Vacio)
} */

/* use crate::{devolucion::Devolucion, stack::Stack, token_parseo::TokenParseo};

use super::matchear_devolucion_numero;

pub fn ejecutar_if(&self, stack: &mut Stack, v1_if: Vec<TokenParseo>, v2_if: Vec<TokenParseo>) -> Result<Devolucion, String>{
    let valor_bool = stack.pop()?;
    let valor_bool = matchear_devolucion_numero(valor_bool).ok_or(String::from("No se pudo extraer el numero del stack"))?;
    if valor_bool == -1{
        for elem in v1{
            self.ejecutar(&elem, stack);
        }
    }else{
        if v2_if.len() > 0{
            for elem in v2{
                self.ejecutar(&elem, stack);
            }
        }
    }
    Ok(Devolucion::Vacio)
} */