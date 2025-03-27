mod stack;
mod utils;
mod word_primitiva;
mod forth;
mod interprete;
mod error_handler;

use forth::Forth;
use stack::Stack;
use interprete::Interprete;


fn main() {
    /* let mut new_stack = Stack::new(5);
    //let mut words_primitivas: HashMap<String, fn(&mut Stack)> = HashMap::new();
    //let probando = WordPrimitiva::new("+".to_string(), funciones_aritmetica::ejecutar_suma(&mut new_stack));
    let mut capo = Forth::new();
    //capo.inicializar();
    capo.agregar_word_usuario("SUM", vec![String::from("1"), String::from("2"), String::from("3"), 
    String::from("4"), String::from("5"), String::from("6"), String::from("7")]);
    //capo.agregar_word_usuario("SUM", vec![String::from("3")]);
    match capo.ejecutar(&String::from("SUM"), &mut new_stack){
        Ok(_) => println!("todo ok"),
        Err(e) => println!("{}", e),
    }
    println!("{:?}", new_stack); */

    let mut nuevo_stack = Stack::new(5);
    let mut test_forth = Forth::new();
    let mut interprete = Interprete::new(test_forth);

    let mut resultado = interprete.leer_archivo("ola.fth");

    match resultado{
        Ok(mut resultado) => {
            println!("{:?}", resultado);
            match interprete.verificar_lectura(&mut resultado){
                Ok(_) => println!("todo ok verificar"),
                Err(e) => println!("{:?}", e),
            }
            match interprete.ejecutar_lectura(&resultado, &mut nuevo_stack){
                Ok(_) => println!("Todo ok ejecutar"),
                Err(e) => println!("{:?}", e),
            }
        },
        Err(e) => println!("{:?}", e),
    }


    println!("{:?}", nuevo_stack);

}
