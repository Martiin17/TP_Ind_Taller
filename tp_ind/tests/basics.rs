use common::{escribir_en_archivo, leer_stack, set_up};

mod common;

#[test]
fn basic_positive_numbers() -> Result<(), String>{
    escribir_en_archivo("1 2 3 4 5")?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![5,4,3,2,1]);
    Ok(())
}

#[test]
fn basic_negative_numbers() -> Result<(), String>{
    escribir_en_archivo("-1 -2 -3 -4 -5")?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![-5,-4,-3,-2,-1]);
    Ok(())
}

#[test]
fn basic_add_1() -> Result<(), String>{
    escribir_en_archivo("1 2 +")?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![3]);
    Ok(())
}

#[test]
fn basic_add_2() -> Result<(), String>{
    escribir_en_archivo("1 2 3 +")?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![5,1]);
    Ok(())
}

#[test]
fn basic_sub_1() -> Result<(), String>{
    escribir_en_archivo("3 4 -")?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![-1]);
    Ok(())
}

#[test]
fn basic_sub_2() -> Result<(), String>{
    escribir_en_archivo("1 12 3 -")?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![9, 1]);
    Ok(())
}

#[test]
fn basic_mul_1() -> Result<(), String>{
    escribir_en_archivo("2 4 *")?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![8]);
    Ok(())
}

#[test]
fn basic_mul_2() -> Result<(), String>{
    escribir_en_archivo("1 2 3 *")?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![6,1]);
    Ok(())
}

#[test]
fn basic_div_1() -> Result<(), String>{
    escribir_en_archivo("12 3 /")?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![4]);
    Ok(())
}

#[test]
fn basic_div_2() -> Result<(), String>{
    escribir_en_archivo("8 3 /")?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![2]);
    Ok(())
}

#[test]
fn basic_div_3() -> Result<(), String>{
    escribir_en_archivo("1 12 3 /")?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![4,1]);
    Ok(())
}

#[test]
fn basic_add_sub() -> Result<(), String>{
    escribir_en_archivo("1 2 + 4 -")?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![-1]);
    Ok(())
}

#[test]
fn basic_mul_div() -> Result<(), String>{
    escribir_en_archivo("2 4 * 3 /")?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![2]);
    Ok(())
}

#[test]
fn basic_mul_add() -> Result<(), String>{
    escribir_en_archivo("1 3 4 * +")?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![13]);
    Ok(())
}

#[test]
fn basic_add_mul() -> Result<(), String>{
    escribir_en_archivo("1 3 4 + *")?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![7]);
    Ok(())
}

#[test]
fn basic_dup_1() -> Result<(), String>{
    escribir_en_archivo("1 dup")?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![1,1]);
    Ok(())
}

#[test]
fn basic_dup_2() -> Result<(), String>{
    escribir_en_archivo("1 2 dup")?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![2,2,1]);
    Ok(())
}

#[test]
fn basic_drop_1() -> Result<(), String>{
    escribir_en_archivo("1 drop")?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![]);
    Ok(())
}

#[test]
fn basic_drop_2() -> Result<(), String>{
    escribir_en_archivo("1 2 drop")?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![1]);
    Ok(())
}

#[test]
fn basic_swap_1() -> Result<(), String>{
    escribir_en_archivo("1 2 swap")?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![1,2]);
    Ok(())
}

#[test]
fn basic_swap_2() -> Result<(), String>{
    escribir_en_archivo("1 2 3 swap")?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![2,3,1]);
    Ok(())
}

#[test]
fn basic_rot_1() -> Result<(), String>{
    escribir_en_archivo("1 2 3 rot")?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![1,3,2]);
    Ok(())
}

#[test]
fn basic_rot_2() -> Result<(), String>{
    escribir_en_archivo("1 2 3 rot rot rot")?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![3,2,1]);
    Ok(())
}

#[test]
fn basic_definition_1() -> Result<(), String>{
    escribir_en_archivo(": dup-twice dup dup ; \n 1 dup-twice")?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![1,1,1]);
    Ok(())
}

#[test]
fn basic_definition_2() -> Result<(), String>{
    escribir_en_archivo(": countup 1 2 3 ; \n countup")?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![3,2,1]);
    Ok(())
}

#[test]
fn basic_redefinition() -> Result<(), String>{
    escribir_en_archivo(": foo dup ; \n : foo dup dup ; \n 1 foo")?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![1,1,1]);
    Ok(())
}

#[test]
fn basic_shadowing() -> Result<(), String>{
    escribir_en_archivo(": swap dup ; \n 1 swap")?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![1,1]);
    Ok(())
}

#[test]
fn basic_shadowing_symbol_1() -> Result<(), String>{
    escribir_en_archivo(": + * ; \n 3 4 +")?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![12]);
    Ok(())
}

#[test]
#[ignore = "Not function good "]
fn basic_non_transitive() -> Result<(), String>{
    escribir_en_archivo(": foo 5 ; \n : bar foo ; \n : foo 6 ; \n bar foo")?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![6,5]);
    Ok(())
}

#[test]
#[ignore = "Not function good (infinitive recursion)"]
fn basic_self_definition() -> Result<(), String>{
    escribir_en_archivo(": foo 10 ; \n : foo foo 1 + ; \n foo")?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![11]);
    Ok(())
}

#[test]
fn basic_case_insensitive_1() -> Result<(), String>{
    escribir_en_archivo("1 DUP Dup dup")?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![1,1,1,1]);
    Ok(())
}

#[test]
fn basic_case_insensitive_2() -> Result<(), String>{
    escribir_en_archivo("1 2 3 4 DROP Drop drop")?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![1]);
    Ok(())
}

#[test]
fn basic_case_insensitive_3() -> Result<(), String>{
    escribir_en_archivo("1 2 SWAP 3 Swap 4 swap")?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![1,4,3,2]);
    Ok(())
}

#[test]
fn basic_case_insensitive_4() -> Result<(), String>{
    escribir_en_archivo("1 2 OVER Over over")?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![1,2,1,2,1]);
    Ok(())
}

#[test]
fn basic_case_insensitive_5() -> Result<(), String>{
    escribir_en_archivo(": foo DUP ; \n 1 FOO Foo foo")?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![1,1,1,1]);
    Ok(())
}

#[test]
fn basic_case_insensitive_6() -> Result<(), String>{
    escribir_en_archivo(": SWAP DUP Dup dup ; \n 1 swap")?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![1,1,1,1]);
    Ok(())
}