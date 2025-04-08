mod utils;
use utils::{leer_resultados, operar};
fn main(){
    let a:i16 = 5;
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
    }
}

