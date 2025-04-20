#![allow(dead_code)]

use tp_ind::{
    forth::Forth, interprete::formar_bodys, parametro_body::ParametroBody, parser::Parser,
    stack::Stack, token_parseo::TokenParseo, word_usuario::WordUsuario,
};

use std::fs::File;
use std::io::{Read, Write};

pub const CAPACIDAD_STACK: usize = 512; // 1024/2
const RUTA_ARCHIVO: &str = "probando.fth";

pub fn set_up<W: Write>(salida: &mut W, capacidad_stack: usize) -> Result<Stack, String> {
    let mut stack_test = Stack::new(capacidad_stack);
    let mut parser_test = Parser::default();
    let mut forth_test = Forth::default();

    let leido = parser_test
        .leer_archivo(RUTA_ARCHIVO)
        .map_err(|e| format!("Error {} al leer {}", e, RUTA_ARCHIVO))?;

    let tokens = parser_test.parseo(&leido)?;
    parser_test.tokens = tokens;

    let _ = formar_bodys(&mut forth_test, parser_test.tokens)?;

    let _ = forth_test.ejecutar_tokens(&mut stack_test, salida)?;

    Ok(stack_test)
}

pub fn leer_archivo_y_almacenar_parser() -> Result<Vec<String>, String> {
    let parser_test = Parser::default();
    parser_test
        .leer_archivo(RUTA_ARCHIVO)
        .map_err(|e| format!("Error {} al leer {}", e, RUTA_ARCHIVO))
}

pub fn formar_tokens(leido: &Vec<String>) -> Result<Vec<TokenParseo>, String> {
    let parser_test = Parser::default();
    let tokens = parser_test.parseo(&leido)?;
    Ok(tokens)
}

pub fn crear_word_usuario<'a>(
    tokens: Vec<TokenParseo>,
) -> Result<(Vec<Vec<ParametroBody>>, Vec<WordUsuario>), String> {
    let mut forth_test = Forth::default();
    let _ = formar_bodys(&mut forth_test, tokens)?;
    Ok((forth_test.bodys, forth_test.words_usuarios))
}

pub fn escribir_en_archivo(texto: &str) -> Result<(), String> {
    let mut archivo =
        File::create(RUTA_ARCHIVO).map_err(|e| format!("Error al crear archivo: {}", e))?;

    archivo
        .write_all(texto.as_bytes())
        .map_err(|e| format!("Error al escribir en archivo: {}", e))?;

    Ok(())
}

pub fn leer_stack() -> Result<Vec<i16>, String> {
    let mut archivo =
        File::open("stack.fth").map_err(|e| format!("Error {} al leer stack.fth", e))?;

    let mut contenido = String::new();

    archivo
        .read_to_string(&mut contenido)
        .map_err(|e| format!("Error {} al leer stack.fth", e))?;

    let numeros = contenido
        .split_whitespace()
        .filter_map(|s| s.parse::<i16>().ok())
        .collect();

    Ok(numeros)
}

pub fn comparar_resultado_stack(resultado_esperado: Vec<i16>) -> Result<(), String> {
    let stack_resultante = set_up(&mut std::io::stdout(), CAPACIDAD_STACK)?;
    assert_eq!(stack_resultante.vector, resultado_esperado);
    Ok(())
}

pub fn comparar_resultado_print(resultado_esperado: &str) -> Result<(), String> {
    let mut buffer = std::io::Cursor::new(Vec::new());
    let _ = set_up(&mut buffer, CAPACIDAD_STACK)?;
    let obtenido = String::from_utf8(buffer.into_inner()).unwrap();
    assert_eq!(obtenido, resultado_esperado);
    Ok(())
}

pub fn comparar_resultado_err(
    resultado_esperado: &str,
    capacidad_stack: usize,
) -> Result<(), String> {
    let resultado = set_up(&mut std::io::stdout(), capacidad_stack);
    assert!(resultado.is_err());
    assert_eq!(resultado.unwrap_err(), resultado_esperado);
    Ok(())
}

pub fn limpiar_archivo(ruta: &str) -> Result<(), String> {
    File::create(ruta)
        .map(|_| ())
        .map_err(|e| format!("Error al limpiar el archivo: {}", e))
}
