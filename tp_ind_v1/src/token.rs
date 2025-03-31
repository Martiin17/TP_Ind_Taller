use crate::word_usuario::WordUsuario;

pub enum Token{
    operador_primitivo(String),
    word_usuario(WordUsuario),
    especial(String), //Capaz no lo uso
}