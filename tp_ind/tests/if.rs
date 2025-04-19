mod common;

#[cfg(test)]
mod tests_if {
    use crate::common::{self};

    use common::{comparar_resultado_stack, escribir_en_archivo};

    #[test]
    fn if_simple() -> Result<(), String> {
        escribir_en_archivo("
        : f if 2 then ;
        -1 f
        ")?;
        comparar_resultado_stack(vec![2])
    }

    #[test]
    fn if_else() -> Result<(), String> {
        escribir_en_archivo("
        : f if 2 else 3 then ;
        -1 f
        0 f
        ")?;
        comparar_resultado_stack(vec![2, 3])
    }

    #[test]
    fn nested_if() -> Result<(), String> {
        escribir_en_archivo("
            : f
        if
            if 1 else 2 then
        else
            drop 3
        then ;
        -1 -1 f
        0 -1 f
        0 0 f
        ")?;
        comparar_resultado_stack(vec![1, 2, 3])
    }

    #[test]
    fn nested_if_else() -> Result<(), String> {
        escribir_en_archivo("
            : f
        dup 0 = if
            drop 2
        else dup 1 = if
            drop 3
        else
            drop 4
        then then ;
        0 f
        1 f
        2 f
        ")?;
        comparar_resultado_stack(vec![2, 3, 4])
    }

    #[test]
    fn if_non_canonical() -> Result<(), String> {
        escribir_en_archivo("
        : f if 10 then ;
        5 f
        ")?;
        comparar_resultado_stack(vec![10])
    }
}