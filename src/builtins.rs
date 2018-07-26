use crate::error::Error;
use crate::interpreter::Interpreter;
use crate::value::{BuiltinFunction, Value, ValueType};

fn f_dup(interp: &mut Interpreter) -> Result<(), Error> {
    let arg1 = interp.data.pop().ok_or(Error::StackUndeflow)?;
    interp.data.push(arg1.clone());
    interp.data.push(arg1);
    Ok(())
}

fn f_swap(interp: &mut Interpreter) -> Result<(), Error> {
    let arg1 = interp.data.pop().ok_or(Error::StackUndeflow)?;
    let arg2 = interp.data.pop().ok_or(Error::StackUndeflow)?;
    interp.data.push(arg1);
    interp.data.push(arg2);
    Ok(())
}

fn f_add(interp: &mut Interpreter) -> Result<(), Error> {
    let arg1 = interp.data.pop().ok_or(Error::StackUndeflow)?;
    let arg2 = interp.data.pop().ok_or(Error::StackUndeflow)?;

    if let Value::Integer(a1) = arg1 {
        if let Value::Integer(a2) = arg2 {
            let res = u64::checked_add(a1, a2).ok_or(Error::IntegerOverflow)?;
            interp.data.push(Value::Integer(res));
            Ok(())
        } else {
            Err(Error::ArgumentTypeError(
                arg2.type_(),
                vec![ValueType::Integer],
            ))
        }
    } else {
        Err(Error::ArgumentTypeError(
            arg1.type_(),
            vec![ValueType::Integer],
        ))
    }
}

fn f_multiply(interp: &mut Interpreter) -> Result<(), Error> {
    let arg1 = interp.data.pop().ok_or(Error::StackUndeflow)?;
    let arg2 = interp.data.pop().ok_or(Error::StackUndeflow)?;

    if let Value::Integer(a1) = arg1 {
        if let Value::Integer(a2) = arg2 {
            let res = u64::checked_mul(a1, a2).ok_or(Error::IntegerOverflow)?;
            interp.data.push(Value::Integer(res));
            Ok(())
        } else {
            Err(Error::ArgumentTypeError(
                arg2.type_(),
                vec![ValueType::Integer],
            ))
        }
    } else {
        Err(Error::ArgumentTypeError(
            arg1.type_(),
            vec![ValueType::Integer],
        ))
    }
}

fn f_equals(interp: &mut Interpreter) -> Result<(), Error> {
    let arg1 = interp.data.pop().ok_or(Error::StackUndeflow)?;
    let arg2 = interp.data.pop().ok_or(Error::StackUndeflow)?;

    interp.data.push(Value::Boolean(arg1 == arg2));
    Ok(())
}

fn f_assert(interp: &mut Interpreter) -> Result<(), Error> {
    let arg1 = interp.data.pop().ok_or(Error::StackUndeflow)?;

    if let Value::Boolean(a1) = arg1 {
        if a1 {
            Ok(())
        } else {
            Err(Error::AssertionFailed)
        }
    } else {
        Err(Error::ArgumentTypeError(
            arg1.type_(),
            vec![ValueType::Boolean],
        ))
    }
}

pub fn register_all(interp: &mut Interpreter) {
    interp.register_builtin(BuiltinFunction::new("dup", f_dup));
    interp.register_builtin(BuiltinFunction::new("swap", f_swap));
    interp.register_builtin(BuiltinFunction::new("add", f_add));
    interp.register_builtin(BuiltinFunction::new("mul", f_multiply));
    interp.register_builtin(BuiltinFunction::new("eq", f_equals));
    interp.register_builtin(BuiltinFunction::new("assert", f_assert));
}
