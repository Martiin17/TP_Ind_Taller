use crate::{devolucion::Devolucion, funcion::Funcion, stack::Stack};

#[derive(Debug)]
pub struct WordPrimitiva{
    nombre: String,
    f: Funcion,
}

impl WordPrimitiva{

    pub fn new(nombre: String, f: Funcion) -> Self{
        Self{ nombre, f}
    }

    pub fn ejecutar_func(&self, stack: &mut Stack, texto: &String) -> Result<Devolucion, String> {
        match &self.f {
            Funcion::SoloStack(f) => f(stack),
            
            Funcion::StackYTexto(f) => {
                //Si va Option<&String>
                /* if let Some(t) = texto {
                    f(stack, t)
                } else {
                    Err("Falta el texto para ejecutar la función StackYTexto".to_string())
                } */
                f(stack, texto)
            }

            Funcion::SoloStackDevuelvoInt(f) => f(stack),  
        }
    }
}