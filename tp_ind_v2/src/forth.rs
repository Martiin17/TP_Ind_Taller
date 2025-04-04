use crate::{devolucion::Devolucion, stack::{self, Stack}, token_parseo::TokenParseo, utils::{
    funciones_aritmetica, funciones_outup, funciones_stack, funciones_logicas}, word_usuario::WordUsuario};

pub struct Forth<'a> {
    pub words_usuarios: Vec<WordUsuario<'a>>,
    pub restante: Vec<&'a TokenParseo>,
}

impl <'a> Forth<'a> {
    pub fn new() -> Self {
        Self {
            words_usuarios: Vec::new(),
            restante: Vec::new(),
        }
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
            _ => return Err("error ejecucion".to_string())
        };
        Ok(Devolucion::Vacio)
    }

    fn encontrar_word_ejecutar(&self, string_ejecutable: &String, stack: &mut Stack) -> Result<bool, String> {
        let mut encontrado = false;
        for word in &self.words_usuarios{
            if word.get_nombre() == string_ejecutable{
                encontrado = true;
                for token in word.get_body_not_mut(){
                    self.ejecutar(token, stack)?;
                }
            }
        }
        Ok(encontrado)
    }  

    fn matchear_ejecutable(&self, string_ejecutable: &String, stack: &mut Stack) -> Result<Devolucion, String> {
        match string_ejecutable.as_str(){
            "+" => funciones_aritmetica::ejecutar_suma(stack),
            "-" => funciones_aritmetica::ejecutar_resta(stack),
            "/" => funciones_aritmetica::ejecutar_division(stack),
            "*" => funciones_aritmetica::ejecutar_division(stack),
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
            _ => Err("No se pudo matchear la word".to_string())
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
}
