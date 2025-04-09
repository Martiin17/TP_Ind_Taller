use forth::Forth;
use interprete::{armar_words_usuario, escribir_stack, interpretar_parametros};
use parametro_body::ParametroBody;
use parser::Parser;
use stack::Stack;

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
mod parametro_body;

fn main() {
    let (capacidad_stack, archivo_leer) = interpretar_parametros();
    let mut stack_test = Stack::new(capacidad_stack);
    let mut parser_test = Parser::new();
    let mut forth_test = Forth::new();

    if let Ok(vector_string) = parser_test.leer_archivo(&archivo_leer) {

        match parser_test.parseo(&vector_string) {
            Ok(rta) => {
                //println!("vector: {:?}", &rta);
                parser_test.tokens = rta;
                //println!("{:?}", rta);
            },
            //Ok(_) => println!("Todo ok. no muestro todos los tokens"),
            Err(e) => println!("{}", e),
        }
        
        match armar_words_usuario(&mut forth_test, parser_test.tokens) {
            Ok(_) => {
                println!("Todo ok");
                println!("forth usuario: {:?}", forth_test.words_usuarios);
                println!("forth restantes: {:?}", forth_test.restante);
                println!("forth bodys: {:?}", forth_test.bodys);
            }
            Err(e) => println!("{}", e),
        }

        /* match forth_test.verificar_no_transitive(){
            Ok(_) => {
                println!("Todo ok no transitive");
                //println!("forth usuario: {:?}", forth_test.words_usuarios);
                //println!("forth restantes: {:?}", forth_test.restante);
            },
            Err(e) => println!("{:?}", e),
        } */

        match forth_test.ejecutar_tokens(&mut stack_test) {
            Ok(_) => println!("{:?}", stack_test),
            Err(e) => println!("Error: {}", e),
        }

        match escribir_stack(&stack_test) {
            Ok(_) => println!("Escribio OK"),
            Err(e) => println!("Error: {}", e),
        }
    }
}
