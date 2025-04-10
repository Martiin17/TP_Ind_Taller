use crate::{devolucion::Devolucion, parametro_body::ParametroBody, stack::Stack, token_parseo::TokenParseo, utils::{
    funciones_aritmetica, funciones_logicas, funciones_outup, funciones_stack, matchear_devolucion_numero}, 
    word_usuario::WordUsuario};

pub struct Forth{
    pub words_usuarios: Vec<WordUsuario>,
    pub bodys: Vec<Vec<ParametroBody>>,
    pub restante: Vec<TokenParseo>,
}

impl Forth{
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
                return Ok(i_invertido);
            }
        }
        Err(String::from("?"))
    }

    pub fn encontrar_word_para_armar_body(&self, nombre: &String) -> Result<usize, String>{
        //let mut primera_vez = true;
        for i in 1..self.words_usuarios.len(){
            let i_invertido:usize = self.words_usuarios.len()-1-i;
            let word_actual = &self.words_usuarios[i_invertido];
            if word_actual.get_nombre() == nombre{
                /* if primera_vez{
                    primera_vez = false;
                    let loop_infinito = self.verificar_loop_infinito_2(&i_invertido);
                    if loop_infinito{
                        continue;
                    }
                } */
                return Ok(i_invertido);
            }
        }
        Err(String::from("?"))
    }

    fn verificar_loop_infinito_2(&self, indice_encontrado: &usize) -> bool{
        let word_encontrada = &self.words_usuarios.get(*indice_encontrado);
        match word_encontrada{
            Some(word) => {
                for tokens in self.bodys.get(*word.get_indice()){
                    println!("llegue1 {:?}", tokens);
                    for token in tokens{
                        println!("llegue2{:?}", token);
                        if let ParametroBody::Indice(numero) = token{
                            //println!("numero{}, word indice {}", numero, word.get_indice());
                            if numero == word.get_indice(){
                                return true;
                            }
                        }
                    }   
                }
                false   
            },
            None => false,
        }
    }

    fn verificar_loop_infinito(&self, nombre: &String, indice_encontrado: &usize) -> bool{
        let word_encontrada = &self.words_usuarios.get(*indice_encontrado);
        match word_encontrada{
            Some(word) => {
                if word.get_nombre() == nombre{
                    return true;
                }else{
                    return  false;
                }
            },
            None => false,
        }
    }

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
                return Err(String::from("ejecution-error"))
            }
        };
        Ok(Devolucion::Vacio)
    }

    fn encontrar_word_ejecutar(&self, string_ejecutable: &String, stack: &mut Stack) -> Result<bool, String> {
        //let mut encontrado= true;
        let indice_word = self.encontrar_word(string_ejecutable);
        match indice_word{
            Ok(indice) => {
                self.ejecutar_por_indice(&indice, stack)?;
            },
            Err(_) => {
                return Ok(false);
            },
        }
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
        let cond = matchear_devolucion_numero(cond)?;

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
                        return Err(String::from("invalid-word (llego a if un token invalido"));
                    },
                }
            }
            return Ok(Devolucion::Vacio);
        }
        Err(String::from("parser-error (No llego un token if a ejecutar if)"))
    }

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
            _ => Err(String::from("?"))
        }
    }  

}