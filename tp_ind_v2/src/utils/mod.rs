use crate::devolucion::Devolucion;

pub mod funciones_stack;
//pub mod funciones_boolean;
pub mod funciones_aritmetica;
pub mod funciones_if;
pub mod funciones_outup;

pub fn matchear_devolucion_numero(posible_numero: Devolucion) -> Option<i16> {
    match posible_numero {
        Devolucion::Numero(nro) => Some(nro),
        Devolucion::Vacio => None,
    }
}
