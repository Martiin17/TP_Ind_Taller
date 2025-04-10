//! Contiene las funciones de interpretacion de Tokens

use std::{env, vec};

use crate::parametro_body::ParametroBody;
use crate::stack::Stack;
use crate::{
    devolucion::Devolucion, forth::Forth, token_parseo::TokenParseo, word_usuario::WordUsuario,
};

use std::fs::File;
use std::io::{self, Write};

/// Forma los bodys de Forth
/// 
/// #Parametros
/// 
/// forth --> Recibe un tipo Forth mutable (donde se pondra el body conseguido)
/// 
/// tokens --> Recibe la lista de TokenParseo a agregar en forth
pub fn formar_bodys(forth: &mut Forth, tokens: Vec<TokenParseo>) -> Result<Devolucion, String> {
    let mut en_armado_word = false;
    let mut body_actual: Vec<ParametroBody> = vec![];
    let mut contador_words: usize = 0;
    for token in tokens {
        if en_armado_word {
            match token {
                TokenParseo::SimboloInicioWord(_) => {
                    return Err(String::from("parser-error (simbolo inicio)"));
                }
                TokenParseo::SimboloFinWord(_) => {
                    en_armado_word = false;
                    forth.bodys.push(body_actual);
                    body_actual = vec![];
                }
                TokenParseo::Simbolo(_) => {}
                TokenParseo::DentroIF(vector) => {
                    let vector_parseado = caso_if(vector);
                    let nuevo_token = TokenParseo::DentroIF(vector_parseado);
                    body_actual.push(ParametroBody::Token(nuevo_token));
                }
                TokenParseo::WordName(nombre) => {
                    let indice = forth.encontrar_word_para_armar_body(&nombre)?;
                    body_actual.push(ParametroBody::Indice(indice));
                }
                TokenParseo::IF | TokenParseo::THEN | TokenParseo::ELSE => {}
                _ => body_actual.push(ParametroBody::Token(token)),
            }
        } else {
            match token {
                TokenParseo::WordName(nombre) => {
                    forth
                        .words_usuarios
                        .push(WordUsuario::new(nombre, contador_words));
                    contador_words += 1;
                    en_armado_word = true;
                }
                TokenParseo::SimboloInicioWord(_) => {}
                TokenParseo::SimboloFinWord(_) => {
                    return Err(String::from("parser-error (simbolo fin)"));
                }
                TokenParseo::Simbolo(_) => {}
                _ => forth.restante.push(token),
            }
        }
    }
    Ok(Devolucion::Vacio)
}

/// Se encarga de ejecutar los casos de TokenParseo::DentroIF
fn caso_if(dentro_token: Vec<TokenParseo>) -> Vec<TokenParseo> {
    let mut nuevo_vector: Vec<TokenParseo> = vec![];
    for elem in dentro_token {
        match elem {
            TokenParseo::SimboloInicioWord(_) => continue,
            TokenParseo::SimboloFinWord(_) => continue,
            TokenParseo::Simbolo(_) => continue,
            TokenParseo::IF => continue,
            TokenParseo::ELSE => continue,
            TokenParseo::THEN => continue,
            TokenParseo::DentroIF(token_parseos) => {
                let sub_nivel = caso_if(token_parseos);
                let nuevo_token = TokenParseo::DentroIF(sub_nivel);
                nuevo_vector.push(nuevo_token);
            }
            TokenParseo::DentroELSE(token_parseos) => {
                let sub_nivel = caso_if(token_parseos);
                let nuevo_token = TokenParseo::DentroELSE(sub_nivel);
                nuevo_vector.push(nuevo_token);
            }
            _ => nuevo_vector.push(elem),
        }
    }
    nuevo_vector
}

/// Interpreta los parametros de ejecucion
pub fn interpretar_parametros() -> (usize, String) {
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


/// Escribe el contenido del stack en "stack.fth"
pub fn escribir_stack(stack: &Stack) -> io::Result<()> {
    let mut archivo = File::create("stack.fth")?;

    let contenido = stack
        .vector
        .iter()
        .rev()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(" ");

    writeln!(archivo, "{}", contenido)?;
    Ok(())
}
