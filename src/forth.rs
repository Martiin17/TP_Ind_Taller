use std::collections::HashMap;

use crate::stack::Stack;
use crate::utils::{funciones_aritmetica, funciones_boolean, funciones_stack, funciones_outup, funciones_if};
pub struct Forth{
    //diccionario: HashMap<String, fn(&mut Stack) -> Result<(),String>>,
    pub diccionario_usuario: HashMap<String, Vec<String>>,
}

impl Forth{
    pub fn new() -> Self{
        Self { 
            //diccionario: HashMap::new(), 
            diccionario_usuario: HashMap::new(),
        }
    }

    pub fn agregar_word_usuario(&mut self, nombre:String, v: Vec<String>){
        self.diccionario_usuario.insert(nombre, v); //Verificar que onda si tiene error
    }

    pub fn get_word_usuario(&mut self, nombre:&String) -> Option<&mut Vec<String>> { 
        self.diccionario_usuario.get_mut(nombre) //Verificar que onda si tiene error
    }

    /* fn verificar_punto_y_comillas(&self, palabra: &String, stack: &mut Stack) -> Result<bool, String>{
        let mut se_matcheo = false;
        if palabra.starts_with(".\" ") && palabra.ends_with("\"") {
            let contenido = &palabra[3..palabra.len() - 1].to_string();
            funciones_outup::ejecutar_punto_y_coma(stack, contenido)?; 
            se_matcheo = true;
        }
        Ok(se_matcheo) 
    } */


    pub fn ejecutar_primitivas(&self, palabra: &String, stack: &mut Stack) -> Result<(), String>{

        //println!("dentro de ejecutar_primitivas: {}", palabra);
        if palabra.starts_with(".\" ") && palabra.ends_with("\"") {
            let contenido = &palabra[3..palabra.len() - 1].to_string();
            funciones_outup::ejecutar_punto_y_coma(stack, contenido) 
        } else if palabra.starts_with("IF"){
            funciones_if::ejecutar_if(stack, palabra, &self)
        } else{
            match palabra.as_str(){
                "+" => funciones_aritmetica::ejecutar_suma(stack),
                "-" => funciones_aritmetica::ejecutar_resta(stack),
                "*" => funciones_aritmetica::ejecutar_multiplicacion(stack),
                "/" => funciones_aritmetica::ejecutar_division(stack),
                "=" => funciones_boolean::ejecutar_igual(stack),
                ">" => funciones_boolean::ejecutar_mayor(stack),
                "<" => funciones_boolean::ejecutar_menor(stack),
                "AND" => funciones_boolean::ejecutar_and(stack),
                "NOT" => funciones_boolean::ejecutar_not(stack),
                "OR" => funciones_boolean::ejecutar_or(stack),
                "DUP"=> funciones_stack::ejecutar_dup(stack),
                "DROP"=> funciones_stack::ejecutar_drop(stack),
                "SWAP"=> funciones_stack::ejecutar_swap(stack),
                "OVER"=> funciones_stack::ejecutar_over(stack),
                "ROT"=> funciones_stack::ejecutar_rot(stack),
                "."=> funciones_outup::ejecutar_punto(stack),
                "CR"=> funciones_outup::ejecutar_cr(stack),
                "EMIT"=> funciones_outup::ejecutar_emit(stack),
                _ => Err("invalid-word".to_string()),
            }
        }
    }

    pub fn ejecutar(&self, nombre: &String, stack: &mut Stack) -> Result<(), String> {
        //println!("en ejecutar {}", nombre);
        if let Ok(numero) = nombre.parse::<i16>() {
            let resultado = stack.push(numero);
            self.manejar_error(resultado);
            return Ok(());
        }
        //println!("en ejecutar {:?}", self.diccionario_usuario);
        if let Some(vector) = self.diccionario_usuario.get(nombre) {
            //println!("entre al if");
            let indice = vector.len();
            let mut i: usize = 0;
            for item in vector{
                i += 1;
                let _ = self.ejecutar(item, stack);
            }

            //Se ejecuto todo correctamente
            if indice == i{
                return Ok(());
            }
        }
        
        //println!("dentro de ejecutar: {}", nombre);
        //let resultado = self.ejecutar_primitivas(nombre, stack);
        //self.manejar_error(resultado);

        match self.ejecutar_primitivas(nombre, stack){
            Ok(_) => return Ok(()),
            Err(e) => return Err(e),
        }

        //Err(String::from("?"))
    }

    pub fn manejar_error(&self, variable: Result<(), String>) {
        match variable {
            Ok(_) => (),
            Err(e) => println!("{}", e),
        }
    }

}