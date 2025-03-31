use crate::{token_parseo::TokenParseo, word_usuario::WordUsuario};

pub struct Forth {
    pub words_usuarios: Vec<WordUsuario>,
    pub restante: Vec<TokenParseo>,
}

impl Forth {
    pub fn new() -> Self {
        Self {
            words_usuarios: Vec::new(),
            restante: Vec::new(),
        }
    }

    pub fn get_word_usuario_mut(&mut self, nombre_word: &String) -> Option<&mut WordUsuario> {
        self.words_usuarios
            .iter_mut()
            .find(|word| word.get_nombre() == nombre_word)
        /* for word in &mut self.words_usuarios {
            if word.get_nombre() == nombre_word {
                return Some(word);
            }
        }
        None */
    }
}
