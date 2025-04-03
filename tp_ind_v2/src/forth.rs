use crate::{token_parseo::TokenParseo, word_usuario::WordUsuario};

pub struct Forth<'a> {
    pub words_usuarios: Vec<WordUsuario<'a>>,
    pub restante: Vec<&'a TokenParseo>,
}

impl <'a> Forth<'a> {
    pub fn new() -> Self {
        Self {
            words_usuarios: Vec::new(),
            restante: Vec::new(),
        }
    }

    /* pub fn new(words_usuarios: Vec<WordUsuario<'a>>, restante: Vec<&'a TokenParseo>) -> Self {
        Self {
            words_usuarios,
            restante,
        }
    } */

   pub fn set_word_usuarios(&mut self, nuevo_words_usuarios: Vec<WordUsuario<'a>>) {
        self.words_usuarios = nuevo_words_usuarios;
    }   

    /* pub fn get_word_usuario_mut(&mut self, nombre_word: &String) -> Option<&mut WordUsuario<'a>> {
        self.words_usuarios
            .iter_mut()
            .find(|word| word.get_nombre() == nombre_word)
    } */

    pub fn get_word_usuario_mut(&mut self, nombre_word: &String) -> Result<&mut WordUsuario<'a>, String> {
        self.words_usuarios
            .iter_mut()
            .find(|word| word.get_nombre() == nombre_word)
            .ok_or(String::from("No se encontro el nombre de la word(invalid word)"))

    }
}
