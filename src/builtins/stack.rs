use crate::error::Error;
use crate::interpreter::Interpreter;
use crate::value::BuiltinFunction;

fn f_drop(interp: &mut Interpreter) -> Result<(), Error> {
    let _ = interp.data.pop().ok_or(Error::StackUndeflow)?;
    Ok(())
}

fn f_dup(interp: &mut Interpreter) -> Result<(), Error> {
    let arg0 = interp.data.pop().ok_or(Error::StackUndeflow)?;
    interp.data.push(arg0.clone());
    interp.data.push(arg0);
    Ok(())
}

fn f_swap(interp: &mut Interpreter) -> Result<(), Error> {
    let arg0 = interp.data.pop().ok_or(Error::StackUndeflow)?;
    let arg1 = interp.data.pop().ok_or(Error::StackUndeflow)?;
    interp.data.push(arg0);
    interp.data.push(arg1);
    Ok(())
}

pub fn register_all(interp: &mut Interpreter) {
    interp.register_builtin(BuiltinFunction::new("drop", f_drop));
    interp.register_builtin(BuiltinFunction::new("dup", f_dup));
    interp.register_builtin(BuiltinFunction::new("swap", f_swap));
}
