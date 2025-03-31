use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::devolucion::Devolucion;
use crate::token_parseo::TokenParseo;
use crate::token_usuario::TokenUsuario;

#[derive(Debug)]
pub struct Parser {
    pub tokens: Vec<TokenUsuario>,
    //pub words_usuario: Vec<WordUsuario>
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
                let word_formateada = word.to_uppercase();
                resultado.push(word_formateada);
            }
        }
        Ok(resultado)
    }

    pub fn parseo(&mut self, leido: Vec<String>) -> Result<Devolucion, String> {
        let mut siguiente_word_name = false;
        let mut termino_word = true;
        let mut en_texto = false;
        for elem in leido {
            if siguiente_word_name {
                //let resultado_parseo = self.matchear_string(elem, &mut siguiente_word_name, &mut termino_word, &mut en_texto);
                self.tokens
                    .push(TokenUsuario::WordName(TokenParseo::WordName(elem)));
                siguiente_word_name = false;
                termino_word = false;
            } else if en_texto {
                if elem.contains("\"") {
                    let elem_sin_comillas = elem.replace("\"", "");
                    if !termino_word {
                        self.tokens.push(TokenUsuario::WordBody(TokenParseo::Texto(
                            elem_sin_comillas,
                        )));
                    } else {
                        self.tokens
                            .push(TokenUsuario::Ninguno(TokenParseo::Texto(elem_sin_comillas)));
                    }
                    en_texto = false;
                } else {
                    if !termino_word {
                        self.tokens
                            .push(TokenUsuario::WordBody(TokenParseo::Texto(elem)));
                    } else {
                        self.tokens
                            .push(TokenUsuario::Ninguno(TokenParseo::Texto(elem)));
                    }
                }
            } else if !termino_word {
                let resultado_parseo = self.matchear_string(
                    elem,
                    &mut siguiente_word_name,
                    &mut termino_word,
                    &mut en_texto,
                );
                self.tokens.push(TokenUsuario::WordBody(resultado_parseo));
            } else if let Ok(nro) = elem.parse::<i16>() {
                self.tokens
                    .push(TokenUsuario::Ninguno(TokenParseo::Numero(nro)));
            } else {
                let resultado_parseo = self.matchear_string(
                    elem,
                    &mut siguiente_word_name,
                    &mut termino_word,
                    &mut en_texto,
                );
                self.tokens.push(TokenUsuario::Ninguno(resultado_parseo));
            }
        }
        Ok(Devolucion::Vacio)
    }

    fn matchear_string(
        &self,
        elem: String,
        siguiente_word_name: &mut bool,
        termino_word: &mut bool,
        en_texto: &mut bool,
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
                *siguiente_word_name = true;
                TokenParseo::Simbolo(elem)
            }
            ";" => {
                *termino_word = true;
                TokenParseo::Simbolo(elem)
            }
            ".\"" => {
                *en_texto = true;
                TokenParseo::Simbolo(elem)
            }
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
