use crate::token_parseo::TokenParseo;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec;

/// struct Parser
/// 
/// Representa un conjunto de funciones que se encargan de parsear lo leido desde un archivo .fth
#[derive(Debug)]
pub struct Parser {
    pub tokens: Vec<TokenParseo>,
}

impl Parser {
    /// Crea un nuevo Parser
    pub fn new() -> Self {
        Self { tokens: Vec::new() }
    }

    ///Lee el archivo y devuelve un io::Result<Vec<String>> con lo leido
    pub fn leer_archivo(&self, nombre_archivo: &str) -> io::Result<Vec<String>> {
        let path = Path::new(nombre_archivo);
        let file = File::open(path)?;
        let reader = io::BufReader::new(file);
    
        let mut resultado: Vec<String> = Vec::new();
        let mut sigue_texto = false;
    
        for linea in reader.lines() {
            let mut linea = linea?;
            linea.insert(0,' ');
            linea.insert(linea.len(), ' ');
    
            self.leer_archivo_aux(&mut linea, &mut resultado, &mut sigue_texto);
        }
        Ok(resultado)
    }
    
    fn leer_archivo_aux(&self, linea: &mut str, resultado: &mut Vec<String>, sigue_texto: &mut bool) {
        let mut armado_string = String::new();
        for word in linea.chars() {
            if word == ' ' && !*sigue_texto{
                if armado_string == ".\""{
                    *sigue_texto = true;
                }
                if armado_string.is_empty(){
                    continue;
                }
                resultado.push(armado_string);
                armado_string = String::new();
            }else if *sigue_texto && word == '\"'{
                *sigue_texto = false
            }else{
                armado_string.push(word);
            }
        }
    }

    ///Se encarga de asignarle un TokenParseo a un slice de Strings
    pub fn parseo(&self, leido: &[String]) -> Result<Vec<TokenParseo>, String> {
        let mut proximo_word_name = false;
        let mut es_texto = false;
        let mut dentro_de_word = false;
        let mut resultado: Vec<TokenParseo> = vec![];
        let mut i: usize = 0;
        let mut niveles_if: i16 = 0;
        while i < leido.len() {
            let elem = &leido[i];
            if proximo_word_name {
                resultado.push(TokenParseo::WordName(elem.to_uppercase()));
                proximo_word_name = false;
            } else if let Ok(nro) = elem.parse::<i16>() {
                resultado.push(TokenParseo::Numero(nro));
            } else if es_texto {
                let token = self.encontrar_texto(leido, &mut i);
                resultado.push(token);
                es_texto = false;
            } else if elem.to_uppercase() == "IF" {
                niveles_if += 1;
                let token = self.hacer_if_dft(leido, &mut i, &mut niveles_if)?;
                resultado.push(token);
            } else {
                let resultado_parseo = self.matchear_string(
                    elem.to_uppercase(),
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

    /// Se encarga de parsear los casos especiales de IF ELSE THEN y IF THEN
    fn hacer_if_dft(
        &self,
        leido: &[String],
        contador: &mut usize,
        niveles_if: &mut i16,
    ) -> Result<TokenParseo, String> {
        let mut contador_if: usize = 0;
        let mut hubo_else = false;

        for (contador_local, i) in (*contador+1..leido.len()).enumerate(){
            let elem = &leido[i];
            if elem.to_uppercase() == "THEN"{
                *niveles_if -= 1;
                if *niveles_if == 0 && !hubo_else {
                    let vector = self.parseo(&leido[*contador + 1..i])?;
                    *contador += contador_local;
                    return Ok(TokenParseo::DentroIF(vector));
                }
                if *niveles_if == 0 && hubo_else {

                    let mut vector_if = self.parseo(&leido[*contador + 1..contador_if])?;
                    let vector_else = self.parseo(&leido[contador_if + 1..i])?;
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
                if *niveles_if == 1{
                    contador_if = i;
                }
                //contador_if = i;
                hubo_else = true;
            }
        }
        Err("No se encontro THEN".to_string())
    }

    /// Se encarga de parsear los casos especiales ." " y el texto que incluye
    /// 
    /// Devuelve un TokenParseo::Texto() con todo lo que estaba entre ." "
    fn encontrar_texto(&self, leido: &[String], contador: &mut usize) -> TokenParseo {
        let mut texto_acumulado = String::new();
        let mut contador_local: usize = 0;
        for elem in leido.iter().skip(*contador){
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

    /// Se encarga de asignarle un TokenParseo a los operadores por defecto
    fn matchear_string(
        &self,
        elem: String,
        proximo_word_name: &mut bool,
        es_texto: &mut bool,
        dentro_de_word: &mut bool,
    ) -> TokenParseo {
        match elem.as_str() {
            "+" | "-" | "/" | "*" | "AND" | "OR" | "<" | ">" | "NOT" | "=" | "." | "CR"
            | "EMIT" | "DUP" | "DROP" | "SWAP" | "OVER" => TokenParseo::Ejecutable(elem),
            "IF" => TokenParseo::IF,
            "THEN" => TokenParseo::THEN,
            "ELSE" => TokenParseo::ELSE,
            ":" => {
                *proximo_word_name = true;
                *dentro_de_word = true;
                TokenParseo::SimboloInicioWord(elem)
            }
            ";" => {
                *dentro_de_word = false;
                TokenParseo::SimboloFinWord(elem)
            }
            ".\"" => {
                *es_texto = true;
                TokenParseo::Simbolo(elem)
            }
            _ => {
                if *dentro_de_word {
                    TokenParseo::WordName(elem)
                } else {
                    TokenParseo::Ejecutable(elem)
                }
            }
        }
    }
}
