use common::{escribir_en_archivo, leer_stack, set_up};

mod common;

#[test]
fn basic_positive_numbers() -> Result<(), String>{
    escribir_en_archivo(&vec![String::from("1"), String::from("2"), 
    String::from("3"), String::from("4"), String::from("5")])?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![5,4,3,2,1]);
    Ok(())
}

#[test]
fn basic_negative_numbers() -> Result<(), String>{
    escribir_en_archivo(&vec![String::from("-1"), String::from("-2"), 
    String::from("-3"), String::from("-4"), String::from("-5")])?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![-5,-4,-3,-2,-1]);
    Ok(())
}

#[test]
fn basic_add_1() -> Result<(), String>{
    escribir_en_archivo(&vec![String::from("1"), String::from("2"), 
    String::from("+")])?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![3]);
    Ok(())
}

#[test]
fn basic_add_2() -> Result<(), String>{
    escribir_en_archivo(&vec![String::from("1"), String::from("2"), 
    String::from("3"), String::from("+")])?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![5,1]);
    Ok(())
}

#[test]
fn basic_sub_1() -> Result<(), String>{
    escribir_en_archivo(&vec![String::from("3"), String::from("4"), 
    String::from("-")])?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![-1]);
    Ok(())
}

#[test]
fn basic_sub_2() -> Result<(), String>{
    escribir_en_archivo(&vec![String::from("1"), String::from("12"), 
    String::from("3"), String::from("-")])?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![9, 1]);
    Ok(())
}

#[test]
fn basic_mul_1() -> Result<(), String>{
    escribir_en_archivo(&vec![String::from("2"), String::from("4"), 
    String::from("*")])?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![8]);
    Ok(())
}

#[test]
fn basic_mul_2() -> Result<(), String>{
    escribir_en_archivo(&vec![String::from("1"), String::from("2"), 
    String::from("3"), String::from("*")])?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![6,1]);
    Ok(())
}

#[test]
fn basic_div_1() -> Result<(), String>{
    escribir_en_archivo(&vec![String::from("12"), String::from("3"), 
    String::from("/")])?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![4]);
    Ok(())
}

#[test]
fn basic_div_2() -> Result<(), String>{
    escribir_en_archivo(&vec![String::from("8"), String::from("3"), 
    String::from("/")])?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![2]);
    Ok(())
}

#[test]
fn basic_div_3() -> Result<(), String>{
    escribir_en_archivo(&vec![String::from("1"), String::from("12"), 
    String::from("3"), String::from("/")])?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![4,1]);
    Ok(())
}

#[test]
fn basic_add_sub() -> Result<(), String>{
    escribir_en_archivo(&vec![String::from("1"), String::from("2"), 
    String::from("+"), String::from("4"), String::from("-")])?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![-1]);
    Ok(())
}

#[test]
fn basic_mul_div() -> Result<(), String>{
    escribir_en_archivo(&vec![String::from("2"), String::from("4"), 
    String::from("*"), String::from("3"), String::from("/")])?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![2]);
    Ok(())
}

#[test]
fn basic_mul_add() -> Result<(), String>{
    escribir_en_archivo(&vec![String::from("1"), String::from("3"), 
    String::from("4"), String::from("*"), String::from("+")])?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![13]);
    Ok(())
}

#[test]
fn basic_add_mul() -> Result<(), String>{
    escribir_en_archivo(&vec![String::from("1"), String::from("3"), 
    String::from("4"), String::from("+"), String::from("*")])?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![7]);
    Ok(())
}

#[test]
fn basic_dup_1() -> Result<(), String>{
    escribir_en_archivo(&vec![String::from("1"), String::from("dup")])?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![1,1]);
    Ok(())
}

#[test]
fn basic_dup_2() -> Result<(), String>{
    escribir_en_archivo(&vec![String::from("1"), String::from("2"),
    String::from("dup")])?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![2,2,1]);
    Ok(())
}

#[test]
fn basic_drop_1() -> Result<(), String>{
    escribir_en_archivo(&vec![String::from("1"), String::from("drop")])?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![]);
    Ok(())
}

#[test]
fn basic_drop_2() -> Result<(), String>{
    escribir_en_archivo(&vec![String::from("1"), String::from("2"),
    String::from("drop")])?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![1]);
    Ok(())
}

#[test]
fn basic_swap_1() -> Result<(), String>{
    escribir_en_archivo(&vec![String::from("1"), String::from("2"),
    String::from("swap")])?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![1,2]);
    Ok(())
}

#[test]
fn basic_swap_2() -> Result<(), String>{
    escribir_en_archivo(&vec![String::from("1"), String::from("2"),
    String::from("3"), String::from("swap")])?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![2,3,1]);
    Ok(())
}

#[test]
fn basic_rot_1() -> Result<(), String>{
    escribir_en_archivo(&vec![String::from("1"), String::from("2"),
    String::from("3"), String::from("rot")])?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![1,3,2]);
    Ok(())
}

#[test]
fn basic_rot_2() -> Result<(), String>{
    escribir_en_archivo(&vec![String::from("1"), String::from("2"),
    String::from("3"), String::from("rot"), String::from("rot"), String::from("rot")])?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![3,2,1]);
    Ok(())
}

#[test]
fn basic_definition_1() -> Result<(), String>{
    escribir_en_archivo(&vec![String::from(":"), String::from("dup-twice"),
    String::from("dup"), String::from("dup"), String::from(";"), String::from("\n"),
    String::from("1"), String::from("dup-twice")])?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![1,1,1]);
    Ok(())
}

#[test]
fn basic_definition_2() -> Result<(), String>{
    escribir_en_archivo(&vec![String::from(":"), String::from("countup"),
    String::from("1"), String::from("2"), String::from("3"), String::from(";"), String::from("\n"),
    String::from("countup")])?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![3,2,1]);
    Ok(())
}

#[test]
fn basic_redefinition() -> Result<(), String>{
    escribir_en_archivo(&vec![String::from(":"), String::from("foo"),
    String::from("dup"), String::from(";"), String::from("\n"), String::from(":"), String::from("foo"),
    String::from("dup"), String::from("dup"), String::from(";"), String::from("\n"),
    String::from("1"),String::from("foo")])?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![1,1,1]);
    Ok(())
}

#[test]
fn basic_shadowing() -> Result<(), String>{
    escribir_en_archivo(&vec![String::from(":"), String::from("swap"),
    String::from("dup"), String::from(";"), String::from("\n"), String::from("1"), String::from("swap")])?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![1,1]);
    Ok(())
}

#[test]
fn basic_shadowing_symbol_1() -> Result<(), String>{
    escribir_en_archivo(&vec![String::from(":"), String::from("+"),
    String::from("*"), String::from(";"), String::from("\n"), String::from("3"), String::from("4"),
    String::from("+")])?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![12]);
    Ok(())
}

#[test]
#[ignore = "Not function good "]
fn basic_non_transitive() -> Result<(), String>{
    escribir_en_archivo(&vec![String::from(":"), String::from("foo"),
    String::from("5"), String::from(";"), String::from("\n"), String::from(":"), String::from("bar"),
    String::from("foo"), String::from(";"), String::from("\n"), String::from(":"), String::from("foo"),
    String::from("6"), String::from(";"), String::from("\n"), String::from("bar"), String::from("foo"),])?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![6,5]);
    Ok(())
}

#[test]
#[ignore = "Not function good (infinitive recursion)"]
fn basic_self_definition() -> Result<(), String>{
    escribir_en_archivo(&vec![String::from(":"), String::from("foo"),
    String::from("10"), String::from(";"), String::from("\n"), String::from(":"), String::from("foo"),
    String::from("foo"), String::from("1"), String::from("+"), String::from(";"), String::from("\n"),
    String::from("foo"),])?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![11]);
    Ok(())
}

#[test]
fn basic_case_insensitive_1() -> Result<(), String>{
    escribir_en_archivo(&vec![String::from("1"), String::from("DUP"),
    String::from("Dup"), String::from("dup")])?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![1,1,1,1]);
    Ok(())
}

#[test]
fn basic_case_insensitive_2() -> Result<(), String>{
    escribir_en_archivo(&vec![String::from("1"), String::from("2"),
    String::from("3"), String::from("4"), String::from("DROP"), String::from("Drop"), String::from("drop")])?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![1]);
    Ok(())
}

#[test]
fn basic_case_insensitive_3() -> Result<(), String>{
    escribir_en_archivo(&vec![String::from("1"), String::from("2"),
    String::from("SWAP"), String::from("3"), String::from("Swap"), String::from("4"), String::from("swap")])?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![1,4,3,2]);
    Ok(())
}

#[test]
fn basic_case_insensitive_4() -> Result<(), String>{
    escribir_en_archivo(&vec![String::from("1"), String::from("2"),
    String::from("OVER"), String::from("Over"), String::from("over")])?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![1,2,1,2,1]);
    Ok(())
}

#[test]
fn basic_case_insensitive_5() -> Result<(), String>{
    escribir_en_archivo(&vec![String::from(":"), String::from("foo"),
    String::from("DUP"), String::from(";"), String::from("\n"), String::from("1"), String::from("FOO"),
    String::from("Foo"), String::from("foo")])?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![1,1,1,1]);
    Ok(())
}

#[test]
fn basic_case_insensitive_6() -> Result<(), String>{
    escribir_en_archivo(&vec![String::from(":"), String::from("SWAP"),
    String::from("DUP"), String::from("Dup"), String::from("dup"), String::from(";"), String::from("\n"),
    String::from("1"), String::from("swap")])?;
    set_up()?;
    let leido_del_stack = leer_stack()?;
    assert_eq!(leido_del_stack, vec![1,1,1,1]);
    Ok(())
}