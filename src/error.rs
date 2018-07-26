use crate::namespace::SymbolPath;
use crate::value::ValueType;

#[derive(Debug, Clone)]
#[must_use]
pub enum Error {
    FunctionEndOutsideFunction,
    StackUndeflow,
    NameNotDefined(SymbolPath),
    IntegerOverflow,
    /// WrongArgumentType(actual, allowed)
    WrongArgumentType(ValueType, Vec<ValueType>),
    AssertionFailed,
}
