#[derive(Debug, PartialEq)]

/// struct WordUsuario
/// 
/// Representa una word de usuario de Forth 79
/// 
/// Posee 2 atributos: un nombre y un indice
/// 
/// El indice concuerda con su posicion en forth.word_usuarios
pub struct WordUsuario {
    nombre: String,
    indice: usize,
}

impl WordUsuario {
    /// Se crea una nueva WordUsuario
    pub fn new(nombre: String, indice: usize) -> Self {
        Self { nombre, indice }
    }
    
    /// Se obtien el nombre de una WordUsuario
    pub fn get_nombre(&self) -> &String {
        &self.nombre
    }

    /// Se obtien el indice de una WordUsuario
    pub fn get_indice(&self) -> &usize {
        &self.indice
    }
}
