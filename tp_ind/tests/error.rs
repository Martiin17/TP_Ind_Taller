mod common;

#[cfg(test)]
mod error {
    use crate::common::{self, comparar_resultado_err, limpiar_archivo, CAPACIDAD_STACK};

    use common::{escribir_en_archivo, leer_stack};

    #[test]
    fn underflow_1()-> Result<(), String> {
        limpiar_archivo("stack.fth")?;
        escribir_en_archivo("+")?;
        comparar_resultado_err("stack-underflow\n", CAPACIDAD_STACK)?;
        let stack_resultante = leer_stack()?;
        assert_eq!(stack_resultante, vec![]);
        Ok(())
    }

    #[test]
    fn underflow_2()-> Result<(), String> {
        limpiar_archivo("stack.fth")?;
        escribir_en_archivo("1 +")?;
        comparar_resultado_err("stack-underflow\n", CAPACIDAD_STACK)?;
        let stack_resultante = leer_stack()?;
        assert_eq!(stack_resultante, vec![]);
        Ok(())
    }

    #[test]
    fn underflow_3()-> Result<(), String> {
        limpiar_archivo("stack.fth")?;
        escribir_en_archivo("-")?;
        comparar_resultado_err("stack-underflow\n", CAPACIDAD_STACK)?;
        let stack_resultante = leer_stack()?;
        assert_eq!(stack_resultante, vec![]);
        Ok(())
    }

    #[test]
    fn underflow_4()-> Result<(), String> {
        limpiar_archivo("stack.fth")?;
        escribir_en_archivo("- 1")?;
        comparar_resultado_err("stack-underflow\n", CAPACIDAD_STACK)?;
        let stack_resultante = leer_stack()?;
        assert_eq!(stack_resultante, vec![]);
        Ok(())
    }

    #[test]
    fn underflow_5()-> Result<(), String> {
        limpiar_archivo("stack.fth")?;
        escribir_en_archivo("*")?;
        comparar_resultado_err("stack-underflow\n", CAPACIDAD_STACK)?;
        let stack_resultante = leer_stack()?;
        assert_eq!(stack_resultante, vec![]);
        Ok(())
    }

    #[test]
    fn underflow_6()-> Result<(), String> {
        limpiar_archivo("stack.fth")?;
        escribir_en_archivo("1 *")?;
        comparar_resultado_err("stack-underflow\n", CAPACIDAD_STACK)?;
        let stack_resultante = leer_stack()?;
        assert_eq!(stack_resultante, vec![]);
        Ok(())
    }

    #[test]
    fn underflow_7()-> Result<(), String> {
        limpiar_archivo("stack.fth")?;
        escribir_en_archivo("/")?;
        comparar_resultado_err("stack-underflow\n", CAPACIDAD_STACK)?;
        let stack_resultante = leer_stack()?;
        assert_eq!(stack_resultante, vec![]);
        Ok(())
    }

    #[test]
    fn underflow_8()-> Result<(), String> {
        limpiar_archivo("stack.fth")?;
        escribir_en_archivo("1 /")?;
        comparar_resultado_err("stack-underflow\n", CAPACIDAD_STACK)?;
        let stack_resultante = leer_stack()?;
        assert_eq!(stack_resultante, vec![]);
        Ok(())
    }

    #[test]
    fn underflow_9()-> Result<(), String> {
        limpiar_archivo("stack.fth")?;
        escribir_en_archivo("dup")?;
        comparar_resultado_err("stack-underflow\n", CAPACIDAD_STACK)?;
        let stack_resultante = leer_stack()?;
        assert_eq!(stack_resultante, vec![]);
        Ok(())
    }

    #[test]
    fn underflow_10()-> Result<(), String> {
        limpiar_archivo("stack.fth")?;
        escribir_en_archivo("drop")?;
        comparar_resultado_err("stack-underflow\n", CAPACIDAD_STACK)?;
        let stack_resultante = leer_stack()?;
        assert_eq!(stack_resultante, vec![]);
        Ok(())
    }

    #[test]
    fn underflow_11()-> Result<(), String> {
        limpiar_archivo("stack.fth")?;
        escribir_en_archivo("swap")?;
        comparar_resultado_err("stack-underflow\n", CAPACIDAD_STACK)?;
        let stack_resultante = leer_stack()?;
        assert_eq!(stack_resultante, vec![]);
        Ok(())
    }

    #[test]
    fn underflow_12()-> Result<(), String> {
        limpiar_archivo("stack.fth")?;
        escribir_en_archivo("1 swap")?;
        comparar_resultado_err("stack-underflow\n", CAPACIDAD_STACK)?;
        let stack_resultante = leer_stack()?;
        assert_eq!(stack_resultante, vec![]);
        Ok(())
    }

    #[test]
    fn underflow_13()-> Result<(), String> {
        limpiar_archivo("stack.fth")?;
        escribir_en_archivo("over")?;
        comparar_resultado_err("stack-underflow\n", CAPACIDAD_STACK)?;
        let stack_resultante = leer_stack()?;
        assert_eq!(stack_resultante, vec![]);
        Ok(())
    }

    #[test]
    fn underflow_14()-> Result<(), String> {
        limpiar_archivo("stack.fth")?;
        escribir_en_archivo("1 over")?;
        comparar_resultado_err("stack-underflow\n", CAPACIDAD_STACK)?;
        let stack_resultante = leer_stack()?;
        assert_eq!(stack_resultante, vec![]);
        Ok(())
    }

    #[test]
    fn division_by_zero()-> Result<(), String> {
        limpiar_archivo("stack.fth")?;
        escribir_en_archivo("4 0 /")?;
        comparar_resultado_err("division-by-zero\n", CAPACIDAD_STACK)?;
        let stack_resultante = leer_stack()?;
        assert_eq!(stack_resultante, vec![]);
        Ok(())
    }

    #[test]
    fn invalid_word_1()-> Result<(), String> {
        limpiar_archivo("stack.fth")?;
        escribir_en_archivo(": 1 2 ;")?;
        comparar_resultado_err("invalid-word\n", CAPACIDAD_STACK)?;
        let stack_resultante = leer_stack()?;
        assert_eq!(stack_resultante, vec![]);
        Ok(())
    }

    #[test]
    fn invalid_word_2()-> Result<(), String> {
        limpiar_archivo("stack.fth")?;
        escribir_en_archivo(": -1 2 ;")?;
        comparar_resultado_err("invalid-word\n", CAPACIDAD_STACK)?;
        let stack_resultante = leer_stack()?;
        assert_eq!(stack_resultante, vec![]);
        Ok(())
    }

    #[test]
    fn unknown_word()-> Result<(), String> {
        limpiar_archivo("stack.fth")?;
        escribir_en_archivo("foo")?;
        comparar_resultado_err("?\n", CAPACIDAD_STACK)?;
        let stack_resultante = leer_stack()?;
        assert_eq!(stack_resultante, vec![]);
        Ok(())
    }
}