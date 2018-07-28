use crate::error::Error;
use crate::interpreter::Interpreter;
use crate::value::{BuiltinFunction, Value, ValueType};

fn f_assert(interp: &mut Interpreter) -> Result<(), Error> {
    let arg0 = interp.data.pop().ok_or(Error::StackUndeflow)?;

    if let Value::Boolean(a1) = arg0 {
        if a1 {
            Ok(())
        } else {
            Err(Error::AssertionFailed)
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

fn f_dbgstackdepth(interp: &mut Interpreter) -> Result<(), Error> {
    let s = interp.data.len();
    interp.data.push(Value::Integer(s as u64));
    Ok(())
}

fn f_dbgshowstack(interp: &mut Interpreter) -> Result<(), Error> {
    println!("{:?}", interp.data);
    Ok(())
}

pub fn register_all(interp: &mut Interpreter) {
    interp.register_builtin(BuiltinFunction::new("assert", f_assert));
    interp.register_builtin(BuiltinFunction::new("dbgshow", f_dbgshow));
    interp.register_builtin(BuiltinFunction::new("dbgshowstack", f_dbgshowstack));
    interp.register_builtin(BuiltinFunction::new("dbgstackdepth", f_dbgstackdepth));
}
