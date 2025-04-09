use tp_ind::{forth::Forth, interprete::{escribir_stack, formar_bodys}, parametro_body::ParametroBody, parser::Parser, stack::Stack, token_parseo::TokenParseo, word_usuario::WordUsuario};

use std::fs::File;
use std::io::{Write, Read};

const CAPACIDAD_STACK: usize = 65536; //1024 * 128 * 1/2
const RUTA_ARCHIVO: &str = "probando.fth";

pub fn set_up() -> Result<(), String>{
    let mut stack_test = Stack::new(CAPACIDAD_STACK);
    let mut parser_test = Parser::new();
    let mut forth_test = Forth::new();

    let leido = parser_test.leer_archivo(RUTA_ARCHIVO)
    .map_err(|e| format!("Error {} al leer {}", e, RUTA_ARCHIVO))?;

    let tokens = parser_test.parseo(&leido)?;
    parser_test.tokens = tokens;


    let _ = formar_bodys(&mut forth_test, parser_test.tokens)?;

    let _ = forth_test.ejecutar_tokens(&mut stack_test)?;

    let _ = escribir_stack(&stack_test)
    .map_err(|e| format!("Error {} al escribir en stack.fth", e))?;

    Ok(())

}

pub fn leer_archivo_y_almacenar_parser() -> Result<Vec<String>, String>{
    let parser_test = Parser::new();
    parser_test.leer_archivo(RUTA_ARCHIVO)
    .map_err(|e| format!("Error {} al leer {}", e, RUTA_ARCHIVO))
}

pub fn formar_tokens(leido: &Vec<String>) -> Result<Vec<TokenParseo>, String>{
    let parser_test = Parser::new();
    let tokens = parser_test.parseo(&leido)?;
    Ok(tokens)
}

pub fn crear_word_usuario<'a>(tokens: Vec<TokenParseo>) -> Result<(Vec<Vec<ParametroBody>>, Vec<WordUsuario>), String> {
    let mut forth_test = Forth::new();
    let _ = formar_bodys(&mut forth_test, tokens)?;
    Ok((forth_test.bodys, forth_test.words_usuarios))
}

fn crear_texto_a_imprimir(texto: &str) -> Vec<String>{
    let texto_separado = texto.split_whitespace();
    let mut resultado: Vec<String> = vec![];
    for palabra in texto_separado{
        resultado.push(String::from(palabra));
    }
    resultado
}

pub fn escribir_en_archivo(texto: &str) -> Result<(), String> {
    let mut archivo = File::create(RUTA_ARCHIVO)
    .map_err(|e| format!("Error {} al escribir en {}", e, RUTA_ARCHIVO))?;

    let texto_a_escribir = crear_texto_a_imprimir(&texto);
    for (i, linea) in texto_a_escribir.iter().enumerate() {
        if i > 0 {
            write!(archivo, " ")
            .map_err(|e| format!("Error {} al escribir en {}", e, RUTA_ARCHIVO))?;
        }
        write!(archivo, "{}", linea)
        .map_err(|e| format!("Error {} al escribir en {}", e, RUTA_ARCHIVO))?;
    }

    Ok(())
}

pub fn leer_stack() -> Result<Vec<i16>, String> {
    let mut archivo = File::open("stack.fth")
    .map_err(|e| format!("Error {} al leer stack.fth", e))?;

    let mut contenido = String::new();

    archivo.read_to_string(&mut contenido)
    .map_err(|e| format!("Error {} al leer stack.fth", e))?;

    let numeros = contenido
        .split_whitespace()
        .filter_map(|s| s.parse::<i16>().ok())
        .collect();

    Ok(numeros)
}