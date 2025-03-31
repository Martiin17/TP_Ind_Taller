use crate::forth::Forth;
use crate::stack::Stack;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
pub struct Interprete {
    forth: Forth,
}

impl Interprete {
    pub fn new(forth: Forth) -> Self {
        Self { forth }
    }

    pub fn leer_archivo(&mut self, nombre_archivo: &str) -> io::Result<Vec<String>> {
        let path = Path::new(nombre_archivo);
        let file = File::open(path)?;
        let reader = io::BufReader::new(file);

        let mut resultado: Vec<String> = Vec::new();

        for linea in reader.lines() {
            let linea = linea?;
            for word in linea.split_whitespace() {
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

        while contador_local != leido.len() {
            if siguiente_body_name {
                if leido[contador_local].parse::<i16>().is_ok() {
                    return Err("invalid-word".to_string());
                }
                siguiente_body_name = false;
            }
            if leido[contador_local].contains(";") && leido[contador_local].len() > 1 {
                return Err("invalid-word".to_string());
            } else if leido[contador_local].contains(":") {
                if leido[contador_local].len() > 1 {
                    return Err("invalid-word".to_string());
                }
                siguiente_body_name = true;
            } else if leido[contador_local] == ".\"" {
                leido = self.armar_palabra(contador, leido)?;
            } else if leido[contador_local] == "IF" {
                leido = self.armar_if(contador, leido)?;
            }
            contador += 1;
            contador_local += 1;
        }
        Ok(())
    }

    pub fn armar_palabra<'a>(
        &self,
        contador: usize,
        leido: &'a mut Vec<String>,
    ) -> Result<&'a mut Vec<String>, String> {
        let mut string_especial = String::from(".\" ");

        for i in contador + 1..leido.len() {
            if leido[i].ends_with("\"") {
                string_especial.push_str(&leido[i]);

                let range = contador..=i;
                leido.drain(range);
                leido.insert(contador, string_especial);

                return Ok(leido);
            }
            string_especial.push_str(&leido[i]);
            string_especial.push_str(" ");
        }

        Err("Error al parsear .\"".to_string())
    }

    fn armar_if<'a>(
        &self,
        contador: usize,
        leido: &'a mut Vec<String>,
    ) -> Result<&'a mut Vec<String>, String> {
        let mut string_especial = String::from("");
        for i in contador..leido.len() {
            if leido[i] == "THEN" {
                string_especial.push_str(&leido[i]);
                let range = contador..=i;
                leido.drain(range);
                leido.insert(contador, string_especial);

                println!("{:?}", leido);
                return Ok(leido);
            }
            string_especial.push_str(&leido[i]);
            string_especial.push_str(" ");
        }
        Err("Error al parsear IF".to_string())
    }

    pub fn ejecutar_lectura(
        &mut self,
        leido: &Vec<String>,
        stack: &mut Stack,
    ) -> Result<(), String> {
        let mut word_name = String::new();
        let mut sigue_word_name = false;
        let mut completo_word = true;
        let mut en_word_body = false;

        for item in leido {
            if item == &(":".to_string()) {
                sigue_word_name = true;
                completo_word = false;
            } else if sigue_word_name {
                word_name = item.to_string();
                self.forth
                    .agregar_word_usuario(item.to_string(), Vec::new());
                sigue_word_name = false;
                en_word_body = true;
            } else if item == &(";".to_string()) {
                completo_word = true;
                en_word_body = false;
            } else if en_word_body {
                match self.forth.get_word_usuario(&word_name) {
                    Some(array) => array.push(item.to_string()),
                    None => println!("Error al parsear"),
                }
            } else {
                self.forth.ejecutar(item, stack)?;
            }
        }
        if !completo_word {
            return Err("?".to_string());
        }
        Ok(())
    }

    /* fn obtener_dicc(&mut self, name: &String) -> Option<&mut Vec<String>> {
        self.forth.get_word_usuario(name)
    } */

    pub fn escribir_stack(&self, stack: Stack) -> io::Result<()> {
        let mut archivo = File::create("stack.fth")?;

        for (i, &valor) in stack.vector.iter().enumerate() {
            if i > 0 {
                write!(archivo, " ")?;
            }
            write!(archivo, "{}", valor)?;
        }

        Ok(())
    }
}
