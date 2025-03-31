use crate::stack::Stack;

pub fn ejecutar_suma(stack: &mut Stack) -> Result<(), String> {
    let a = stack.pop()?;
    let b = stack.pop()?;
    stack.push(a + b)
}

pub fn ejecutar_resta(stack: &mut Stack) -> Result<(), String> {
    let a = stack.pop()?;
    let b = stack.pop()?;
    stack.push(a - b)
}

pub fn ejecutar_multiplicacion(stack: &mut Stack) -> Result<(), String> {
    let a = stack.pop()?;
    let b = stack.pop()?;
    stack.push(a * b)
}

pub fn ejecutar_division(stack: &mut Stack) -> Result<(), String> {
    let a = stack.pop()?;
    let b = stack.pop()?;
    if b == 0 {
        Err("division-by-zero".to_string())
    } else {
        stack.push(b / a)
    }
}
