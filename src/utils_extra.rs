use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn leer_archivo(nombre_archivo: &str) -> io::Result<HashMap<String, Vec<String>>> {
    let path = Path::new(nombre_archivo);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut word_name = String::new();
    let mut words: HashMap<String, Vec<String>> = HashMap::new();
    let mut sigue_word_name = false;
    let mut en_word_body = false;

    for linea in reader.lines() {
        let linea = linea?;
        for contenido in linea.split_whitespace(){
            let contenido_string = contenido.to_string();
            let aux = contenido.to_string();
            if contenido_string == ":"{
                sigue_word_name = true;
                en_word_body = true;
            } else if sigue_word_name == true {
                word_name = contenido_string;
                sigue_word_name = false;
                words.insert(aux, Vec::new());
            } else if en_word_body == true {
                if contenido == ";"{
                    en_word_body = false;
                    //words.insert(word_name, vector);
                    //vector.clear();
                } else {
                    match words.get_mut(&word_name){
                        Some(array) => array.push(contenido_string),
                        None => println!("Error al parsear"),
                    }
                }
            } else {
                println!("ejecutar {}", contenido);
            }
        }
    }
    Ok(words)
}

fn main() {
    let ola = leer_archivo("bye.fth");
    println!("{:?}", ola);

}