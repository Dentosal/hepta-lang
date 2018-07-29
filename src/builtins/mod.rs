mod boolean;
mod compare;
mod control_flow;
mod debug;
mod generated;
mod stack;

use crate::interpreter::Interpreter;

pub fn register_all(interp: &mut Interpreter) {
    debug::register_all(interp);
    stack::register_all(interp);
    compare::register_all(interp);
    boolean::register_all(interp);
    control_flow::register_all(interp);

    generated::int::register_all(interp);
}
