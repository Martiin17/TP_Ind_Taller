use crate::devolucion::Devolucion;

/// struct Stack
/// 
/// Representa el Stack utilizado en Forth 79
#[derive(Debug)]
pub struct Stack {
    pub vector: Vec<i16>,
    cap_maxima: usize,
}

impl Stack {
    /// Se crea un stack con una capacidad y un vector vacio
    pub fn new(cap_maxima: usize) -> Stack {
        Stack {
            vector: Vec::new(),
            cap_maxima,
        }
    }

    /// Devuelve la capacidad que tiene el stack
    pub fn obtener_capacidad(&self) -> usize {
        self.vector.len()
    }

    /// Pushea un elemento al stack
    pub fn push(&mut self, elem: i16) -> Result<Devolucion, String> {
        if self.obtener_capacidad() < self.cap_maxima {
            self.vector.push(elem);
            Ok(Devolucion::Vacio)
        } else {
            Err("Stack-overflow".to_string())
        }
    }

    /// Quita el ultimo elemento del stack y lo devuelve
    pub fn pop(&mut self) -> Result<Devolucion, String> {
        if let Some(elem) = self.vector.pop() {
            Ok(Devolucion::Numero(elem))
        } else {
            Err("Stack-underflow".to_string())
        }
    }
}
