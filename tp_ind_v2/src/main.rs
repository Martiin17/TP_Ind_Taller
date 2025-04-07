use forth::Forth;
use interprete::{armar_words_usuario};
use parser::Parser;
use stack::Stack;
use token_parseo::TokenParseo;
use word_usuario::WordUsuario;

mod devolucion;
mod forth;
mod funcion;
mod interprete;
mod parser;
mod stack;
mod token_parseo;
mod utils;
mod word_primitiva;
mod word_usuario;
mod token;
mod estructura_if;
mod parametro_if;
fn main() {
    let mut stack_test = Stack::new(20);
    let mut parser_test = Parser::new();
    let mut forth_test = Forth::new();
    let mut aux: &Vec<WordUsuario<'_>> = &vec![];

    if let Ok(vector_string) = parser_test.leer_archivo("probando.fth") {
        /* match parser_test.parseo(vector_string) {
            Ok(_) => println!("{:?}", parser_test.tokens),
            //Ok(_) => println!("Todo ok. no muestro todos los tokens"),
            Err(e) => println!("{}", e),
        } */

        match parser_test.parseo(&vector_string) {
            Ok(rta) => {
                println!("vector: {:?}", &rta);
                parser_test.tokens = rta;
                //println!("{:?}", rta);
            },
            //Ok(_) => println!("Todo ok. no muestro todos los tokens"),
            Err(e) => println!("{}", e),
        }
        
        match armar_words_usuario(&mut forth_test, &parser_test.tokens) {
            Ok(_) => {
                println!("Todo ok");
                println!("forth usuario: {:?}", forth_test.words_usuarios);
                println!("forth restantes: {:?}", forth_test.restante);
            }
            Err(e) => println!("{}", e),
        }

        match forth_test.verificar_no_transitive(){
            Ok(_) => {
                println!("Todo ok no transitive");
                println!("forth usuario: {:?}", forth_test.words_usuarios);
                println!("forth restantes: {:?}", forth_test.restante);
            },
            Err(e) => println!("{:?}", e),
        }

        /* let prueba = forth_test.words_usuarios.get(0);
        match prueba{
            Some(valor) => {
                match forth_test.encontrar_cierre_if(valor.get_body_not_mut()){
                    Ok(rta) => {
                        println!("{:?}", rta);
                    },
                    Err(e) => println!("{:?}", e),
                }
            },
            None => println!("NONE"),
        } */

        match forth_test.ejecutar_tokens(&mut stack_test) {
            Ok(_) => println!("{:?}", stack_test),
            Err(e) => println!("Error: {}", e),
        }
    }
}
