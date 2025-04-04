pub enum Token{
    InicioString,
    FinString,
    PuntoBarra(String),
    EjecutablesSoloStack,
    Numero(i16),
}