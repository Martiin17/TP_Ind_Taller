use forth::Forth;
use interprete::armar_words_usuario;
//use interprete::obtener_word;
use parser::Parser;
use stack::Stack;

mod devolucion;
mod forth;
mod funcion;
mod interprete;
mod parser;
mod stack;
mod token;
mod token_parseo;
mod token_usuario;
mod utils;
mod word_primitiva;
mod word_usuario;

fn main() {
    let mut stack_test = Stack::new(20);
    let mut parser_test = Parser::new();
    let mut forth_test = Forth::new();

    if let Ok(vector_string) = parser_test.leer_archivo("probando.fth") {
        match parser_test.parseo(vector_string) {
            Ok(_) => println!("{:?}", parser_test.tokens),
            Err(e) => println!("{}", e),
        }
        match armar_words_usuario(parser_test.tokens, &mut forth_test) {
            Ok(_) => {
                println!("Todo ok");
                println!("forth usuario: {:?}", forth_test.words_usuarios);
                println!("forth restantes: {:?}", forth_test.restante);
            }
            Err(e) => println!("{}", e),
        }
        /* match obtener_word(parser_test.tokens){
            Ok(_) => println!("llegue"),
            Err(e) => println!("{}", e),
        } */
    }
}
