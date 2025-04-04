use crate::{
    devolucion::Devolucion, forth::Forth, token_parseo:: TokenParseo,
    word_usuario::WordUsuario,
};


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
                TokenParseo::SimboloInicioWord(_) => {}
                TokenParseo::SimboloFinWord(_) => return Err(String::from("parser error (simbolo fin)")),
                TokenParseo::Simbolo(_) => {},
                _ => forth.restante.push(token),
            }
        }
    }
    Ok(Devolucion::Vacio)
}