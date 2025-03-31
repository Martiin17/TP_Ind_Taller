use crate::stack::Stack;

pub fn ejecutar_dup(stack: &mut Stack) -> Result<(), String> {
    let a = stack.pop()?;
    stack.push(a)?;
    stack.push(a)
}

//REVISAR
pub fn ejecutar_drop(stack: &mut Stack) -> Result<(), String> {
    //No devuelve el nro, ver si causa problemas
    stack.pop()?;
    Ok(())
}

pub fn ejecutar_swap(stack: &mut Stack) -> Result<(), String> {
    let a = stack.pop()?;
    let b = stack.pop()?;
    stack.push(a)?;
    stack.push(b)
}

pub fn ejecutar_over(stack: &mut Stack) -> Result<(), String> {
    let a = stack.pop()?;
    let b = stack.pop()?;
    stack.push(b)?;
    stack.push(a)?;
    stack.push(b)
}

pub fn ejecutar_rot(stack: &mut Stack) -> Result<(), String> {
    let a = stack.pop()?;
    let b = stack.pop()?;
    let c = stack.pop()?;
    stack.push(b)?;
    stack.push(a)?;
    stack.push(c)
}
