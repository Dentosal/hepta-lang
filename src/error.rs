use crate::namespace::SymbolPath;
use crate::value::ValueType;

#[derive(Debug, Clone)]
#[must_use]
pub enum SyntaxError {
    UnexpectedEndOfInput,
    AssignToEmpty,
}

#[derive(Debug, Clone)]
#[must_use]
pub enum Error {
    InvalidSyntax(SyntaxError),
    FunctionEndOutsideFunction,
    StackUndeflow,
    NameNotDefined(SymbolPath),
    IntegerOverflow,
    /// WrongArgumentType(actual, allowed)
    WrongArgumentType(ValueType, Vec<ValueType>),
    AssertionFailed,
}
