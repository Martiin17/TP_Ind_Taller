mod utils;
use utils::{leer_resultados, operar};
mod descarte;

fn crear_texto(texto: &str) -> Vec<String>{
    let texto_separado = texto.split_whitespace();
    let mut resultado: Vec<String> = vec![];
    for palabra in texto_separado{
        resultado.push(String::from(palabra));
    }
    resultado
}

fn imprimir_lo_escrito(vector_palabras: &Vec<String>) {
    for palabra in vector_palabras{
        println!("{}", palabra);
    }
}
fn main(){
    /* let a:i16 = 5;
    let b:i16 = 2;
    let _ = operar(&a, &b);
    let resultado = leer_resultados();
    match resultado{
        Ok(resultado) => {
            for elem in resultado{
                println!("El resultado leido es: {}", elem);
            }
        },
        Err(e) => println!("error: {}", e),
    } */
   let vector_palabras = crear_texto(": CINCO 5 ;
    CINCO");
   imprimir_lo_escrito(&vector_palabras);

}

