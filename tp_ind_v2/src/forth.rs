use crate::{devolucion::Devolucion, stack::{self, Stack}, token_parseo::TokenParseo, utils::{
    funciones_aritmetica, funciones_logicas, funciones_outup, funciones_stack, matchear_devolucion_numero}, word_usuario::WordUsuario};

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

    //El posta
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

    /* // Método principal para ejecutar tokens
    pub fn ejecutar_tokens(&self, tokens: &[TokenParseo], stack: &mut Stack, pos: usize) 
    -> Result<(Devolucion, usize), String> {

        if pos >= tokens.len() {
            return Ok((Devolucion::Vacio, pos));
        }

        match &tokens[pos] {
            TokenParseo::Numero(nro) => {
                funciones_stack::ejecutar_int(stack, *nro)?;
                Ok((Devolucion::Vacio, pos + 1))
            },
            TokenParseo::Texto(texto) => {
                funciones_outup::ejecutar_punto_y_coma(stack, texto)?;
                Ok((Devolucion::Vacio, pos + 1))
            },
            TokenParseo::Ejecutable(comando) => self.ejecutar_comando(comando, tokens, stack, pos),
            _ => Err("error ejecucion".to_string())
        }
    }

    // Maneja comandos ejecutables (incluidos IF, ELSE, THEN)
    fn ejecutar_comando(&self, comando: &String, tokens: &[TokenParseo], stack: &mut Stack, pos: usize) 
    -> Result<(Devolucion, usize), String> {

        match comando.as_str() {
            "IF" => self.ejecutar_if(tokens, stack, pos),
            "ELSE" | "THEN" => Ok((Devolucion::Vacio, pos + 1)),
            _ => {
                let encontrado = self.encontrar_word_ejecutar(comando, stack)?;
                if !encontrado {
                    self.matchear_ejecutable(comando, stack)?;
                }
                Ok((Devolucion::Vacio, pos + 1))
            }
        }
    }

    // Ejecuta un bloque IF-ELSE-THEN
    fn ejecutar_if(&self, tokens: &[TokenParseo], stack: &mut Stack, pos: usize) 
    -> Result<(Devolucion, usize), String> {

        let condicion = stack.pop()?;

        if condicion != 0 {
            // Condición verdadera - ejecutar bloque IF
            self.ejecutar_bloque_if(tokens, stack, pos + 1)
        } else {
            // Condición falsa - buscar ELSE o THEN
            let pos_salto = self.buscar_else_o_then(tokens, pos + 1, 0)?;
            
            if pos_salto < tokens.len() && self.es_token_else(&tokens[pos_salto]) {
                // Ejecutar bloque ELSE si existe
                self.ejecutar_bloque_else(tokens, stack, pos_salto + 1)
            } else {
                // Saltar al THEN
                Ok((Devolucion::Vacio, pos_salto + 1))
            }
        }
    }

    // Ejecuta secuencialmente tokens hasta encontrar ELSE o THEN
    fn ejecutar_bloque_if(&self, tokens: &[TokenParseo], stack: &mut Stack, pos_inicio: usize) 
    -> Result<(Devolucion, usize), String> {

        let mut pos = pos_inicio;

        while pos < tokens.len() {
            if self.es_token_else(&tokens[pos]) || self.es_token_then(&tokens[pos]) {
                // Si encontramos ELSE, saltamos al THEN
                if self.es_token_else(&tokens[pos]) {
                    pos = self.buscar_then(tokens, pos + 1, 0)?;
                }
                return Ok((Devolucion::Vacio, pos + 1));
            }
            
            let (_, nueva_pos) = self.ejecutar_tokens(tokens, stack, pos)?;
            pos = nueva_pos;
        }

        Err("Bloque IF sin THEN correspondiente".to_string())
    }

    // Ejecuta secuencialmente tokens hasta encontrar THEN
    fn ejecutar_bloque_else(&self, tokens: &[TokenParseo], stack: &mut Stack, pos_inicio: usize) 
    -> Result<(Devolucion, usize), String> {

        let mut pos = pos_inicio;

        while pos < tokens.len() {
            if self.es_token_then(&tokens[pos]) {
                return Ok((Devolucion::Vacio, pos + 1));
            }
            
            let (_, nueva_pos) = self.ejecutar_tokens(tokens, stack, pos)?;
            pos = nueva_pos;
        }

        Err("Bloque ELSE sin THEN correspondiente".to_string())
    }

    // Busca el siguiente ELSE o THEN correspondiente al nivel de anidamiento
    fn buscar_else_o_then(&self, tokens: &[TokenParseo], pos_inicio: usize, nivel: usize) 
    -> Result<usize, String> {

        let mut pos = pos_inicio;
        let mut nivel_actual = nivel;

        while pos < tokens.len() {
            if self.es_token_if(&tokens[pos]) {
                nivel_actual += 1;
            } else if self.es_token_then(&tokens[pos]) {
                if nivel_actual == 0 {
                    return Ok(pos);
                }
                nivel_actual -= 1;
            } else if self.es_token_else(&tokens[pos]) && nivel_actual == 0 {
                return Ok(pos);
            }
            
            pos += 1;
        }

        Err("No se encontró ELSE o THEN correspondiente".to_string())
    }

    // Busca el siguiente THEN correspondiente al nivel de anidamiento
    fn buscar_then(&self, tokens: &[TokenParseo], pos_inicio: usize, nivel: usize) 
    -> Result<usize, String> {

        let mut pos = pos_inicio;
        let mut nivel_actual = nivel;

        while pos < tokens.len() {
            if self.es_token_if(&tokens[pos]) {
                nivel_actual += 1;
            } else if self.es_token_then(&tokens[pos]) {
                if nivel_actual == 0 {
                    return Ok(pos);
                }
                nivel_actual -= 1;
            }
            
            pos += 1;
        }

        Err("No se encontró THEN correspondiente".to_string())
    }

    // Funciones auxiliares para mejorar legibilidad y evitar duplicación
    fn es_token_if(&self, token: &TokenParseo) -> bool {
        matches!(token, TokenParseo::Ejecutable(s) if s == "IF")
    }

    fn es_token_else(&self, token: &TokenParseo) -> bool {
        matches!(token, TokenParseo::Ejecutable(s) if s == "ELSE")
    }

    fn es_token_then(&self, token: &TokenParseo) -> bool {
        matches!(token, TokenParseo::Ejecutable(s) if s == "THEN")
    }

    // Método para ejecutar una palabra completa
    pub fn ejecutar_palabra(&self, tokens: &[TokenParseo], stack: &mut Stack) -> Result<Devolucion, String> {
        let mut pos = 0;

        while pos < tokens.len() {
            let (_, nueva_pos) = self.ejecutar_tokens(tokens, stack, pos)?;
            pos = nueva_pos;
        }

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
 */
    /* pub fn ejecutar_tokens(&'a self, tokens: &'a [&TokenParseo], stack: &mut Stack) -> Result<(), String> {
        let mut i = 0;
        while i < tokens.len() {
            match &tokens[i] {
                TokenParseo::Numero(n) => {
                    funciones_stack::ejecutar_int(stack, *n)?;
                    i += 1;
                },
                TokenParseo::Texto(t) => {
                    funciones_outup::ejecutar_punto_y_coma(stack, t)?;
                    i += 1;
                },
                TokenParseo::Ejecutable(word) if word == "IF" => {
                    i += 1;
                    let cond = stack.pop()?;
                    let cond = matchear_devolucion_numero(cond)
                        .ok_or("Condición no válida en IF")?;

                    let (bloque_verdadero, bloque_falso, avance) = self.extraer_bloque_if(&tokens[i..])?;
                    let ejecutar = if cond == -1 {
                        bloque_verdadero
                    } else {
                        bloque_falso
                    };

                    self.ejecutar_tokens(ejecutar, stack)?;
                    i += avance;
                },
                TokenParseo::Ejecutable(word) => {
                    let encontrado = self.encontrar_word_ejecutar(word, stack)?;
                    if !encontrado {
                        self.matchear_ejecutable(word, stack)?;
                    }
                    i += 1;
                },
                TokenParseo::WordName(_) => {
                    return Err("No se esperaba un WordName durante ejecución".to_string());
                }
                _ => return Err("aca".to_string()),
            }
        }
        Ok(())
    }

    fn extraer_bloque_if(
        &self,
        tokens: &'a [&TokenParseo],
    ) -> Result<(&'a [&TokenParseo], &'a [&TokenParseo], usize), String> {
        let mut profundidad = 1;
        let mut else_index = None;
        let mut then_index = None;
    
        for (i, token) in tokens.iter().enumerate() {
            match token {
                TokenParseo::Ejecutable(s) if s == "IF" => profundidad += 1,
                TokenParseo::Ejecutable(s) if s == "THEN" => {
                    profundidad -= 1;
                    if profundidad == 0 {
                        then_index = Some(i);
                        break;
                    }
                },
                TokenParseo::Ejecutable(s) if s == "ELSE" && profundidad == 1 => {
                    else_index = Some(i);
                },
                _ => {}
            }
        }
    
        let then_idx = then_index.ok_or("Falta THEN")?;
    
        // En vez de usar slice vacío `&[]`, usamos un slice válido
        let (bloque_true, bloque_false) = match else_index {
            Some(ei) => (&tokens[..ei], &tokens[ei + 1..then_idx]),
            None => (&tokens[..then_idx], &tokens[then_idx..then_idx]), // Slice vacío válido
        };
    
        Ok((bloque_true, bloque_false, then_idx + 1))
    }
    
    

    fn encontrar_word_ejecutar(&self, nombre: &String, stack: &mut Stack) -> Result<bool, String> {
        for word in &self.words_usuarios {
            if word.get_nombre() == nombre {
                for token in word.get_body_not_mut() {
                    self.ejecutar_tokens(std::slice::from_ref(token), stack)?; // Ejecutar uno por uno
                }
                return Ok(true);
            }
        }
        Ok(false)
    }

    fn matchear_ejecutable(&self, string_ejecutable: &str, stack: &mut Stack) -> Result<Devolucion, String> {
        match string_ejecutable {
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
            _ => Err(format!("No se pudo matchear la palabra: {}", string_ejecutable)),
        }
    } */

}