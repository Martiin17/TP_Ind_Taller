use crate::token_parseo::TokenParseo;

#[derive(Debug)]
pub struct WordUsuario<'a> {
    nombre: String,
    body: Vec<&'a TokenParseo>,
}

impl <'a> WordUsuario<'a>{
    pub fn new(nombre: String) -> Self {
        Self {
            nombre,
            body: Vec::new(),
        }
    }

    pub fn get_nombre(&self) -> &String {
        &self.nombre
    }

    pub fn set_nombre(&mut self, nuevo_nombre: String) {
        self.nombre = nuevo_nombre;
    }

    pub fn agregar_elemento(&mut self, elem: &'a TokenParseo) {
        self.body.push(elem)
    }

    pub fn get_body(&mut self) -> &mut Vec<&'a TokenParseo> {
        &mut self.body
    }
}
