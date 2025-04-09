use crate::devolucion::Devolucion;

pub mod funciones_stack;
pub mod funciones_logicas;
pub mod funciones_aritmetica;
pub mod funciones_if;
pub mod funciones_outup;

pub fn matchear_devolucion_numero(posible_numero: Devolucion) -> Result<i16, String> {
    match posible_numero {
        Devolucion::Numero(nro) => Ok(nro),
        _ => Err(String::from("cant pop from stack")),
    }
}
