use crate::stack::Stack;
pub struct WordPrimitiva{
    Nombre: String,
    Funcion: fn(&mut Stack) -> Result<(), String>, //A los nros sueltos lo consideramos caso especial cuando ejecutemos
}


impl WordPrimitiva{
    pub fn new(nombre: String, funcion: fn(&mut Stack)-> Result<(), String>) -> Self {
        WordPrimitiva { Nombre: nombre, Funcion: funcion }
    }
}