use crate::error::Error;
use crate::interpreter::Interpreter;
use crate::value::BuiltinFunction;

/// (a -- )
fn f_drop(interp: &mut Interpreter) -> Result<(), Error> {
    let _ = interp.data.pop().ok_or(Error::StackUndeflow)?;
    Ok(())
}

/// (a -- a a)
fn f_dup(interp: &mut Interpreter) -> Result<(), Error> {
    let arg0 = interp.data.pop().ok_or(Error::StackUndeflow)?;
    interp.data.push(arg0.clone());
    interp.data.push(arg0);
    Ok(())
}

/// (a b -- a b a)
fn f_over(interp: &mut Interpreter) -> Result<(), Error> {
    let arg1 = interp.data.pop().ok_or(Error::StackUndeflow)?;
    let arg0 = interp.data.pop().ok_or(Error::StackUndeflow)?;
    interp.data.push(arg0.clone());
    interp.data.push(arg1);
    interp.data.push(arg0);
    Ok(())
}

/// (a b -- b a)
fn f_swap(interp: &mut Interpreter) -> Result<(), Error> {
    let arg1 = interp.data.pop().ok_or(Error::StackUndeflow)?;
    let arg0 = interp.data.pop().ok_or(Error::StackUndeflow)?;
    interp.data.push(arg1);
    interp.data.push(arg0);
    Ok(())
}

/// (a b c -- b c a)
fn f_rot(interp: &mut Interpreter) -> Result<(), Error> {
    let arg1 = interp.data.pop().ok_or(Error::StackUndeflow)?;
    let arg0 = interp.data.pop().ok_or(Error::StackUndeflow)?;
    interp.data.push(arg1);
    interp.data.push(arg0);
    Ok(())
}

pub fn register_all(interp: &mut Interpreter) {
    interp.register_builtin(BuiltinFunction::new("drop", f_drop));
    interp.register_builtin(BuiltinFunction::new("dup", f_dup));
    interp.register_builtin(BuiltinFunction::new("over", f_over));
    interp.register_builtin(BuiltinFunction::new("swap", f_swap));
    interp.register_builtin(BuiltinFunction::new("rot", f_rot));
}
