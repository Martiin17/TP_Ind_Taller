use std::{string, vec};

use crate::{devolucion::Devolucion, parametro_body::ParametroBody, stack::{self, Stack}, token_parseo::TokenParseo, utils::{
    funciones_aritmetica, funciones_logicas, funciones_outup, funciones_stack, matchear_devolucion_numero}, word_usuario::{self, WordUsuario}};

pub struct Forth<'a> {
    pub words_usuarios: Vec<WordUsuario<'a>>,
    pub restante: Vec<TokenParseo>,
    pub bodys: Vec<Vec<ParametroBody>>,
}

impl <'a> Forth<'a> {
    pub fn new() -> Self {
        Self {
            words_usuarios: Vec::new(),
            restante: Vec::new(),
            bodys: Vec::new(),
        }
    }

    pub fn encontrar_word(&self, nombre: &String) -> Result<usize, String>{
        for i in 0..self.words_usuarios.len(){
            let i_invertido:usize = self.words_usuarios.len()-1-i;
            let word_actual = &self.words_usuarios[i_invertido];
            if word_actual.get_nombre() == nombre{
                return  Ok(i_invertido);
            }
        }
        Err(String::from("No se encontro la definicion dentro de la definicion"))
    }

   pub fn set_word_usuarios(&mut self, nuevo_words_usuarios: Vec<WordUsuario<'a>>) {
        self.words_usuarios = nuevo_words_usuarios;
    }   


    pub fn get_word_usuario_mut(&mut self, nombre_word: &String) -> Result<&mut WordUsuario<'a>, String> {
        self.words_usuarios
            .iter_mut()
            .find(|word| word.get_nombre() == nombre_word)
            .ok_or(String::from("No se encontro el nombre de la word(invalid word)"))

    }

    pub fn get_word_usuario(&self, nombre_word: &String) -> Result<&WordUsuario<'a>, String> {
        self.words_usuarios
            .iter()
            .find(|word| word.get_nombre() == nombre_word)
            .ok_or(String::from("No se encontro el nombre de la word(invalid word)"))

    }

    pub fn quitar_elemento(&mut self, i: usize) {
        self.words_usuarios.remove(i);

    }

    //El posta

    pub fn ejecutar_tokens(&self, stack: &mut Stack) -> Result<Devolucion, String>{
        for token in &self.restante{
            self.ejecutar(token, stack)?;
        }
        Ok(Devolucion::Vacio)
    }

    pub fn ejecutar(&self, token_a_ejecutar: &TokenParseo, stack: &mut Stack) -> Result<Devolucion, String>{

        let _ = match token_a_ejecutar{
            TokenParseo::Numero(nro) => funciones_stack::ejecutar_int(stack, *nro),
            TokenParseo::Texto(texto) => funciones_outup::ejecutar_punto_y_coma(stack, texto),
            TokenParseo::Ejecutable(string_ejecutable) => {
                let encontrado = self.encontrar_word_ejecutar(string_ejecutable, stack)?;
                if !encontrado{
                    self.matchear_ejecutable(string_ejecutable, stack)
                }else{
                    Ok(Devolucion::Vacio)
                }
            },
            TokenParseo::DentroIF(_) => {
                self.ejecutar_if(token_a_ejecutar, stack)
            }
            _ => {
                return Err("error ejecucion".to_string())}
        };
        Ok(Devolucion::Vacio)
    }

    fn encontrar_word_ejecutar(&self, string_ejecutable: &String, stack: &mut Stack) -> Result<bool, String> {
        //let encontrado = false;
        let indice_word = self.encontrar_word(string_ejecutable)?;
        self.ejecutar_por_indice(&indice_word, stack)?;
        /* for word in &self.words_usuarios{
            if word.get_nombre() == string_ejecutable{
                encontrado = true;
                self.ejecutar_por_indice(word.get_indice(), stack)?;
                break;
            }
        } */
        //Ok(encontrado)
        Ok(true)
    }

    fn ejecutar_por_indice(&self, indice: &usize, stack: &mut Stack) -> Result<Devolucion, String>{
        for tokens_ejecutar in self.bodys.get(*indice){
            for token in tokens_ejecutar{
                match token{
                    ParametroBody::Token(token_actual) => {
                        self.ejecutar(token_actual, stack)?;
                    },
                    ParametroBody::Indice(indice) => {
                        self.ejecutar_por_indice(indice, stack)?;
                    },
                }
            }
        }
        Ok(Devolucion::Vacio)
    }

    fn ejecutar_if(&self, token: &TokenParseo, stack: &mut Stack) -> Result<Devolucion, String> {
        let cond = stack.pop()?;
        let cond = matchear_devolucion_numero(cond)
            .ok_or(String::from("No se pudo extraer el numero del stack"))?;

        if let TokenParseo::DentroIF(vector) = token{
            for token_dentro_if in vector{
                match token_dentro_if{
                    TokenParseo::Numero(_) | TokenParseo::Texto(_) |
                    TokenParseo::Ejecutable(_)=> {
                        if cond == -1{
                            self.ejecutar(token_dentro_if, stack)?;
                        }
                    },
                    TokenParseo::DentroIF(_) => {
                        if cond == -1{
                            self.ejecutar_if(token_dentro_if, stack)?;
                        }
                    },
                    TokenParseo::DentroELSE(tokens_else) => {
                        if cond != -1{
                            for token_else in tokens_else{
                                self.ejecutar(token_else, stack)?;
                            }
                        }
                    },
                    _ => {
                        return Err(String::from("Llego a token if un token que no puede ser"));
                    },
                }
            }
            return Ok(Devolucion::Vacio);
        }
        Err(String::from("No llego un token if a ejecutar if"))
    }

    /* fn ejecutar_condicion_verdadera(&self, vector: &Vec<TokenParseo>, stack: &mut Stack) -> Result<Devolucion, String> {
        for token_dentro_if in vector{
            match token_dentro_if{
                TokenParseo::Numero(_) | TokenParseo::Texto(_) |
                TokenParseo::Ejecutable(_)=> {
                    self.ejecutar(token_dentro_if, stack)?;
                },
                TokenParseo::DentroIF(_) => {
                    self.ejecutar_if(token_dentro_if, stack)?;
                },
                TokenParseo::DentroELSE(_) => {
                    continue;
                },
                _ => {
                    return Err(String::from("Llego a token if un token que no puede ser"));
                },
            }
        }
        return Ok(Devolucion::Vacio);
    } */

    fn matchear_ejecutable(&self, string_ejecutable: &String, stack: &mut Stack) -> Result<Devolucion, String> {
        match string_ejecutable.as_str(){
            "+" => funciones_aritmetica::ejecutar_suma(stack),
            "-" => funciones_aritmetica::ejecutar_resta(stack),
            "/" => funciones_aritmetica::ejecutar_division(stack),
            "*" => funciones_aritmetica::ejecutar_multiplicacion(stack),
            "AND" => funciones_logicas::ejecutar_and(stack),
            "OR" => funciones_logicas::ejecutar_or(stack),
            "<" => funciones_logicas::ejecutar_menor(stack),
            ">" => funciones_logicas::ejecutar_mayor(stack),
            "NOT" => funciones_logicas::ejecutar_not(stack),
            "=" => funciones_logicas::ejecutar_igual(stack),
            "." => funciones_outup::ejecutar_punto(stack),
            "CR" => funciones_outup::ejecutar_cr(stack),
            "EMIT" => funciones_outup::ejecutar_emit(stack),
            "DUP" => funciones_stack::ejecutar_dup(stack),
            "DROP" => funciones_stack::ejecutar_drop(stack),
            "SWAP" => funciones_stack::ejecutar_swap(stack),
            "OVER" => funciones_stack::ejecutar_over(stack),
            "ROT" => funciones_stack::ejecutar_rot(stack),
            _ => Err("No se pudo matchear la word".to_string())
        }
    }  

    /* fn ejecutar_if(&self, stack: &mut Stack, tokens: &[&TokenParseo]) -> Result<Devolucion, String>{
        let cond = stack.pop()?;
        let cond = matchear_devolucion_numero(cond)
            .ok_or(String::from("No se pudo extraer el numero del stack"))?;
        let indices = self.encontrar_cierre_if(tokens)?;
        if cond == -1{
            for i in 1..indices.0{
                let token = &tokens[i];
                self.ejecutar(token, stack)?;
            }
            if indices.1 == 0{
                return Ok(Devolucion::Indice(indices.0))
            }
            return Ok(Devolucion::Indice(indices.1))
        }else {
            if indices.1 != 0{
                for i in indices.0+1..indices.1{
                    let token = &tokens[i];
                    self.ejecutar(token, stack)?;
                }
                return Ok(Devolucion::Indice(indices.1))
            }
            return Ok(Devolucion::Indice(indices.0))
        }
    }

    pub fn verificar_no_transitive(&mut self) -> Result<(), String> {
        for i in 0..self.words_usuarios.len() {
            let indices_word_names = self.identificar_word_names(i)?;
            
            if indices_word_names.is_empty() {
                continue;
            }
            
            self.agregar_tokens(i, indices_word_names)?;
        }
        
        Ok(())
    }
    
    fn identificar_word_names(&self, indice_palabra: usize) -> Result<Vec<(usize, String)>, String> {
        let mut indices_word_names: Vec<(usize, String)> = Vec::new();
        let nombre_actual = self.words_usuarios[indice_palabra].get_nombre();
        
        for (j, &token) in self.words_usuarios[indice_palabra].get_body_not_mut().iter().enumerate() {
            if let TokenParseo::WordName(nombre) = token {
                if nombre == nombre_actual {
                    return Err("invalid-word (recursion infinita)".to_string());
                }
                
                indices_word_names.push((j, nombre.to_string()));
            }
        }
        
        indices_word_names.sort_by(|a, b| b.0.cmp(&a.0));
        
        Ok(indices_word_names)
    }
    

    fn agregar_tokens(&mut self, indice_palabra: usize, indices: Vec<(usize, String)>) -> Result<(), String> {
        for (indice, nombre) in indices {
            let body_a_insertar = self.obtener_body_palabra(&nombre, indice_palabra)?;
            
            let body = self.words_usuarios[indice_palabra].get_body();
            
            body.remove(indice);
            
            for (i, &token) in body_a_insertar.iter().enumerate() {
                body.insert(indice + i, token);
            }
        }
        
        Ok(())
    }
    
    fn obtener_body_palabra(&self, nombre: &str, indice_actual: usize) -> Result<Vec<&'a TokenParseo>, String> {
        let mut tokens_body = Vec::new();
        let mut encontrado = false;
        
        for j in 0..self.words_usuarios.len() {
            if indice_actual == j {
                continue;
            }
            
            if self.words_usuarios[j].get_nombre() == nombre {
                encontrado = true;
                for &token in self.words_usuarios[j].get_body_not_mut() {
                    tokens_body.push(token);
                }
                break;
            }
        }
        
        if !encontrado {
            return Err("Invalid-word (No se encontro la palabra)".to_string());
        }
        
        Ok(tokens_body)
    }

    pub fn encontrar_cierre_if(&self, tokens: &[&TokenParseo]) -> Result<(usize, usize), String>{
        let mut contador_local = 0;
        let mut cantidad_if = 0;
        let mut fin_if: usize = 0;
        let mut fin_else: usize = 0;
        let mut hay_else = false;
        for elem in tokens{
            if let TokenParseo::IF = elem{
                cantidad_if += 1;
            }
            if let TokenParseo::THEN = elem{
                cantidad_if -= 1;
                if cantidad_if == 0{
                    if hay_else{
                        fin_else = contador_local;
                        return Ok((fin_if,fin_else));
                    }
                    fin_if = contador_local;
                    return Ok((fin_if,fin_else));
                }
            }
            if let TokenParseo::ELSE = elem{
                hay_else = true;
                if cantidad_if == 1{
                    fin_if = contador_local;
                }
            }
            contador_local += 1;
        }
        Err("no se cerro bien el if".to_string())
    } */
}