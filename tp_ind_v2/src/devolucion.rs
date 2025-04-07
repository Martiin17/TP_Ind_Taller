use crate::token_parseo::TokenParseo;

pub enum Devolucion {
    Numero(i16),
    Vacio,
    Token(TokenParseo),
    Indice(usize)
}
