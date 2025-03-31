use crate::{
    devolucion::Devolucion, forth::Forth, token_parseo::TokenParseo, token_usuario::TokenUsuario,
    word_usuario::WordUsuario,
};

pub fn armar_words_usuario(
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
}
