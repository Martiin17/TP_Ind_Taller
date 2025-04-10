use forth::Forth;
use interprete::{escribir_stack, formar_bodys, interpretar_parametros};
use parser::Parser;
use stack::Stack;

mod devolucion;
mod forth;
mod interprete;
mod parametro_body;
mod parser;
mod stack;
mod token_parseo;
mod utils;
mod word_usuario;

fn main() {
    match interpretar_parametros(){
        Ok((capacidad_stack, archivo_leer)) => {
            let mut stack_test = Stack::new(capacidad_stack);
            let mut parser_test = Parser::new();
            let mut forth_test = Forth::new();

            match parser_test.leer_archivo(&archivo_leer){
                Ok(vector_string) => {
                    match parser_test.parseo(&vector_string) {
                    Ok(rta) => {
                        parser_test.tokens = rta;
                    },
                    Err(e) => {
                        println!("{}", e);
                        return;
                    },
                }

                match formar_bodys(&mut forth_test, parser_test.tokens) {
                    Ok(_) => (),
                    Err(e) => {
                        println!("{}", e);
                        return;
                    },
                }

                match forth_test.ejecutar_tokens(&mut stack_test) {
                    Ok(_) => (),
                    Err(e) => {
                        println!("{}", e);
                        return;
                    },
                }

                match escribir_stack(&stack_test) {
                    Ok(_) => (),
                    Err(e) => {
                        println!("{}", e);
                        return;
                    },
                }
                },
                Err(e) => {
                    println!("{}", e);
                    return;
                },
            }
        },
        Err(e) => {
            println!("{}", e);
            return;
        },
    }
}