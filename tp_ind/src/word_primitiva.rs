/* use crate::{devolucion::Devolucion, funcion::Funcion, stack::Stack};

#[derive(Debug)]
pub struct WordPrimitiva {
    pub nombre: String,
    f: Funcion,
}

impl WordPrimitiva {
    pub fn new(nombre: String, f: Funcion) -> Self {
        Self { nombre, f }
    }

    /* pub fn ejecutar_func(&self, stack: &mut Stack, texto: &String, numero: i16) -> Result<Devolucion, String> {
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

            Funcion::StackYNumero(f) => f(stack, numero),
        }
    } */

    pub fn ejecutar_func_solo_stack(&self, stack: &mut Stack) -> Result<Devolucion, String> {
        match &self.f {
            Funcion::SoloStack(f) => f(stack),
            Funcion::SoloStackDevuelvoInt(f) => f(stack),
            _ => Err(String::from(
                "Se ejecuto la func solo stack lo cual es incorrecto",
            )),
        }
    }

    pub fn ejecutar_func__stack_y_texto(
        &self,
        stack: &mut Stack,
        texto: &String,
    ) -> Result<Devolucion, String> {
        if let Funcion::StackYTexto(f) = &self.f {
            let _ = f(stack, texto);
            Ok(Devolucion::Vacio)
        } else {
            Err(String::from(
                "Se ejecuto la func stack y texto lo cual es incorrecto",
            ))
        }
    }

    pub fn ejecutar_func_stack_e_int(
        &self,
        stack: &mut Stack,
        nro: i16,
    ) -> Result<Devolucion, String> {
        if let Funcion::StackYNumero(f) = &self.f {
            let _ = f(stack, nro);
            Ok(Devolucion::Vacio)
        } else {
            Err(String::from(
                "Se ejecuto la func stack e int lo cual es incorrecto",
            ))
        }
    }
}
 */