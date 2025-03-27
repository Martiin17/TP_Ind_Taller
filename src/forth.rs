use std::collections::HashMap;

use crate::stack::Stack;
use crate::utils::{funciones_aritmetica, funciones_boolean, funciones_stack, funciones_outup};
pub struct Forth{
    //diccionario: HashMap<String, fn(&mut Stack) -> Result<(),String>>,
    diccionario_usuario: HashMap<String, Vec<String>>,
}

impl Forth{
    pub fn new() -> Self{
        Self { 
            //diccionario: HashMap::new(), 
            diccionario_usuario: HashMap::new(),
        }
    }

    /* pub fn agregar_word_primitiva(&mut self, nombre:&str, f: fn(&mut Stack) -> Result<(),String>){
        self.diccionario.insert(nombre.to_string(), f); //Verificar que onda si tiene error
    } */

    /* pub fn agregar_word_usuario(&mut self, nombre:&str, v: Vec<String>){
        self.diccionario_usuario.insert(nombre.to_string(), v); //Verificar que onda si tiene error
    } */

    pub fn agregar_word_usuario(&mut self, nombre:String, v: Vec<String>){
        self.diccionario_usuario.insert(nombre, v); //Verificar que onda si tiene error
    }

    pub fn get_word_usuario(&mut self, nombre:&String) -> Option<&mut Vec<String>> { 
        self.diccionario_usuario.get_mut(nombre) //Verificar que onda si tiene error
    }

    /* pub fn inicializar(&mut self){
        self.agregar_word_primitiva("+", funciones_aritmetica::ejecutar_suma);
        self.agregar_word_primitiva("-", funciones_aritmetica::ejecutar_resta);
        self.agregar_word_primitiva("*", funciones_aritmetica::ejecutar_multiplicacion);
        self.agregar_word_primitiva("/", funciones_aritmetica::ejecutar_division);

        self.agregar_word_primitiva("=", funciones_boolean::ejecutar_igual);
        self.agregar_word_primitiva(">", funciones_boolean::ejecutar_mayor);
        self.agregar_word_primitiva("<", funciones_boolean::ejecutar_menor);
        self.agregar_word_primitiva("AND", funciones_boolean::ejecutar_and);
        self.agregar_word_primitiva("OR", funciones_boolean::ejecutar_or);
        self.agregar_word_primitiva("NOT", funciones_boolean::ejecutar_not);

        self.agregar_word_primitiva("DUP", funciones_stack::ejecutar_dup);
        self.agregar_word_primitiva("DROP", funciones_stack::ejecutar_drop);
        self.agregar_word_primitiva("SWAP", funciones_stack::ejecutar_swap);
        self.agregar_word_primitiva("OVER", funciones_stack::ejecutar_over);
        self.agregar_word_primitiva("ROT", funciones_stack::ejecutar_rot);

        self.agregar_word_primitiva(".", funciones_outup::ejecutar_punto);
        self.agregar_word_primitiva("CR", funciones_outup::ejecutar_cr);
        self.agregar_word_primitiva("EMIT", funciones_outup::ejecutar_emit);
        //self.agregar_word_primitiva(".'{palabra}'", funciones_outup::ejecutar_punto_y_coma);

    } */


    pub fn ejecutar_primitivas(&self, palabra: &String, stack: &mut Stack) -> Result<(), String>{

        if palabra.starts_with(".\" ") && palabra.ends_with("\"") {
            // Obtener el contenido entre las comillas
            let contenido = &palabra[3..palabra.len() - 1].to_string();
            funciones_outup::ejecutar_punto_y_coma(stack, contenido) 
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
        if let Ok(numero) = nombre.parse::<i16>() {
            let resultado = stack.push(numero);
            self.manejar_error(resultado);
            return Ok(());
        }

        if let Some(vector) = self.diccionario_usuario.get(nombre) {
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

        let resultado = self.ejecutar_primitivas(nombre, stack);
        self.manejar_error(resultado);

        Err(String::from("?"))
    }

    pub fn manejar_error(&self, variable: Result<(), String>) {
        match variable {
            Ok(_) => (),
            Err(e) => println!("{}", e),
        }
    }

    /* pub fn ejecutar(&mut self, nombre: &String) -> Result<(), String> {
        if let Ok(numero) = nombre.parse::<i16>() {
            self.stack.push(numero)?;
            return Ok(());
        }

        if let Some(comando) = self.diccionario.get(nombre) {
            comando(&mut self.stack)?;
            return Ok(());
        }

        if let Some(vector) = self.diccionario_usuario.get(nombre) {
            let indice = vector.len();
            let mut i: usize = 0;
            for item in vector{
                i += 1;
                if let Some(funcion) = self.diccionario.get(item){
                    funcion(&mut self.stack)?;
                } else if let Ok(numero) = item.parse::<i16>() {
                    self.stack.push(numero)?;
                }
            }

            //Se ejecuto todo correctamente
            if indice == i{
                return  Ok(());
            }
        }

        Err(String::from("?"))
    } */
}