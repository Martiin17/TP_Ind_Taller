use crate::token_parseo::TokenParseo;

#[derive(Debug)]
pub enum ParametroBody{
    Token(TokenParseo),
    Indice(usize),
}