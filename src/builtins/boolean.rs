use crate::error::Error;
use crate::interpreter::Interpreter;
use crate::value::{BuiltinFunction, Value, ValueType};

fn f_not(interp: &mut Interpreter) -> Result<(), Error> {
    let arg0 = interp.data.pop().ok_or(Error::StackUndeflow)?;

    if let Value::Boolean(a0) = arg0 {
        interp.data.push(Value::Boolean(!a0));
        Ok(())
    } else {
        Err(Error::WrongArgumentType(
            arg0.type_(),
            vec![ValueType::Boolean],
        ))
    }
}

fn f_and(interp: &mut Interpreter) -> Result<(), Error> {
    let arg1 = interp.data.pop().ok_or(Error::StackUndeflow)?;
    let arg0 = interp.data.pop().ok_or(Error::StackUndeflow)?;
    if let Value::Boolean(a0) = arg0 {
        if let Value::Boolean(a1) = arg1 {
            interp.data.push(Value::Boolean(a0 && a1));
            Ok(())
        } else {
            Err(Error::WrongArgumentType(
                arg1.type_(),
                vec![ValueType::Boolean],
            ))
        }
    } else {
        Err(Error::WrongArgumentType(
            arg0.type_(),
            vec![ValueType::Boolean],
        ))
    }
}

fn f_or(interp: &mut Interpreter) -> Result<(), Error> {
    let arg1 = interp.data.pop().ok_or(Error::StackUndeflow)?;
    let arg0 = interp.data.pop().ok_or(Error::StackUndeflow)?;
    if let Value::Boolean(a0) = arg0 {
        if let Value::Boolean(a1) = arg1 {
            interp.data.push(Value::Boolean(a0 || a1));
            Ok(())
        } else {
            Err(Error::WrongArgumentType(
                arg1.type_(),
                vec![ValueType::Boolean],
            ))
        }
    } else {
        Err(Error::WrongArgumentType(
            arg0.type_(),
            vec![ValueType::Boolean],
        ))
    }
}

pub fn register_all(interp: &mut Interpreter) {
    interp.register_builtin(BuiltinFunction::new("not", f_not));
    interp.register_builtin(BuiltinFunction::new("and", f_and));
    interp.register_builtin(BuiltinFunction::new("or", f_or));
}
