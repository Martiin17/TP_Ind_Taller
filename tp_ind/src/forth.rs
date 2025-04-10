use crate::{
    devolucion::Devolucion,
    parametro_body::ParametroBody,
    stack::Stack,
    token_parseo::TokenParseo,
    utils::{
        funciones_aritmetica, funciones_logicas, funciones_outup, funciones_stack,
        matchear_devolucion_numero,
    },
    word_usuario::WordUsuario,
};

/// struct Forth
/// 
/// Es una estructura que nos permite almacenar:
/// 
/// words_usuarios --> Las words de usuario
/// 
/// bodys --> Los bodys de dichas words
/// 
/// restante --> Los Tokens que no fueron parte de ninguna word (por lo que deben ser ejecutables si no hay ningun error)
pub struct Forth {
    pub words_usuarios: Vec<WordUsuario>,
    pub bodys: Vec<Vec<ParametroBody>>,
    pub restante: Vec<TokenParseo>,
}

impl Forth {
    /// Crea un Forth con todas las listas vacias
    pub fn new() -> Self {
        Self {
            words_usuarios: Vec::new(),
            restante: Vec::new(),
            bodys: Vec::new(),
        }
    }

    /// Encuentra el indice del body de la word a partir del nombre de la misma
    pub fn encontrar_word(&self, nombre: &String) -> Result<usize, String> {
        for i in 0..self.words_usuarios.len() {
            let i_invertido: usize = self.words_usuarios.len() - 1 - i;
            let word_actual = &self.words_usuarios[i_invertido];
            if word_actual.get_nombre() == nombre {
                return Ok(i_invertido);
            }
        }
        Err(String::from("?"))
    }

    /// Encuentra el indice del body de la word a partir del nombre de la misma
    /// 
    /// La diferencia con ```encontrar_word()``` es que aquí empezamos la busqueda sin contar el elemento actual
    pub fn encontrar_word_para_armar_body(&self, nombre: &String) -> Result<usize, String> {
        for i in 1..self.words_usuarios.len() {
            let i_invertido: usize = self.words_usuarios.len() - 1 - i;
            let word_actual = &self.words_usuarios[i_invertido];
            if word_actual.get_nombre() == nombre {
                return Ok(i_invertido);
            }
        }
        Err(String::from("?"))
    }

    /// Ejecuta los tokens de restante
    pub fn ejecutar_tokens(&self, stack: &mut Stack) -> Result<Devolucion, String> {
        for token in &self.restante {
            self.ejecutar(token, stack)?;
        }
        Ok(Devolucion::Vacio)
    }

    /// Matchea cada token de restante con su ejecucion
    pub fn ejecutar(
        &self,
        token_a_ejecutar: &TokenParseo,
        stack: &mut Stack,
    ) -> Result<Devolucion, String> {
        let _ = match token_a_ejecutar {
            TokenParseo::Numero(nro) => funciones_stack::ejecutar_int(stack, *nro),
            TokenParseo::Texto(texto) => funciones_outup::ejecutar_punto_y_coma(stack, texto),
            TokenParseo::Ejecutable(string_ejecutable) => {
                let encontrado = self.encontrar_word_ejecutar(string_ejecutable, stack)?;
                if !encontrado {
                    self.matchear_ejecutable(string_ejecutable, stack)
                } else {
                    Ok(Devolucion::Vacio)
                }
            }
            TokenParseo::DentroIF(_) => self.ejecutar_if(token_a_ejecutar, stack),
            _ => return Err(String::from("ejecution-error")),
        };
        Ok(Devolucion::Vacio)
    }

    /// Busca el nombre de la word para ver si fue definida por el usuario
    /// 
    /// En caso de encontrarlo, ejecuta esa word y devuelve true
    /// En caso de no encontrarlo, devuelve false
    fn encontrar_word_ejecutar(
        &self,
        string_ejecutable: &String,
        stack: &mut Stack,
    ) -> Result<bool, String> {
        let indice_word = self.encontrar_word(string_ejecutable);
        match indice_word {
            Ok(indice) => {
                self.ejecutar_por_indice(&indice, stack)?;
            }
            Err(_) => {
                return Ok(false);
            }
        }
        Ok(true)
    }

    /// Ejecuta una word por el indice
    /// 
    /// Esta funcion es muy util cuando una word depende de otra
    /// 
    /// #Example:
    /// 
    /// : foo 5 ;
    /// : bar foo ;
    /// : foo 6 ;
    /// bar foo
    /// 
    ///  A partir del ejemplo, obtenemos:
    /// 
    /// ```forth bodys: [[Token(Numero(5))], [Indice(0)], [Token(Numero(6))]]```
    /// 
    /// Para ejecutar ```Indice(0)``` es donde se utiliza esta funcion ya que toma el body con ese indice
    /// y lo ejecuta (en este caso ```Token(Numero(5))```)
    fn ejecutar_por_indice(&self, indice: &usize, stack: &mut Stack) -> Result<Devolucion, String> {
        for tokens_ejecutar in self.bodys.get(*indice) {
            for token in tokens_ejecutar {
                match token {
                    ParametroBody::Token(token_actual) => {
                        self.ejecutar(token_actual, stack)?;
                    }
                    ParametroBody::Indice(indice) => {
                        self.ejecutar_por_indice(indice, stack)?;
                    }
                }
            }
        }
        Ok(Devolucion::Vacio)
    }

    /// Ejecuta if
    fn ejecutar_if(&self, token: &TokenParseo, stack: &mut Stack) -> Result<Devolucion, String> {
        let cond = stack.pop()?;
        let cond = matchear_devolucion_numero(cond)?;

        if let TokenParseo::DentroIF(vector) = token {
            for token_dentro_if in vector {
                match token_dentro_if {
                    TokenParseo::Numero(_) | TokenParseo::Texto(_) | TokenParseo::Ejecutable(_) => {
                        if cond == -1 {
                            self.ejecutar(token_dentro_if, stack)?;
                        }
                    }
                    TokenParseo::DentroIF(_) => {
                        if cond == -1 {
                            self.ejecutar_if(token_dentro_if, stack)?;
                        }
                    }
                    TokenParseo::DentroELSE(tokens_else) => {
                        if cond != -1 {
                            for token_else in tokens_else {
                                self.ejecutar(token_else, stack)?;
                            }
                        }
                    }
                    _ => {
                        return Err(String::from("invalid-word (llego a if un token invalido"));
                    }
                }
            }
            return Ok(Devolucion::Vacio);
        }
        Err(String::from(
            "parser-error (No llego un token if a ejecutar if)",
        ))
    }

    /// Matchea los simbolos con sus respectivas funciones
    fn matchear_ejecutable(
        &self,
        string_ejecutable: &String,
        stack: &mut Stack,
    ) -> Result<Devolucion, String> {
        match string_ejecutable.as_str() {
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
            _ => Err(String::from("?")),
        }
    }
}
