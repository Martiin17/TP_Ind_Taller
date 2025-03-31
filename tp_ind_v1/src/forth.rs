use std::collections::HashMap;

use crate::stack::Stack;
use crate::utils::{
    funciones_aritmetica, funciones_boolean, funciones_if, funciones_outup, funciones_stack,
};
pub struct Forth {
    pub diccionario_usuario: HashMap<String, Vec<String>>,
}

impl Forth {
    pub fn new() -> Self {
        Self {
            diccionario_usuario: HashMap::new(),
        }
    }

    pub fn agregar_word_usuario(&mut self, nombre: String, v: Vec<String>) {
        self.diccionario_usuario.insert(nombre, v);
    }

    pub fn get_word_usuario(&mut self, nombre: &String) -> Option<&mut Vec<String>> {
        self.diccionario_usuario.get_mut(nombre)
    }

    /* pub fn es_word_usuario(&mut self, nombre: &String) -> bool {
        match self.diccionario_usuario.get_mut(nombre) {
            Some(_) => true,
            None => false,
        }
    } */

    pub fn ejecutar_primitivas(&self, palabra: &String, stack: &mut Stack) -> Result<(), String> {
        if palabra.starts_with(".\" ") && palabra.ends_with("\"") {
            let contenido = &palabra[3..palabra.len() - 1].to_string();
            funciones_outup::ejecutar_punto_y_coma(stack, contenido)
        } else if palabra.starts_with("IF") {
            funciones_if::ejecutar_if(stack, palabra, self)
        } else {
            match palabra.as_str() {
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
                "DUP" => funciones_stack::ejecutar_dup(stack),
                "DROP" => funciones_stack::ejecutar_drop(stack),
                "SWAP" => funciones_stack::ejecutar_swap(stack),
                "OVER" => funciones_stack::ejecutar_over(stack),
                "ROT" => funciones_stack::ejecutar_rot(stack),
                "." => funciones_outup::ejecutar_punto(stack),
                "CR" => funciones_outup::ejecutar_cr(stack),
                "EMIT" => funciones_outup::ejecutar_emit(stack),
                _ => Err("invalid-word".to_string()),
            }
        }
    }

    pub fn ejecutar(&self, nombre: &String, stack: &mut Stack) -> Result<(), String> {
        if let Ok(numero) = nombre.parse::<i16>() {
            stack.push(numero)?;
            return Ok(());
        }

        if let Some(vector) = self.diccionario_usuario.get(nombre) {
            let indice = vector.len();
            let mut i: usize = 0;
            for item in vector {
                i += 1;
                let _ = self.ejecutar(item, stack);
            }

            //Se ejecuto todo correctamente
            if indice == i {
                return Ok(());
            }
        }

        match self.ejecutar_primitivas(nombre, stack) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
