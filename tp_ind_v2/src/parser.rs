use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::devolucion::Devolucion;
use crate::forth::Forth;
use crate::token::Token;
use crate::token_parseo::TokenParseo;
use crate::word_usuario::WordUsuario;

#[derive(Debug)]
pub struct Parser {
    pub tokens: Vec<TokenParseo>,
}

impl Parser {
    pub fn new() -> Self {
        Self { tokens: Vec::new() }
    }

    pub fn leer_archivo(&self, nombre_archivo: &str) -> io::Result<Vec<String>> {
        let path = Path::new(nombre_archivo);
        let file = File::open(path)?;
        let reader = io::BufReader::new(file);

        let mut resultado: Vec<String> = Vec::new();

        for linea in reader.lines() {
            let linea = linea?;
            for word in linea.split_whitespace() {
                //let word_formateada = word.to_uppercase();
                resultado.push(word.to_string());
            }
        }
        Ok(resultado)
    }

    pub fn parseo(&mut self, leido: Vec<String>) -> Result<Devolucion, String> {
        let mut proximo_word_name = false;
        let mut es_texto = false;
        let mut dentro_de_word = false;
        for elem in leido {
            if proximo_word_name {
                self.tokens.push(TokenParseo::WordName(elem.to_uppercase()));
                proximo_word_name = false;
            }else if let Ok(nro) = elem.parse::<i16>(){
                self.tokens.push(TokenParseo::Numero(nro));
            }else if es_texto{
                if elem.contains("\""){
                    let elem_sin_comillas = elem.replace("\"", "");
                    self.tokens.push(TokenParseo::Texto(elem_sin_comillas));
                    es_texto = false;
                }else{
                    self.tokens.push(TokenParseo::Texto(elem));
                }
            }else{
                let resultado_parseo = self.matchear_string(elem.to_uppercase(), 
                &mut proximo_word_name,
                &mut es_texto,
                &mut dentro_de_word,
            );
                self.tokens.push(resultado_parseo);
            }
        }
        Ok(Devolucion::Vacio)
    }

    fn matchear_string(
        &self,
        elem: String,
        proximo_word_name: &mut bool,
        es_texto: &mut bool,
        dentro_de_word: &mut bool
    ) -> TokenParseo {
        match elem.as_str() {
            "+" => TokenParseo::Ejecutable(elem),
            "-" => TokenParseo::Ejecutable(elem),
            "/" => TokenParseo::Ejecutable(elem),
            "*" => TokenParseo::Ejecutable(elem),
            "AND" => TokenParseo::Ejecutable(elem),
            "OR" => TokenParseo::Ejecutable(elem),
            "<" => TokenParseo::Ejecutable(elem),
            ">" => TokenParseo::Ejecutable(elem),
            "NOT" => TokenParseo::Ejecutable(elem),
            "=" => TokenParseo::Ejecutable(elem),
            "." => TokenParseo::Ejecutable(elem),
            "CR" => TokenParseo::Ejecutable(elem),
            "EMIT" => TokenParseo::Ejecutable(elem),
            "DUP" => TokenParseo::Ejecutable(elem),
            "DROP" => TokenParseo::Ejecutable(elem),
            "SWAP" => TokenParseo::Ejecutable(elem),
            "OVER" => TokenParseo::Ejecutable(elem),
            "IF" => TokenParseo::Ejecutable(elem),
            "THEN" => TokenParseo::Ejecutable(elem),
            "ELSE" => TokenParseo::Ejecutable(elem),
            ":" => {
                *proximo_word_name = true;
                *dentro_de_word = true;
                TokenParseo::SimboloInicioWord(elem)
            },
            ";" => {
                *dentro_de_word = false;
                TokenParseo::SimboloFinWord(elem)
            },
            ".\"" => {
                *es_texto = true;
                TokenParseo::Simbolo(elem)
            },
            _ => {
                if *dentro_de_word{
                    TokenParseo::WordName(elem)
                }else{
                    TokenParseo::Ejecutable(elem)
                }
            },
        }
    }
}
