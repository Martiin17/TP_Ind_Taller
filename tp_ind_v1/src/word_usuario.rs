/* #[derive(Debug)]
pub struct WordUsuario {
    nombre: String,
    body: Vec<String>,
}

impl WordUsuario {
    pub fn new(nombre: String) -> Self {
        Self {
            nombre,
            body: Vec::new(),
        }
    }

    pub fn get_nombre(&self) -> &String {
        &self.nombre
    }

    pub fn agregar_elemento(&mut self, elem: String) {
        self.body.push(elem)
    }

    pub fn get_body(&self) -> &Vec<String> {
        &self.body
    }
}
 */