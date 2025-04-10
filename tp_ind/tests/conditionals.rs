use common::{comparar_resultado_stack, escribir_en_archivo};

mod common;

#[test]
fn conditionals_equal_true() -> Result<(), String> {
    escribir_en_archivo("1 1 = ")?;
    comparar_resultado_stack(vec![-1])
}

#[test]
fn conditionals_equal_false() -> Result<(), String> {
    escribir_en_archivo("1 2 = ")?;
    comparar_resultado_stack(vec![0])
}

#[test]
fn conditionals_less_true() -> Result<(), String> {
    escribir_en_archivo("1 2 < ")?;
    comparar_resultado_stack(vec![-1])
}

#[test]
fn conditionals_less_false() -> Result<(), String> {
    escribir_en_archivo("2 1 < ")?;
    comparar_resultado_stack(vec![0])
}

#[test]
fn conditionals_less_equals() -> Result<(), String> {
    escribir_en_archivo("2 2 < ")?;
    comparar_resultado_stack(vec![0])
}

#[test]
fn conditionals_more_true() -> Result<(), String> {
    escribir_en_archivo("2 1 > ")?;
    comparar_resultado_stack(vec![-1])
}

#[test]
fn conditionals_more_false() -> Result<(), String> {
    escribir_en_archivo("1 2 > ")?;
    comparar_resultado_stack(vec![0])
}

#[test]
fn conditionals_more_equals() -> Result<(), String> {
    escribir_en_archivo("2 2 > ")?;
    comparar_resultado_stack(vec![0])
}

#[test]
fn conditionals_and_none() -> Result<(), String> {
    escribir_en_archivo("0 0 and ")?;
    comparar_resultado_stack(vec![0])
}

#[test]
fn conditionals_and_one() -> Result<(), String> {
    escribir_en_archivo("-1 0 and ")?;
    comparar_resultado_stack(vec![0])
}

#[test]
fn conditionals_and_both() -> Result<(), String> {
    escribir_en_archivo("-1 -1 and ")?;
    comparar_resultado_stack(vec![-1])
}

#[test]
fn conditionals_or_none() -> Result<(), String> {
    escribir_en_archivo("0 0 or ")?;
    comparar_resultado_stack(vec![0])
}

#[test]
fn conditionals_or_one() -> Result<(), String> {
    escribir_en_archivo("-1 0 or ")?;
    comparar_resultado_stack(vec![-1])
}

#[test]
fn conditionals_or_both() -> Result<(), String> {
    escribir_en_archivo("-1 -1 or ")?;
    comparar_resultado_stack(vec![-1])
}

#[test]
fn conditionals_not_true() -> Result<(), String> {
    escribir_en_archivo("-1 not ")?;
    comparar_resultado_stack(vec![0])
}

#[test]
fn conditionals_not_false() -> Result<(), String> {
    escribir_en_archivo("0 not ")?;
    comparar_resultado_stack(vec![-1])
}

#[test]
fn conditionals_not_not() -> Result<(), String> {
    escribir_en_archivo("10 not not ")?;
    comparar_resultado_stack(vec![-1])
}