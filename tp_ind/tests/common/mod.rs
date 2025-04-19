use tp_ind::{
    forth::Forth,
    interprete::{escribir_stack, formar_bodys},
    parametro_body::ParametroBody,
    parser::Parser,
    stack::Stack,
    token_parseo::TokenParseo,
    word_usuario::WordUsuario,
};

use std::fs::File;
use std::io::{Read, Write};

const CAPACIDAD_STACK: usize = 512; // 1024/2
const RUTA_ARCHIVO: &str = "probando.fth";

pub fn set_up() -> Result<(), String> {
    let mut stack_test = Stack::new(CAPACIDAD_STACK);
    let mut parser_test = Parser::new();
    let mut forth_test = Forth::new();

    let leido = parser_test
        .leer_archivo(RUTA_ARCHIVO)
        .map_err(|e| format!("Error {} al leer {}", e, RUTA_ARCHIVO))?;

    let tokens = parser_test.parseo(&leido)?;
    parser_test.tokens = tokens;

    let _ = formar_bodys(&mut forth_test, parser_test.tokens)?;

    let _ = forth_test.ejecutar_tokens(&mut stack_test, &mut std::io::stdout())?;

    let _ =
        escribir_stack(&stack_test).map_err(|e| format!("Error {} al escribir en stack.fth", e))?;

    Ok(())
}

pub fn set_up_devuelve_stack<W: Write>(salida: &mut W) -> Result<Stack, String> {
    let mut stack_test = Stack::new(CAPACIDAD_STACK);
    let mut parser_test = Parser::new();
    let mut forth_test = Forth::new();

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
    let parser_test = Parser::new();
    parser_test
        .leer_archivo(RUTA_ARCHIVO)
        .map_err(|e| format!("Error {} al leer {}", e, RUTA_ARCHIVO))
    
}

pub fn formar_tokens(leido: &Vec<String>) -> Result<Vec<TokenParseo>, String> {
    let parser_test = Parser::new();
    let tokens = parser_test.parseo(&leido)?;
    Ok(tokens)
}

pub fn crear_word_usuario<'a>(
    tokens: Vec<TokenParseo>,
) -> Result<(Vec<Vec<ParametroBody>>, Vec<WordUsuario>), String> {
    let mut forth_test = Forth::new();
    let _ = formar_bodys(&mut forth_test, tokens)?;
    Ok((forth_test.bodys, forth_test.words_usuarios))
}

pub fn escribir_en_archivo(texto: &str) -> Result<(), String> {
    let mut archivo = File::create(RUTA_ARCHIVO)
        .map_err(|e| format!("Error al crear archivo: {}", e))?;

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
    let stack_resultante = set_up_devuelve_stack(&mut std::io::stdout())?;
    assert_eq!(stack_resultante.vector, resultado_esperado);
    Ok(())
}

pub fn comparar_resultado_print(resultado_esperado: &str) -> Result<(), String> {
    let mut buffer = std::io::Cursor::new(Vec::new());
    let _ = set_up_devuelve_stack(&mut buffer)?;
    let obtenido = String::from_utf8(buffer.into_inner()).unwrap();
    assert_eq!(obtenido, resultado_esperado);
    Ok(())
}
