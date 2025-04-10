mod common;

#[cfg(test)]
mod conditionals {
    use crate::common;

    use common::{comparar_resultado_stack, escribir_en_archivo};

    #[test]
    fn equal_true() -> Result<(), String> {
        escribir_en_archivo("1 1 = ")?;
        comparar_resultado_stack(vec![-1])
    }

    #[test]
    fn equal_false() -> Result<(), String> {
        escribir_en_archivo("1 2 = ")?;
        comparar_resultado_stack(vec![0])
    }

    #[test]
    fn less_true() -> Result<(), String> {
        escribir_en_archivo("1 2 < ")?;
        comparar_resultado_stack(vec![-1])
    }

    #[test]
    fn less_false() -> Result<(), String> {
        escribir_en_archivo("2 1 < ")?;
        comparar_resultado_stack(vec![0])
    }

    #[test]
    fn less_equals() -> Result<(), String> {
        escribir_en_archivo("2 2 < ")?;
        comparar_resultado_stack(vec![0])
    }

    #[test]
    fn more_true() -> Result<(), String> {
        escribir_en_archivo("2 1 > ")?;
        comparar_resultado_stack(vec![-1])
    }

    #[test]
    fn more_false() -> Result<(), String> {
        escribir_en_archivo("1 2 > ")?;
        comparar_resultado_stack(vec![0])
    }

    #[test]
    fn more_equals() -> Result<(), String> {
        escribir_en_archivo("2 2 > ")?;
        comparar_resultado_stack(vec![0])
    }

    #[test]
    fn and_none() -> Result<(), String> {
        escribir_en_archivo("0 0 and ")?;
        comparar_resultado_stack(vec![0])
    }

    #[test]
    fn and_one() -> Result<(), String> {
        escribir_en_archivo("-1 0 and ")?;
        comparar_resultado_stack(vec![0])
    }

    #[test]
    fn and_both() -> Result<(), String> {
        escribir_en_archivo("-1 -1 and ")?;
        comparar_resultado_stack(vec![-1])
    }

    #[test]
    fn or_none() -> Result<(), String> {
        escribir_en_archivo("0 0 or ")?;
        comparar_resultado_stack(vec![0])
    }

    #[test]
    fn or_one() -> Result<(), String> {
        escribir_en_archivo("-1 0 or ")?;
        comparar_resultado_stack(vec![-1])
    }

    #[test]
    fn or_both() -> Result<(), String> {
        escribir_en_archivo("-1 -1 or ")?;
        comparar_resultado_stack(vec![-1])
    }

    #[test]
    fn not_true() -> Result<(), String> {
        escribir_en_archivo("-1 not ")?;
        comparar_resultado_stack(vec![0])
    }

    #[test]
    fn not_false() -> Result<(), String> {
        escribir_en_archivo("0 not ")?;
        comparar_resultado_stack(vec![-1])
    }

    #[test]
    fn not_not() -> Result<(), String> {
        escribir_en_archivo("10 not not ")?;
        comparar_resultado_stack(vec![-1])
    }
}