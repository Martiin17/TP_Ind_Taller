mod common;

#[cfg(test)]
mod basics {
    use crate::common;

    use common::{comparar_resultado_stack, escribir_en_archivo};

    #[test]
    fn positive_numbers() -> Result<(), String> {
        escribir_en_archivo("1 2 3 4 5")?;
        comparar_resultado_stack(vec![1, 2, 3, 4, 5])
    }

    #[test]
    fn negative_numbers() -> Result<(), String> {
        escribir_en_archivo("-1 -2 -3 -4 -5")?;
        comparar_resultado_stack(vec![-1, -2, -3, -4, -5])
    }

    #[test]
    fn add_1() -> Result<(), String> {
        escribir_en_archivo("1 2 +")?;
        comparar_resultado_stack(vec![3])
    }

    #[test]
    fn add_2() -> Result<(), String> {
        escribir_en_archivo("1 2 3 +")?;
        comparar_resultado_stack(vec![1, 5])
    }

    #[test]
    fn sub_1() -> Result<(), String> {
        escribir_en_archivo("3 4 -")?;
        comparar_resultado_stack(vec![-1])
    }

    #[test]
    fn sub_2() -> Result<(), String> {
        escribir_en_archivo("1 12 3 -")?;
        comparar_resultado_stack(vec![1, 9])
    }

    #[test]
    fn mul_1() -> Result<(), String> {
        escribir_en_archivo("2 4 *")?;
        comparar_resultado_stack(vec![8])
    }

    #[test]
    fn mul_2() -> Result<(), String> {
        escribir_en_archivo("1 2 3 *")?;
        comparar_resultado_stack(vec![1, 6])
    }

    #[test]
    fn div_1() -> Result<(), String> {
        escribir_en_archivo("12 3 /")?;
        comparar_resultado_stack(vec![4])
    }

    #[test]
    fn div_2() -> Result<(), String> {
        escribir_en_archivo("8 3 /")?;
        comparar_resultado_stack(vec![2])
    }

    #[test]
    fn div_3() -> Result<(), String> {
        escribir_en_archivo("1 12 3 /")?;
        comparar_resultado_stack(vec![1, 4])
    }

    #[test]
    fn add_sub() -> Result<(), String> {
        escribir_en_archivo("1 2 + 4 -")?;
        comparar_resultado_stack(vec![-1])
    }

    #[test]
    fn mul_div() -> Result<(), String> {
        escribir_en_archivo("2 4 * 3 /")?;
        comparar_resultado_stack(vec![2])
    }

    #[test]
    fn mul_add() -> Result<(), String> {
        escribir_en_archivo("1 3 4 * +")?;
        comparar_resultado_stack(vec![13])
    }

    #[test]
    fn add_mul() -> Result<(), String> {
        escribir_en_archivo("1 3 4 + *")?;
        comparar_resultado_stack(vec![7])
    }

    #[test]
    fn dup_1() -> Result<(), String> {
        escribir_en_archivo("1 dup")?;
        comparar_resultado_stack(vec![1, 1])
    }

    #[test]
    fn dup_2() -> Result<(), String> {
        escribir_en_archivo("1 2 dup")?;
        comparar_resultado_stack(vec![1, 2, 2])
    }

    #[test]
    fn drop_1() -> Result<(), String> {
        escribir_en_archivo("1 drop")?;
        comparar_resultado_stack(vec![])
    }

    #[test]
    fn drop_2() -> Result<(), String> {
        escribir_en_archivo("1 2 drop")?;
        comparar_resultado_stack(vec![1])
    }

    #[test]
    fn swap_1() -> Result<(), String> {
        escribir_en_archivo("1 2 swap")?;
        comparar_resultado_stack(vec![2, 1])
    }

    #[test]
    fn swap_2() -> Result<(), String> {
        escribir_en_archivo("1 2 3 swap")?;
        comparar_resultado_stack(vec![1, 3, 2])
    }

    #[test]
    fn rot_1() -> Result<(), String> {
        escribir_en_archivo("1 2 3 rot")?;
        comparar_resultado_stack(vec![2, 3, 1])
    }

    #[test]
    fn rot_2() -> Result<(), String> {
        escribir_en_archivo("1 2 3 rot rot rot")?;
        comparar_resultado_stack(vec![1, 2, 3])
    }

    #[test]
    fn definition_1() -> Result<(), String> {
        escribir_en_archivo(": dup-twice dup dup ; \n 1 dup-twice")?;
        comparar_resultado_stack(vec![1, 1, 1])
    }

    #[test]
    fn definition_2() -> Result<(), String> {
        escribir_en_archivo(": countup 1 2 3 ; \n countup")?;
        comparar_resultado_stack(vec![1, 2, 3])
    }

    #[test]
    fn redefinition() -> Result<(), String> {
        escribir_en_archivo(": foo dup ; \n : foo dup dup ; \n 1 foo")?;
        comparar_resultado_stack(vec![1, 1, 1])
    }

    #[test]
    fn shadowing() -> Result<(), String> {
        escribir_en_archivo(": swap dup ; \n 1 swap")?;
        comparar_resultado_stack(vec![1, 1])
    }

    #[test]
    fn shadowing_symbol_1() -> Result<(), String> {
        escribir_en_archivo(": + * ; \n 3 4 +")?;
        comparar_resultado_stack(vec![12])
    }

    #[test]
    //#[ignore = "Not function good "]
    fn non_transitive() -> Result<(), String> {
        escribir_en_archivo(": foo 5 ; \n : bar foo ; \n : foo 6 ; \n bar foo")?;
        comparar_resultado_stack(vec![5, 6])
    }

    #[test]
    //#[ignore = "Not function good (infinitive recursion)"]
    fn self_definition() -> Result<(), String> {
        escribir_en_archivo(": foo 10 ; \n : foo foo 1 + ; \n foo")?;
        comparar_resultado_stack(vec![11])
    }

    #[test]
    fn case_insensitive_1() -> Result<(), String> {
        escribir_en_archivo("1 DUP Dup dup")?;
        comparar_resultado_stack(vec![1, 1, 1, 1])
    }

    #[test]
    fn case_insensitive_2() -> Result<(), String> {
        escribir_en_archivo("1 2 3 4 DROP Drop drop")?;
        comparar_resultado_stack(vec![1])
    }

    #[test]
    fn case_insensitive_3() -> Result<(), String> {
        escribir_en_archivo("1 2 SWAP 3 Swap 4 swap")?;
        comparar_resultado_stack(vec![2, 3, 4, 1])
    }

    #[test]
    fn case_insensitive_4() -> Result<(), String> {
        escribir_en_archivo("1 2 OVER Over over")?;
        comparar_resultado_stack(vec![1, 2, 1, 2, 1])
    }

    #[test]
    fn case_insensitive_5() -> Result<(), String> {
        escribir_en_archivo(": foo DUP ; \n 1 FOO Foo foo")?;
        comparar_resultado_stack(vec![1, 1, 1, 1])
    }

    #[test]
    fn case_insensitive_6() -> Result<(), String> {
        escribir_en_archivo(": SWAP DUP Dup dup ; \n 1 swap")?;
        comparar_resultado_stack(vec![1, 1, 1, 1])
    }
}
