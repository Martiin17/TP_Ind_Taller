use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::devolucion::Devolucion;
use crate::token_parseo::TokenParseo;

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
        for elem in leido {
            if proximo_word_name {
                self.tokens.push(TokenParseo::WordName(elem));
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
        es_texto: &mut bool
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
                TokenParseo::SimboloInicioWord(elem)
            },
            ";" => TokenParseo::SimboloFinWord(elem),
            ".\"" => {
                *es_texto = true;
                TokenParseo::Simbolo(elem)
            },
            _ => TokenParseo::Ejecutable(elem),
        }
    }

    //lo de arriba parser y lo de abajo interprete?
    /* pub fn ejecutar(tokens: Vec<TokenParseo>) -> Result<Devolucion, String> {
           let mut word_aux = WordUsuario::new(String::from("-1"));
           let mut en_word_body = false;
           for token in tokens{
               if en_word_body == true{
                   if let TokenParseo::Simbolo(_) = TokenParseo::Simbolo(String::from(";")){
                       en_word_body = false;
                   }else{
                       word_aux.agregar_elemento(token);
                   }
               }else{
                   match token {
                       TokenParseo::Numero(_) => todo!(),
                       TokenParseo::Texto(_) => todo!(),
                       TokenParseo::WordName(nombre) => {
                           word_aux = WordUsuario::new(nombre);
                       },
                       TokenParseo::WordBody(_) => {
                           //if esta en el de usuarios otra vez esta fun, si no:
                           word_aux.agregar_elemento(token);
                       },
                       TokenParseo::Simbolo(string) => {
                           if string == String::from(";"){
                               en_word_body = true;
                           }
                       },
                       TokenParseo::Ejecutable(_) => todo!(),
                   }
               }
           }
           Ok(Devolucion::Vacio)
       }
    */
}
