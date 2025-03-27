use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::forth::Forth;
use crate::stack::{self, Stack};
use crate::error_handler::ErrorHandler;
pub struct Interprete{
    forth: Forth,
}

impl Interprete{
    pub fn new(forth: Forth) -> Self {
        Self { forth: forth }
    }
    //IDEA --> Vector con todo lo que leimos y lo recorremos secuencial
    //Si hay :algo, algo; ==> error
    //Si el body se puede parsear a un nro ==> error

    pub fn leer_archivo(&mut self, nombre_archivo: &str) -> io::Result<Vec<String>> {
        let path = Path::new(nombre_archivo);
        let file = File::open(&path)?;
        let reader = io::BufReader::new(file);
    
        let mut resultado: Vec<String> = Vec::new();

        for linea in reader.lines() {
            let linea = linea?;
            for word in linea.split_whitespace(){
                resultado.push(word.to_string());
            }
        }
        Ok(resultado)
    }

    /* pub fn verificar_lectura(&self, leido: &Vec<String>) -> Result<(), String> {
        let mut siguiente_body_name = false;
        let mut dentro_string= false;
        let mut contador: usize = 0;
        let mut contador_local: usize = 0;

        for item in leido{
            if siguiente_body_name == true {
                if let Ok(_) = item.parse::<i16>(){
                    return Err("invalid-word".to_string());
                }
                siguiente_body_name = false;
            }
            if item.contains(";") && item.len() > 1 {
                return Err("invalid-word".to_string());
            }
            else if item.contains(":"){
                if item.len() > 1 {
                    return Err("invalid-word".to_string());
                }
                siguiente_body_name = true;
            }
        }
        Ok(())
    } */

    pub fn verificar_lectura(&self, mut leido: &mut Vec<String>) -> Result<(), String> {
        let mut siguiente_body_name = false;
        let mut contador: usize = 0;
        let mut contador_local: usize = 0;

        while contador_local != leido.len(){
            if siguiente_body_name == true {
                if let Ok(_) = &leido[contador_local].parse::<i16>(){
                    return Err("invalid-word".to_string());
                }
                siguiente_body_name = false;
            }
            if leido[contador_local].contains(";") && leido[contador_local].len() > 1 {
                return Err("invalid-word".to_string());
            }
            else if leido[contador_local].contains(":"){
                if leido[contador_local].len() > 1 {
                    return Err("invalid-word".to_string());
                }
                siguiente_body_name = true;
            } else if leido[contador_local] == ".\""{
                leido = self.armar_palabra(contador, leido)?;
            }
            contador += 1;
            contador_local += 1;
        }
        Ok(())
    }

    pub fn armar_palabra<'a>(&self, contador: usize, leido: &'a mut Vec<String>) -> Result<&'a mut Vec<String>, String> {
        /* let mut string_especial = String::from(".\" ");

        for i in contador+1..leido.len(){
            if leido[i].ends_with("\""){
                string_especial.push_str(&leido[i]);
                for j in contador..i{
                    println!("{}", j);
                    leido.remove(j);
                }
                leido.insert(contador, string_especial);
                println!("{:?}", leido);
                return Ok(leido);
            }
            string_especial.push_str(&leido[i]);
        }
        Err("Error al parsear .\"".to_string()) */

    let mut string_especial = String::from(".\" ");

    // Recorremos desde el siguiente índice al contador
        for i in contador + 1..leido.len() {
            if leido[i].ends_with("\"") {
                string_especial.push_str(&leido[i]); // Agregamos el último elemento que termina con comilla

                // Ahora eliminamos todos los elementos entre el contador y `i` (inclusive el último)
                let range = contador..=i; // Incluye el índice `i`
                leido.drain(range); // Eliminamos todos los elementos entre el rango

                // Insertamos la cadena especial en el lugar donde estaba el contador
                leido.insert(contador , string_especial);

                //println!("{:?}", leido); // Para ver el vector actualizado
                return Ok(leido);
            }
            string_especial.push_str(&leido[i]); // Agregamos los elementos intermedios
        }

        Err("Error al parsear .\"".to_string()) // Si no se encuentra una comilla de cierre
    }

    /* pub fn verificar_termina_string(self, leido: &Vec<String>, i: &usize) -> Result<usize, String> {
       for j in i..leido.len(){
        if &leido[j] == "\"" {
            return Ok(j);
           }
       }
       Err("texto no cerrado".to_string())
    }

    pub fn formar_string(&self, leido: &Vec<String>, i: &mut i32) -> Result<(), String> {
        for item in i..leido.len(){

        }
    } */

    pub fn ejecutar_lectura(&mut self, leido: &Vec<String>, stack: &mut Stack) -> Result<(), String> {
        let mut word_name = String::new();
        let mut sigue_word_name = false;
        let mut completo_word = true;
        let mut en_word_body = false;

        for item in leido{
            if item == &(":".to_string()){
                sigue_word_name = true;
                completo_word = false;
            } else if sigue_word_name == true{
                word_name = item.to_string();
                self.forth.agregar_word_usuario(item.to_string(), Vec::new());
                sigue_word_name = false;
                en_word_body = true;
            } else if item == &(";".to_string()){
                completo_word = true;
                en_word_body = false;
            } else if en_word_body == true{
                match self.forth.get_word_usuario(&word_name){
                    Some(array) => array.push(item.to_string()),
                    None => println!("Error al parsear"),
                }
            } else{
                self.forth.ejecutar(item, stack)?;
            }
        }
        if completo_word == false{
            return Err("?".to_string());
        }
        Ok(())
    }

    //FUNCIONA
    /* pub fn leer_archivo(&mut self, nombre_archivo: &str, stack: &mut Stack ) -> Result<(), ErrorHandler> {
        let path = Path::new(nombre_archivo);
        let file = File::open(&path).map_err(|e| ErrorHandler::ErrIo(e))?;
        let reader = io::BufReader::new(file);
    
        let mut word_name = String::new();
        let mut sigue_word_name = false;
        let mut en_word_body = false;
        let mut completo_word = true;
    
        for linea in reader.lines() {
            let linea = linea.map_err(|e| ErrorHandler::ErrIo(e))?;
            for contenido in linea.split_whitespace(){
                //println!("{}", contenido);
                let contenido_string = contenido.to_string();
                let aux = contenido.to_string();
                if contenido_string == ":".to_string(){
                    sigue_word_name = true;
                    en_word_body = true;
                    completo_word = false;
                } else if sigue_word_name == true {
                    word_name = contenido_string;
                    sigue_word_name = false;
                    //Lo consume
                    self.forth.agregar_word_usuario(aux, Vec::new());
                } else if en_word_body == true {
                    if contenido_string == ";".to_string(){
                        en_word_body = false;
                        completo_word = true;
                    } else {
                        match self.forth.get_word_usuario(&word_name){
                            Some(array) => array.push(contenido_string),
                            None => println!("Error al parsear"),
                        }
                    }
                } else {
                    match self.forth.ejecutar(&contenido.to_string(), stack){
                        Ok(_) => println!("Ejecuto ok"),
                        Err(e) => println!("{}", e),
                    }
                }
            }
        }
        if completo_word == false{
            match ErrorHandler::ErrString("invalid-word".to_string()){
                ErrorHandler::ErrString(error) => println!("{}", error),
                ErrorHandler::ErrIo(_) => println!("wrong error type"),
            }
        }
        Ok(())
    } */ 

    //Lo de arriba modularizado (no funca)
    /* pub fn leer_archivo(&mut self, nombre_archivo: &str, stack: &mut Stack ) -> Result<(), ErrorHandler> {
        let path = Path::new(nombre_archivo);
        let file = File::open(&path).map_err(|e| ErrorHandler::ErrIo(e))?;
        let reader = io::BufReader::new(file);
    
        let mut word_name = String::new();
        let mut sigue_word_name = false;
        let mut en_word_body = false;
        let mut completo_word = true;
    
        for linea in reader.lines() {
            let linea = linea.map_err(|e| ErrorHandler::ErrIo(e))?;
            let en_word_body = self.interpretar_lineas(linea, stack)?;
        }
        self.chequear_error_interpretar_lineas(en_word_body);
        Ok(())
    }  

    
    pub fn interpretar_lineas(&mut self, linea: String, stack: &mut Stack) -> Result<bool, ErrorHandler> {
        let mut word_name = String::new();
        let mut sigue_word_name = false;
        let mut en_word_body = false;
    
        for contenido in linea.split_whitespace(){
            let contenido_string = contenido.to_string();
            let aux = contenido.to_string();
            if contenido_string == ":".to_string(){
                sigue_word_name = true;
                en_word_body = true;
            } else if sigue_word_name == true {
                word_name = contenido_string;
                sigue_word_name = false;
                self.forth.agregar_word_usuario(aux, Vec::new());
            } else if en_word_body == true {
                if contenido_string == ";".to_string(){
                    en_word_body = false;
                } else {
                    match self.forth.get_word_usuario(&word_name){
                        Some(array) => array.push(contenido_string),
                        None => println!("Error al parsear"),
                    }
                }
            } else {
                match self.forth.ejecutar(&contenido.to_string(), stack){
                    Ok(_) => println!("Ejecuto ok"),
                    Err(e) => println!("{}", e),
                }
            }
        }
        Ok(en_word_body)
        //self.chequear_error_interpretar_lineas(en_word_body)
    }

    fn chequear_error_interpretar_lineas(&self, en_word_body: bool) -> Result<(), ErrorHandler>{
        if en_word_body == false{
            match ErrorHandler::ErrString("invalid-word".to_string()){
                ErrorHandler::ErrString(error) => println!("{}", error),
                ErrorHandler::ErrIo(_) => println!("wrong error type"),
            }
        }
        Ok(())
    } */

    /* pub fn leer_archivo(&mut self, nombre_archivo: &str, stack: &mut Stack ) -> io::Result<()> {
        let path = Path::new(nombre_archivo);
        let file = File::open(&path)?;
        let reader = io::BufReader::new(file);
    
        let mut word_name = String::new();
        //let mut words: HashMap<String, Vec<String>> = HashMap::new();
        let mut sigue_word_name = false;
        let mut en_word_body = false;
        let mut completo_word = true;
    
        for linea in reader.lines() {
            let linea = linea?;
            for contenido in linea.split_whitespace(){
                let contenido_string = contenido.to_string();
                let aux = contenido.to_string();
                if contenido_string == ":".to_string(){
                    sigue_word_name = true;
                    en_word_body = true;
                    completo_word = false;
                } else if sigue_word_name == true {
                    word_name = contenido_string;
                    sigue_word_name = false;
                    //Lo consume
                    self.forth.agregar_word_usuario(aux, Vec::new());
                } else if en_word_body == true {
                    if contenido_string == ";".to_string(){
                        en_word_body = false;
                        completo_word = true;
                    } else {
                        match self.forth.get_word_usuario(&word_name){
                            Some(array) => array.push(contenido_string),
                            None => println!("Error al parsear"),
                        }
                    }
                } else {
                    match self.forth.ejecutar(&contenido.to_string(), stack){
                        Ok(_) => println!("Ejecuto ok"),
                        Err(e) => println!("{}", e),
                    }
                }
            }
            if completo_word == false{
                return Err("invalid-word".to_string());
            }
        }
        Ok(())
    } */
}