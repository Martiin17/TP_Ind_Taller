mod common;

#[cfg(test)]
mod error {
    use crate::common::{self, comparar_resultado_err};

    use common::{escribir_en_archivo, leer_stack};

    #[test]
    fn underflow_1()-> Result<(), String> {
        escribir_en_archivo("+")?;
        comparar_resultado_err("stack-underflow\n")?;
        let stack_resultante = leer_stack()?;
        //assert_eq!(stack_resultante, vec![]);
        Ok(())
    }

    #[test]
    fn underflow_2()-> Result<(), String> {
        escribir_en_archivo("1 +")?;
        comparar_resultado_err("stack-underflow\n")?;
        let stack_resultante = leer_stack()?;
        //assert_eq!(stack_resultante, vec![]);
        Ok(())
    }

    #[test]
    fn underflow_3()-> Result<(), String> {
        escribir_en_archivo("-")?;
        comparar_resultado_err("stack-underflow\n")?;
        let stack_resultante = leer_stack()?;
        //assert_eq!(stack_resultante, vec![]);
        Ok(())
    }

    #[test]
    fn underflow_4()-> Result<(), String> {
        escribir_en_archivo("- 1")?;
        comparar_resultado_err("stack-underflow\n")?;
        let stack_resultante = leer_stack()?;
        //assert_eq!(stack_resultante, vec![]);
        Ok(())
    }

    #[test]
    fn underflow_5()-> Result<(), String> {
        escribir_en_archivo("*")?;
        comparar_resultado_err("stack-underflow\n")?;
        let stack_resultante = leer_stack()?;
        //assert_eq!(stack_resultante, vec![]);
        Ok(())
    }

    #[test]
    fn underflow_6()-> Result<(), String> {
        escribir_en_archivo("1 *")?;
        comparar_resultado_err("stack-underflow\n")?;
        let stack_resultante = leer_stack()?;
        //assert_eq!(stack_resultante, vec![]);
        Ok(())
    }

    #[test]
    fn underflow_7()-> Result<(), String> {
        escribir_en_archivo("/")?;
        comparar_resultado_err("stack-underflow\n")?;
        let stack_resultante = leer_stack()?;
        //assert_eq!(stack_resultante, vec![]);
        Ok(())
    }

    #[test]
    fn underflow_8()-> Result<(), String> {
        escribir_en_archivo("1 /")?;
        comparar_resultado_err("stack-underflow\n")?;
        let stack_resultante = leer_stack()?;
        //assert_eq!(stack_resultante, vec![]);
        Ok(())
    }

    #[test]
    fn underflow_9()-> Result<(), String> {
        escribir_en_archivo("dup")?;
        comparar_resultado_err("stack-underflow\n")?;
        let stack_resultante = leer_stack()?;
        //assert_eq!(stack_resultante, vec![]);
        Ok(())
    }

    #[test]
    fn underflow_10()-> Result<(), String> {
        escribir_en_archivo("drop")?;
        comparar_resultado_err("stack-underflow\n")?;
        let stack_resultante = leer_stack()?;
        //assert_eq!(stack_resultante, vec![]);
        Ok(())
    }

    #[test]
    fn underflow_11()-> Result<(), String> {
        escribir_en_archivo("swap")?;
        comparar_resultado_err("stack-underflow\n")?;
        let stack_resultante = leer_stack()?;
        //assert_eq!(stack_resultante, vec![]);
        Ok(())
    }

    #[test]
    fn underflow_12()-> Result<(), String> {
        escribir_en_archivo("1 swap")?;
        comparar_resultado_err("stack-underflow\n")?;
        let stack_resultante = leer_stack()?;
        //assert_eq!(stack_resultante, vec![]);
        Ok(())
    }

    #[test]
    fn underflow_13()-> Result<(), String> {
        escribir_en_archivo("over")?;
        comparar_resultado_err("stack-underflow\n")?;
        let stack_resultante = leer_stack()?;
        //assert_eq!(stack_resultante, vec![]);
        Ok(())
    }

    #[test]
    fn underflow_14()-> Result<(), String> {
        escribir_en_archivo("1 over")?;
        comparar_resultado_err("stack-underflow\n")?;
        let stack_resultante = leer_stack()?;
        //assert_eq!(stack_resultante, vec![]);
        Ok(())
    }

    #[test]
    fn division_by_zero()-> Result<(), String> {
        escribir_en_archivo("4 0 /")?;
        comparar_resultado_err("division-by-zero\n")?;
        let stack_resultante = leer_stack()?;
        //assert_eq!(stack_resultante, vec![]);
        Ok(())
    }

    #[test]
    fn invalid_word_1()-> Result<(), String> {
        escribir_en_archivo(": 1 2 ;")?;
        comparar_resultado_err("invalid-word\n")?;
        let stack_resultante = leer_stack()?;
        //assert_eq!(stack_resultante, vec![]);
        Ok(())
    }

    #[test]
    fn invalid_word_2()-> Result<(), String> {
        escribir_en_archivo(": -1 2 ;")?;
        comparar_resultado_err("invalid-word\n")?;
        let stack_resultante = leer_stack()?;
        //assert_eq!(stack_resultante, vec![]);
        Ok(())
    }

    #[test]
    fn unknown_word()-> Result<(), String> {
        escribir_en_archivo("foo")?;
        comparar_resultado_err("?\n")?;
        let stack_resultante = leer_stack()?;
        //assert_eq!(stack_resultante, vec![]);
        Ok(())
    }
}