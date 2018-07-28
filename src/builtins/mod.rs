mod generated;
mod stack;
mod compare;
mod boolean;
mod debug;

use crate::interpreter::Interpreter;

pub fn register_all(interp: &mut Interpreter) {
    debug::register_all(interp);
    stack::register_all(interp);
    compare::register_all(interp);
    boolean::register_all(interp);
    
    generated::int::register_all(interp);
}
