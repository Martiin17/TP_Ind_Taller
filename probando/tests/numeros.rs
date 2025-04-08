mod common;

use common::set_up;

//use probando::utils::{operar, escribir_resultados, leer_resultados};
//use std::io;

/* fn preparar_y_leer(a: i16, b: i16) -> Result<Vec<i16>, String> {
    let resultados = operar(&a, &b)?;
    //map_err es para pasar de errores io a String
    escribir_resultados(&resultados)
        .map_err(|e| format!("Error al escribir resultados: {}", e))?;
    leer_resultados()
        .map_err(|e| format!("Error al leer resultados: {}", e))
} */


#[test]
fn test_positivos() -> Result<(), String>{
    let resultados = set_up(5, 2)?;
    if resultados == vec![7, 3, 2, 10]{
        Ok(())
    }else{
        Err(String::from("error en test con ambos numeros positivos"))
    }
    //let resultados = preparar_y_leer(5, 2).expect("falló test_positivos");
    //assert_eq!(resultados, vec![7, 3, 2, 10]);
}

#[test]
fn test_negativo_positivo() -> Result<(), String>{
    let resultados = set_up(-5, 3)?;
    if resultados == vec![-2, -8, -1, -15]{
        Ok(())
    }else{
        Err(String::from("error en test con a negativo y b positivo"))
    }
    //let resultados = preparar_y_leer(-5, 3).expect("falló test_negativo_positivo");
    //assert_eq!(resultados, vec![-2, -8, -1, -15]);
}

#[test]
fn test_cero_positivo() -> Result<(), String>{
    let resultados = set_up(0, 4)?;
    if resultados == vec![4, -4, 0, 0]{
        Ok(())
    }else{
        Err(String::from("error en test con a = 0 y b positivo"))
    }
    //let resultados = preparar_y_leer(0, 4).expect("falló test_cero_positivo");
    //assert_eq!(resultados, vec![4, -4, 0, 0]);
}

#[test]
fn test_division_por_cero() -> Result<(), String>{
    let resultados = set_up(4, 0);
    if resultados.is_err(){
        Ok(())
    }else{
        Err(String::from("No dio error el division by zero"))
    }
    //let resultado = preparar_y_leer(10, 0);
    //assert!(resultado.is_err());
    //assert_eq!(resultado.unwrap_err(), "division-by-zero");
}

