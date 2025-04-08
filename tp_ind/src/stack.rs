use crate::devolucion::Devolucion;

#[derive(Debug)]
pub struct Stack {
    pub vector: Vec<i16>,
    cap_maxima: usize,
}

impl Stack {
    pub fn new(cap_maxima: usize) -> Stack {
        Stack {
            vector: Vec::new(),
            cap_maxima,
        }
    }

    pub fn obtener_capacidad(&self) -> usize {
        self.vector.len()
    }

    pub fn push(&mut self, elem: i16) -> Result<Devolucion, String> {
        if self.obtener_capacidad() < self.cap_maxima {
            self.vector.push(elem);
            Ok(Devolucion::Vacio)
        } else {
            Err("Stack-overflow".to_string())
        }
    }

    pub fn pop(&mut self) -> Result<Devolucion, String> {
        if let Some(elem) = self.vector.pop() {
            Ok(Devolucion::Numero(elem))
        } else {
            Err("Stack-underflow".to_string())
        }
    }
}
