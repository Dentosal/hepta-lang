use crate::error::Error;
use crate::interpreter::Interpreter;
use crate::value::{BuiltinFunction, Value, ValueType};

fn f_eq(interp: &mut Interpreter) -> Result<(), Error> {
    let arg1 = interp.data.pop().ok_or(Error::StackUndeflow)?;
    let arg2 = interp.data.pop().ok_or(Error::StackUndeflow)?;

    interp.data.push(Value::Boolean(arg1 == arg2));
    Ok(())
}

fn f_lt(interp: &mut Interpreter) -> Result<(), Error> {
    let arg1 = interp.data.pop().ok_or(Error::StackUndeflow)?;
    let arg0 = interp.data.pop().ok_or(Error::StackUndeflow)?;
    if let Value::Integer(a0) = arg0 {
        if let Value::Integer(a1) = arg1 {
            interp.data.push(Value::Boolean(a0 < a1));
            Ok(())
        } else {
            Err(Error::WrongArgumentType(
                arg1.type_(),
                vec![ValueType::Integer],
            ))
        }
    } else {
        Err(Error::WrongArgumentType(
            arg0.type_(),
            vec![ValueType::Integer],
        ))
    }
}

pub fn register_all(interp: &mut Interpreter) {
    interp.register_builtin(BuiltinFunction::new("eq", f_eq));
    interp.register_builtin(BuiltinFunction::new("lt", f_lt));
}
