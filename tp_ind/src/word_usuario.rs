#[derive(Debug, PartialEq)]
pub struct WordUsuario {
    nombre: String,
    indice: usize,
}

impl WordUsuario{
    pub fn new(nombre: String, indice: usize) -> Self {
        Self {
            nombre,
            indice,
        }
    }

    pub fn get_nombre(&self) -> &String {
        &self.nombre
    }

    pub fn get_indice(&self) -> &usize {
        &self.indice
    }
}
