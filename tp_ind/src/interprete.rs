use std::env;

use crate::stack::{self, Stack};
use crate::{
    devolucion::Devolucion, forth::Forth, token_parseo:: TokenParseo,
    word_usuario::WordUsuario,
};


pub fn armar_words_usuario<'a>(forth: &mut Forth<'a>, tokens: &'a Vec<TokenParseo>) -> Result<Devolucion, String>{
    let mut en_armado_word = false;
    let mut nombre_word_actual = String::new();
    for token in tokens{
        if en_armado_word {
            let mut word_actual= forth.get_word_usuario_mut(&nombre_word_actual)?;
            match token{
                TokenParseo::SimboloInicioWord(_) => return Err(String::from("parser error (simbolo inicio)")),
                TokenParseo::SimboloFinWord(_) => en_armado_word = false,
                TokenParseo::Simbolo(_) => {},
                _ => word_actual.agregar_elemento(token)
            }
        }else{
            match token {
                TokenParseo::WordName(nombre) => {
                    nombre_word_actual = nombre.to_string();
                    en_armado_word = true;
                    let mut word_actual= forth.get_word_usuario_mut(&nombre_word_actual);
                    match word_actual{
                        Ok(word_actual) => word_actual.get_body().clear(),
                        Err(_) => forth.words_usuarios.push(WordUsuario::new(nombre.to_string())),
                    }
                }
                TokenParseo::SimboloInicioWord(_) => {}
                TokenParseo::SimboloFinWord(_) => return Err(String::from("parser error (simbolo fin)")),
                TokenParseo::Simbolo(_) => {},
                _ => forth.restante.push(token),
            }
        }
    }
    Ok(Devolucion::Vacio)
}

pub fn interpretar_parametros() -> (usize, String){
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("No se llamo al proyecto con un formato invalido");
        std::process::exit(1);
    }

    let ruta_fth = &args[1];

    let mut stack_size: usize = 128 * 1024; // 128 KB

    if args.len() >= 3 {
        let arg2 = &args[2];
        if let Some(size_str) = arg2.strip_prefix("stack-size=") {
            match size_str.parse::<usize>() {
                Ok(size) => {
                    stack_size = size * 1024;
                }
                Err(_) => {
                    println!("El valor de stack-size debe ser un nro entero");
                    std::process::exit(1);
                }
            }
        } else {
            println!("El segundo argumento debe tener el formato stack-size={{número}}");
            std::process::exit(1);
        }
    }
    let capacidad_stack: usize = stack_size / std::mem::size_of::<i16>();
    (capacidad_stack, ruta_fth.to_string())
}

use std::fs::File;
use std::io::{self, Write};


pub fn escribir_stack(stack: &Stack) -> io::Result<()> {
    let mut archivo = File::create("stack.fth")?; // crea o sobrescribe el archivo

    // Convertimos cada número a string y los separamos con espacios
    let contenido = stack.vector
        .iter()
        .rev()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(" ");

    // Escribimos al archivo
    writeln!(archivo, "{}", contenido)?;
    Ok(())
}

