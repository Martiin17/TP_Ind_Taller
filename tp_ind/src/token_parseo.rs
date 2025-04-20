/// Representa los posibles Tokens que pueden ocurrir a la hora de parsear
#[derive(Debug, PartialEq)]
pub enum TokenParseo {
    Numero(i16),
    Texto(String),
    WordName(String),
    SimboloInicioWord(String),
    SimboloFinWord(String),
    Simbolo(String),
    Ejecutable(String),
    DentroIF(Vec<TokenParseo>),
    DentroELSE(Vec<TokenParseo>),
    If,
    Else,
    Then,
}
