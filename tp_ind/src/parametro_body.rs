use crate::token_parseo::TokenParseo;

#[derive(Debug)]

/// Representa los posibles parametros que puede tomar el body de una word
pub enum ParametroBody {
    Token(TokenParseo),
    Indice(usize),
}
