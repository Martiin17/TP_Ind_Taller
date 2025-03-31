#[derive(Debug)]
pub enum TokenParseo {
    Numero(i16),
    Texto(String),
    WordName(String),
    WordBody(String),
    Simbolo(String),
    Ejecutable(String),
}
