use probando::utils::{operar, escribir_resultados, leer_resultados};
use std::io;

pub fn set_up(a: i16, b: i16) -> Result<Vec<i16>, String>{
    let resultados = operar(&a, &b)?;
    //map_err es para pasar de errores io a String
    escribir_resultados(&resultados)
        .map_err(|e| format!("Error al escribir resultados: {}", e))?;
    
    leer_resultados()
        .map_err(|e| format!("Error al leer resultados: {}", e))
}