mod common;

//cargo test -- --nocapture
//cargo test -- --test-threads=1
//RUST_TEST_THREADS=1 cargo test -- --nocapture

#[cfg(test)]
mod metodo {
    use crate::common;

    use common::{
        crear_word_usuario, escribir_en_archivo, formar_tokens, leer_archivo_y_almacenar_parser,
        leer_stack, set_up,
    };

    use tp_ind::{token_parseo::TokenParseo, word_usuario::WordUsuario};

    #[test]
    fn escritura_y_lectura() -> Result<(), String> {
        escribir_en_archivo(": CINCO 5 ;")?;
        let lectura = leer_archivo_y_almacenar_parser()?;
        let lectura_esperada = vec![
            String::from(":"),
            String::from("CINCO"),
            String::from("5"),
            String::from(";"),
        ];
        assert_eq!(lectura, lectura_esperada);
        Ok(())
    }

    #[test]
    fn creacion_tokens() -> Result<(), String> {
        let leido = vec![
            String::from(":"),
            String::from("CINCO"),
            String::from("5"),
            String::from(";"),
        ];
        let tokens = formar_tokens(&leido)?;
        let respuesta_correcta = vec![
            TokenParseo::SimboloInicioWord(String::from(":")),
            TokenParseo::WordName(String::from("CINCO")),
            TokenParseo::Numero(5),
            TokenParseo::SimboloFinWord(String::from(";")),
        ];
        assert_eq!(tokens, respuesta_correcta);
        Ok(())
    }

    #[test]
    fn creacion_word_usuario() -> Result<(), String> {
        let tokens = vec![
            TokenParseo::SimboloInicioWord(String::from(":")),
            TokenParseo::WordName(String::from("CINCO")),
            TokenParseo::Numero(5),
            TokenParseo::SimboloFinWord(String::from(";")),
        ];
        let (_, word) = crear_word_usuario(tokens)?;
        let word_creada = word
            .get(0)
            .ok_or("No se pudo crear la word para comparar")?;
        let word_correcta = WordUsuario::new(String::from("CINCO"), 0);
        assert_eq!(*word_creada, word_correcta);
        Ok(())
    }

    #[test]
    fn ejemplo_sencillo() -> Result<(), String> {
        escribir_en_archivo(": CINCO 5 ; \n CINCO")?;
        set_up()?;
        let leido_del_stack = leer_stack()?;
        assert_eq!(leido_del_stack, vec![5]);
        Ok(())
    }
}