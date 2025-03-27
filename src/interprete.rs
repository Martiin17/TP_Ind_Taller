use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::forth::Forth;
use crate::stack::{self, Stack};
use crate::error_handler::ErrorHandler;
use crate::word_primitiva;
pub struct Interprete{
    forth: Forth,
}

impl Interprete{
    pub fn new(forth: Forth) -> Self {
        Self { forth: forth }
    }

    pub fn leer_archivo(&mut self, nombre_archivo: &str) -> io::Result<Vec<String>> {
        let path = Path::new(nombre_archivo);
        let file = File::open(&path)?;
        let reader = io::BufReader::new(file);
    
        let mut resultado: Vec<String> = Vec::new();

        for linea in reader.lines() {
            let linea = linea?;
            for word in linea.split_whitespace(){
                let mut word_formateada = word.to_string();
                word_formateada = word_formateada.to_uppercase();
                resultado.push(word_formateada);
            }
        }
        Ok(resultado)
    }

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
            } else if leido[contador_local] == "IF"{
                leido = self.armar_if(contador, leido)?;
            }
            contador += 1;
            contador_local += 1;
        }
        Ok(())
    }

    pub fn armar_palabra<'a>(&self, contador: usize, leido: &'a mut Vec<String>) -> Result<&'a mut Vec<String>, String> {
        let mut string_especial = String::from(".\" ");

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
            string_especial.push_str(" ");
        }

        Err("Error al parsear .\"".to_string()) // Si no se encuentra una comilla de cierre
    }

    fn armar_if<'a>(&self, contador: usize, leido: &'a mut Vec<String>) -> Result<&'a mut Vec<String>, String> {
        let mut string_especial = String::from("");
        for i in contador..leido.len() {
            if leido[i] == "THEN" {
                //println!("llegue");
                string_especial.push_str(&leido[i]);
                let range = contador..=i; 
                leido.drain(range);
                leido.insert(contador , string_especial);

                println!("{:?}", leido); // Para ver el vector actualizado
                return Ok(leido);
            }
            //println!("{}", contador);
            string_especial.push_str(&leido[i]);
            string_especial.push_str(" ");
        }
        Err("Error al parsear IF".to_string())
    }

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
                //println!("En ejecutar_lectura {}", item);
                //println!("{:?}", self.forth.diccionario_usuario);
                self.forth.ejecutar(item, stack)?;
            }
        }
        if completo_word == false{
            //println!("Error en ejecutar_lectura");
            return Err("?".to_string());
        }
        Ok(())
    }
}