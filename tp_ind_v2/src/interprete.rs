use std::vec;

use crate::{
    devolucion::Devolucion, forth::Forth, token_parseo::{self, TokenParseo},
    word_usuario::WordUsuario,
};

/* pub fn armar_words_usuario(
    tokens: Vec<TokenUsuario>,
    forth: &mut Forth,
) -> Result<Devolucion, String> {
    let mut word_name = String::from("-1");
    for token in tokens {
        match token {
            TokenUsuario::WordName(token_parseo) => {
                if let TokenParseo::WordName(string) = token_parseo {
                    word_name = string.to_string();
                    forth.words_usuarios.push(WordUsuario::new(string));
                } else {
                    return Err(String::from("Error parseo word name"));
                }
            }
            TokenUsuario::WordBody(token_parseo) => {
                let word_actual = forth.get_word_usuario_mut(&word_name);
                if let Some(word) = word_actual {
                    word.agregar_elemento(token_parseo);
                } else {
                    return Err(String::from("No se encontro el word name"));
                }
            }
            TokenUsuario::Ninguno(token_parseo) => {
                if let TokenParseo::Simbolo(_) = &token_parseo {
                } else {
                    forth.restante.push(token_parseo);
                }
                //forth.restante.push(token_parseo);
            }
        }
    }
    Ok(Devolucion::Vacio)
} */

pub fn armar_words_usuario<'a>(forth: &mut Forth<'a>, tokens: &'a Vec<TokenParseo>) -> Result<Devolucion, String>{
    let mut en_armado_word = false;
    let mut nombre_word_actual = String::new();
    for token in tokens{
        if en_armado_word {
            let mut word_actual= forth.get_word_usuario_mut(&nombre_word_actual)?;
            match token{
                TokenParseo::SimboloInicioWord(_) => return Err(String::from("parser error (simbolo inicio)")),
                TokenParseo::SimboloFinWord(_) => en_armado_word = false,
                TokenParseo::Simbolo(_) => {},
                _ => word_actual.agregar_elemento(token)
            }
        }else{
            match token {
                TokenParseo::WordName(nombre) => {
                    nombre_word_actual = nombre.to_string();
                    forth.words_usuarios.push(WordUsuario::new(nombre.to_string()));
                    en_armado_word = true;
                }
                TokenParseo::SimboloInicioWord(_) => {},
                TokenParseo::SimboloFinWord(_) => return Err(String::from("parser error (simbolo fin)")),
                TokenParseo::Simbolo(_) => {},
                _ => forth.restante.push(token),
            }
        }
    }
    Ok(Devolucion::Vacio)
}

/* pub fn armar_words_usuario<'a>(tokens: &'a Vec<TokenParseo>) -> Result<Vec<WordUsuario>, String>{
    let mut resultado: Vec<WordUsuario> = vec![WordUsuario::new("restantes".to_string())];
    let mut en_armado_word = false;
    let mut nombre_word_actual = String::new();
    for token in tokens{
        if en_armado_word {
            let contador_words = resultado.len()-1;
            let mut word_actual= resultado.get_mut(contador_words).ok_or(String::from("error en el get mut"))?;
            match token{
                TokenParseo::SimboloInicioWord(_) => return Err(String::from("parser error (simbolo inicio)")),
                TokenParseo::SimboloFinWord(_) => en_armado_word = false,
                TokenParseo::Simbolo(_) => {},
                _ => word_actual.agregar_elemento(token)
            }
        }else{
            match token {
                TokenParseo::WordName(nombre) => {
                    nombre_word_actual = nombre.to_string();
                    resultado.push(WordUsuario::new(nombre.to_string()));
                    en_armado_word = true;
                }
                TokenParseo::SimboloInicioWord(_) => {},
                TokenParseo::SimboloFinWord(_) => return Err(String::from("parser error (simbolo fin)")),
                TokenParseo::Simbolo(_) => {},
                _ => {
                    let mut word_actual= resultado.get_mut(0).ok_or(String::from("error en el get mut restantes"))?;
                    word_actual.agregar_elemento(token);
                },
            }
        }
    }
    Ok(resultado)
}

pub fn comprobar_no_transitive<'a>(forth: &mut Forth<'a>, words_usuario: &mut Vec<WordUsuario<'a>>) {
    for word in words_usuario{
        let _ = chequeo_word_ya_creada(forth, word);
    }
}

fn chequeo_word_ya_creada<'a>(forth: &mut Forth<'a>, word_actual: &mut WordUsuario<'a>) -> Result<Devolucion, String>{
    let mut contenido = word_actual.get_body();
    for i in 0..contenido.len(){
        if let TokenParseo::Ejecutable(nombre) = contenido[i]{
            let word_encontrada = forth.get_word_usuario_mut(nombre);
            match word_encontrada{
                Ok(word) => {
                    contenido.remove(i);
                    let mut aux:usize = i;
                    for elem in word.get_body(){
                        contenido.insert(aux, elem);
                        aux += 1;
                    }
                },
                Err(_) => {},
            }
        }
    }
    Ok(Devolucion::Vacio)
} */