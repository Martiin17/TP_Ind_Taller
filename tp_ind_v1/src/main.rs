mod forth;
mod interprete;
mod stack;
mod utils;
mod word_usuario;
use forth::Forth;
use interprete::Interprete;
use stack::Stack;

fn main() {
    let mut nuevo_stack = Stack::new(5);
    let test_forth = Forth::new();
    let mut interprete = Interprete::new(test_forth);

    let resultado = interprete.leer_archivo("lectura.fth");

    match resultado {
        Ok(mut resultado) => {
            println!("{:?}", resultado);
            match interprete.verificar_lectura(&mut resultado) {
                Ok(_) => println!("todo ok verificar"),
                Err(e) => println!("{:?}", e),
            }
            match interprete.ejecutar_lectura(&resultado, &mut nuevo_stack) {
                Ok(_) => println!("todo ok ejecutar"),
                Err(e) => println!("{:?}", e),
            }
        }
        Err(e) => println!("{:?}", e),
    }

    match interprete.escribir_stack(nuevo_stack) {
        Ok(_) => println!("Se escribio ok"),
        Err(e) => println!("{}", e),
    }
}
