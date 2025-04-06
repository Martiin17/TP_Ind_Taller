#[derive(Debug)]
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
}
