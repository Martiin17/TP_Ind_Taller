use crate::token_parseo::TokenParseo;

#[derive(Debug)]
pub enum TokenUsuario {
    WordName(TokenParseo),
    WordBody(TokenParseo),
    Ninguno(TokenParseo),
}
