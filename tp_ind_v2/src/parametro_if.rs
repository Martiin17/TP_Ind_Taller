use crate::token_parseo::TokenParseo;

#[derive(Debug)]
pub enum ParametroIf<'a>{
    TokenIndividual(&'a TokenParseo),
    VectorTokens(Vec<&'a TokenParseo>),
}