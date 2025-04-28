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
    match interpretar_parametros() {
        Ok((capacidad_stack, archivo_leer)) => {
            let mut stack = Stack::new(capacidad_stack);
            let mut parser = Parser::default();
            let mut forth = Forth::default();

            match parser.leer_archivo(&archivo_leer) {
                Ok(vector_string) => {
                    match parser.parseo(&vector_string) {
                        Ok(rta) => {
                            parser.tokens = rta;
                        }
                        Err(e) => println!("{}", e),
                    }

                    match formar_bodys(&mut forth, parser.tokens) {
                        Ok(_) => (),
                        Err(e) => println!("{}", e),
                    }

                    match forth.ejecutar_tokens(&mut stack, &mut std::io::stdout()) {
                        Ok(_) => (),
                        Err(e) => println!("{}", e),
                    }

                    match escribir_stack(&stack) {
                        Ok(_) => (),
                        Err(e) => println!("{}", e),
                    }
                }
                Err(_) => println!("No se encontro el archivo '{}'", archivo_leer),
            }
        }
        Err(e) => println!("{}", e),
    }
}
