use crate::{devolucion::Devolucion, stack::Stack};

#[derive(Debug)]
/* pub enum Funcion {
    SoloStack(fn(&mut Stack) -> Result<(), String>),
    SoloStackDevuelvoInt(fn(&mut Stack) -> Result<i16, String>),
    StackYTexto(fn(&mut Stack, &String) -> Result<(), String>),
} */

pub enum Funcion {
    SoloStack(fn(&mut Stack) -> Result<Devolucion, String>),
    SoloStackDevuelvoInt(fn(&mut Stack) -> Result<Devolucion, String>),
    StackYTexto(fn(&mut Stack, &String) -> Result<Devolucion, String>),
}