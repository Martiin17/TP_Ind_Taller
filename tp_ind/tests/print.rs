mod common;

#[cfg(test)]
mod print {
    use crate::common::{self, comparar_resultado_print};

    use common::{comparar_resultado_stack, escribir_en_archivo};

    //use tp_ind::{token_parseo::TokenParseo, word_usuario::WordUsuario};

    //En el hilo "Trailing whitespaces" dice:
    //"(no consideramos como error ni el trailing whitespace, ni el leading whitespace)""
    #[test]
    fn dot_without_leftover()-> Result<(), String> {
        escribir_en_archivo("1 2 
        . .
        ")?;
        comparar_resultado_print("2 1 ")?;
        comparar_resultado_stack(vec![])
    }

    #[test]
    fn dot_with_leftover()-> Result<(), String> {
        escribir_en_archivo("1 2 3 4 5
        . . .
        ")?;
        comparar_resultado_print("5 4 3 ")?;
        comparar_resultado_stack(vec![1, 2])
    }

    #[test]
    fn cr_1()-> Result<(), String> {
        escribir_en_archivo("cr")?;
        comparar_resultado_print("\n")?;
        comparar_resultado_stack(vec![])
    }

    #[test]
    fn cr_2()-> Result<(), String> {
        escribir_en_archivo("cr cr")?;
        comparar_resultado_print("\n\n")?;
        comparar_resultado_stack(vec![])
    }

    #[test]
    fn dot_and_cr()-> Result<(), String> {
        escribir_en_archivo("1 .
        cr cr
        2 .
        ")?;
        comparar_resultado_print("1 \n\n2 ")?;
        comparar_resultado_stack(vec![])
    }

    #[test]
    fn emit_uppercase()-> Result<(), String> {
        escribir_en_archivo("65 emit")?;
        comparar_resultado_print("A ")?;
        comparar_resultado_stack(vec![])
    }

    #[test]
    fn emit_lowercase()-> Result<(), String> {
        escribir_en_archivo("97 emit")?;
        comparar_resultado_print("a ")?;
        comparar_resultado_stack(vec![])
    }

    #[test]
    fn emit_multiple()-> Result<(), String> {
        escribir_en_archivo("68 67 66 65
        emit emit emit emit
        ")?;
        comparar_resultado_print("A B C D ")?;
        comparar_resultado_stack(vec![])
    }

    #[test]
    fn dot_quote_hello_world()-> Result<(), String> {
        escribir_en_archivo(".\" hello world\"")?;
        comparar_resultado_print("hello world ")?;
        comparar_resultado_stack(vec![])
    }

    #[test]
    fn dot_quote_multiple_hello_world()-> Result<(), String> {
        escribir_en_archivo(".\" hello      world!\"")?;
        comparar_resultado_print("hello      world! ")?;
        comparar_resultado_stack(vec![])
    }

    #[test]
    fn dot_quote_and_cr()-> Result<(), String> {
        escribir_en_archivo(".\" hello\"
        cr
        .\" world\"
        ")?;
        comparar_resultado_print("hello \nworld ")?;
        comparar_resultado_stack(vec![])
    }
}
