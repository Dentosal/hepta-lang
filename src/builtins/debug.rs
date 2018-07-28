use crate::error::Error;
use crate::interpreter::Interpreter;
use crate::value::{BuiltinFunction, Value, ValueType};

fn f_assert(interp: &mut Interpreter) -> Result<(), Error> {
    let arg1 = interp.data.pop().ok_or(Error::StackUndeflow)?;

    if let Value::Boolean(a1) = arg1 {
        if a1 {
            Ok(())
        } else {
            Err(Error::AssertionFailed)
        }
    } else {
        Err(Error::WrongArgumentType(
            arg1.type_(),
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

fn f_dbgshow(interp: &mut Interpreter) -> Result<(), Error> {
    match interp.data.last() {
        Some(v) => println!("{:?}", v),
        None => println!("(stack empty)"),
    };
    Ok(())
}

pub fn register_all(interp: &mut Interpreter) {
    interp.register_builtin(BuiltinFunction::new("assert", f_assert));
    interp.register_builtin(BuiltinFunction::new("dbgshow", f_dbgshow));
}
