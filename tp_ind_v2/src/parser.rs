use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec;

use crate::devolucion::Devolucion;
use crate::estructura_if::{self, EstructuraIf};
use crate::forth::Forth;
use crate::stack::Stack;
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

    //El posta
    /* pub fn parseo(&mut self, leido: Vec<String>) -> Result<Devolucion, String> {
        let mut proximo_word_name = false;
        let mut es_texto = false;
        let mut dentro_de_word = false;
        let mut texto_acumulado = String::new();
        for elem in leido {
            if proximo_word_name {
                self.tokens.push(TokenParseo::WordName(elem.to_uppercase()));
                proximo_word_name = false;
            }else if let Ok(nro) = elem.parse::<i16>(){
                self.tokens.push(TokenParseo::Numero(nro));
            }else if es_texto{
                es_texto = self.procesar_texto(&elem, &mut texto_acumulado);
            }else if elem == ".\""{
                es_texto = true;
                texto_acumulado.clear();
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

    fn procesar_texto(&mut self, elem: &str, texto_acumulado: &mut String) -> bool {
        if elem.contains("\"") {
            let partes: Vec<&str> = elem.split('\"').collect();
            texto_acumulado.push_str(partes[0]);
            
            let texto_final = std::mem::take(texto_acumulado);
            self.tokens.push(TokenParseo::Texto(texto_final));
            
            return false;
        } else {
            texto_acumulado.push_str(elem);
            texto_acumulado.push_str(" ");
            
            return true;
        }
    }

    fn matchear_string(
        &self,
        elem: String,
        proximo_word_name: &mut bool,
        es_texto: &mut bool,
        dentro_de_word: &mut bool
    ) -> TokenParseo {
        match elem.as_str() {
            "+" | "-" | "/" | "*" | "AND" | "OR" | "<" | ">" |
            "NOT" | "=" | "." | "CR" | "EMIT" | "DUP" | "DROP" |
            "SWAP" | "OVER" => TokenParseo::Ejecutable(elem),
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
                println!("error ./");
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
 */

    pub fn parseo(&self, leido: &[String]) -> Result<Vec<TokenParseo>, String> {
        let mut proximo_word_name = false;
        let mut es_texto = false;
        let mut dentro_de_word = false;
        let mut resultado: Vec<TokenParseo> = vec![];
        let mut i:usize = 0;
        let mut niveles_if: i16 = 0;
        while i < leido.len() {
            let elem = &leido[i];
            if proximo_word_name {
                resultado.push(TokenParseo::WordName(elem.to_uppercase()));
                proximo_word_name = false;
            }else if let Ok(nro) = elem.parse::<i16>(){
                resultado.push(TokenParseo::Numero(nro));
            }else if es_texto{
                let token = self.encontrar_texto(&leido, &mut i);
                resultado.push(token);
                es_texto = false;
            }/* else if elem.to_uppercase() == "IF"{
                niveles_if += 1;
                let token = self.hacer_if_dft(&leido, &mut i, &mut niveles_if)?;
                resultado.push(token);
            } */
            else{
                let resultado_parseo = self.matchear_string(elem.to_uppercase(), 
                &mut proximo_word_name,
                &mut es_texto,
                &mut dentro_de_word,
            );
                resultado.push(resultado_parseo);
            }
            i += 1;
        }
        Ok(resultado)
    }

    /* fn hacer_if_dft(&self, leido: &[String], contador: &mut usize, niveles_if: &mut i16) -> Result<TokenParseo, String>{
        let mut contador_local: usize = 0;
        let mut contador_if: usize = 0;
        let mut hubo_else = false;
        for i in *contador+1..leido.len(){
            let elem = &leido[i];
            if elem.to_uppercase() == "THEN"{
                *niveles_if -= 1;
                if *niveles_if == 0 && !hubo_else{
                    let vector = self.parseo(&leido[*contador+1..i])?;
                    *contador += contador_local;
                    return Ok(TokenParseo::DentroIF(vector));
                }
                if *niveles_if == 0 && hubo_else{
                    let mut vector_if = self.parseo(&leido[*contador+1..contador_if])?;
                    let vector_else = self.parseo(&leido[contador_if+1..i])?;
                    let token_else = TokenParseo::DentroELSE(vector_else);
                    vector_if.push(token_else);
                    *contador += contador_local;
                    return Ok(TokenParseo::DentroIF(vector_if));
                }
            }
            if elem.to_uppercase() == "IF"{
                *niveles_if += 1;
            }
            if elem.to_uppercase() == "ELSE"{
                contador_if = i;
                hubo_else = true;
            }
            contador_local += 1;
        }
        Err("No se encontro THEN".to_string())
    } */

    fn encontrar_texto(&self, leido: &[String], contador: &mut usize) -> TokenParseo {
        let mut texto_acumulado = String::new();
        let mut contador_local: usize = 0;
        for i in *contador..leido.len(){
            let elem = &leido[i];
            println!("elem: {}", elem);
            if elem.contains("\"") {
                let partes: Vec<&str> = elem.split('\"').collect();
                texto_acumulado.push_str(partes[0]);
                break;
            } else {
                texto_acumulado.push_str(elem);
                texto_acumulado.push_str(" ");   
            }
            contador_local += 1;
        }
        *contador += contador_local;
        TokenParseo::Texto(texto_acumulado)
    }

    fn matchear_string(
        &self,
        elem: String,
        proximo_word_name: &mut bool,
        es_texto: &mut bool,
        dentro_de_word: &mut bool
    ) -> TokenParseo {
        match elem.as_str() {
            "+" | "-" | "/" | "*" | "AND" | "OR" | "<" | ">" |
            "NOT" | "=" | "." | "CR" | "EMIT" | "DUP" | "DROP" |
            "SWAP" | "OVER" => TokenParseo::Ejecutable(elem),
            "IF" => TokenParseo::IF,
            "THEN" => TokenParseo::THEN,
            "ELSE" => TokenParseo::ELSE,
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

    /* pub fn parseo_2(
        &self,
        elem: String,
        proximo_word_name: &mut bool,
        es_texto: &mut bool,
        dentro_de_word: &mut bool,
        mut niveles_if: &mut i16,

    ) -> Result<TokenParseo, String> {
        if *proximo_word_name {
            *proximo_word_name = false;
            return Ok(TokenParseo::WordName(elem.to_uppercase()));
        }else if let Ok(nro) = elem.parse::<i16>(){
            return Ok(TokenParseo::Numero(nro));
        }else if *es_texto{
            if elem.contains(".\""){
                *es_texto = false;
            }
            return Ok(TokenParseo::Texto(elem));
        }else if elem.to_uppercase() == "IF"{
            *niveles_if += 1;
            let token = self.hacer_if_dft(&elem, &mut niveles_if)?;
            return Ok(token);
        }
        else{
            let resultado_parseo = self.matchear_string(elem.to_uppercase(), 
            proximo_word_name,
            es_texto,
            dentro_de_word,
        );
            return Ok(resultado_parseo);
        }
    }

    fn hacer_if_dft(&self, elem: &String, niveles_if: &mut i16) -> Result<TokenParseo, String>{
        let mut contador_local: usize = 0;
        let mut contador_if: usize = 0;
        let mut hubo_else = false;
        for i in *contador+1..leido.len(){
            let elem = &leido[i];
            if elem.to_uppercase() == "THEN"{
                *niveles_if -= 1;
                if *niveles_if == 0 && !hubo_else{
                    let vector = self.parseo(&leido[*contador+1..i])?;
                    *contador += contador_local;
                    return Ok(TokenParseo::DentroIF(vector));
                }
                if *niveles_if == 0 && hubo_else{
                    let mut vector_if = self.parseo(&leido[*contador+1..contador_if])?;
                    let vector_else = self.parseo(&leido[contador_if+1..i])?;
                    let token_else = TokenParseo::DentroELSE(vector_else);
                    vector_if.push(token_else);
                    *contador += contador_local;
                    return Ok(TokenParseo::DentroIF(vector_if));
                }
            }
            if elem.to_uppercase() == "IF"{
                *niveles_if += 1;
            }
            if elem.to_uppercase() == "ELSE"{
                contador_if = i;
                hubo_else = true;
            }
            contador_local += 1;
        }
        Err("No se encontro THEN".to_string())
    } */

}