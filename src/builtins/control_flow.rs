use crate::error::Error;
use crate::interpreter::Interpreter;
use crate::value::{BuiltinFunction, Value, ValueType};

/// Pop (unnamed) function from stack and excutes that (i.e. calls or pushes)
fn f_exec(interp: &mut Interpreter) -> Result<(), Error> {
    let arg0 = interp.data.pop().ok_or(Error::StackUndeflow)?;
    interp.execute_value(arg0)
}

/// Skip next "token" (item) if false
fn f_if(interp: &mut Interpreter) -> Result<(), Error> {
    let arg0 = interp.data.pop().ok_or(Error::StackUndeflow)?;

    if let Value::Boolean(a1) = arg0 {
        if !a1 {
            interp.set_skip();
        }
        Ok(())
    } else {
        Err(Error::WrongArgumentType(
            arg0.type_(),
            vec![ValueType::Boolean],
        ))
    }
}

pub fn register_all(interp: &mut Interpreter) {
    interp.register_builtin(BuiltinFunction::new("exec", f_exec));
    interp.register_builtin(BuiltinFunction::new("if", f_if));
}
