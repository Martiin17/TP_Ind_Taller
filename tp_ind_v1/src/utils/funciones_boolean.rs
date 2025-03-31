use crate::stack::Stack;

pub fn ejecutar_igual(stack: &mut Stack) -> Result<(), String> {
    let a = stack.pop()?;
    let b = stack.pop()?;
    if a == b {
        stack.push(-1)
    } else {
        stack.push(0)
    }
}

pub fn ejecutar_mayor(stack: &mut Stack) -> Result<(), String> {
    let a = stack.pop()?;
    let b = stack.pop()?;
    if b > a { stack.push(-1) } else { stack.push(0) }
}

pub fn ejecutar_menor(stack: &mut Stack) -> Result<(), String> {
    let a = stack.pop()?;
    let b = stack.pop()?;
    if b < a { stack.push(-1) } else { stack.push(0) }
}

pub fn ejecutar_and(stack: &mut Stack) -> Result<(), String> {
    let a = stack.pop()?;
    let b = stack.pop()?;
    stack.push(b & a)
}

pub fn ejecutar_or(stack: &mut Stack) -> Result<(), String> {
    let a = stack.pop()?;
    let b = stack.pop()?;
    stack.push(b | a)
}

pub fn ejecutar_not(stack: &mut Stack) -> Result<(), String> {
    let a = stack.pop()?;
    if a != 0 {
        stack.push(-1)
    } else {
        stack.push(0)
    }
}
