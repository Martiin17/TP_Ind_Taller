use std::vec;

use crate::{parametro_if::ParametroIf, token_parseo::TokenParseo};

#[derive(Debug)]
pub struct EstructuraIf<'a>{
    v_if: Vec<ParametroIf<'a>>,
    v_else: Vec<ParametroIf<'a>>
}

impl <'a>EstructuraIf<'a>{
    pub fn new() -> Self{
        Self{
            v_if: vec![],
            v_else: vec![],
        }
    }

    pub fn get_v_if(&mut self) -> &mut Vec<ParametroIf<'a>>{
        &mut self.v_if
    }

    pub fn get_v_else(&mut self) -> &mut Vec<ParametroIf<'a>>{
        &mut self.v_else
    }
}