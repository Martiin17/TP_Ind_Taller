use crate::{word_primitiva::WordPrimitiva, word_usuario::WordUsuario};

pub enum Token {
    WordPrimitiva(WordPrimitiva),
    WordPrimitivaConInt(WordPrimitiva, i16),
    WordPrimitivaConString(WordPrimitiva, String),
    WordUsuario(WordUsuario),
    IfThen(Vec<Token>),
    IfElseThen(Vec<Token>, Vec<Token>),
}
