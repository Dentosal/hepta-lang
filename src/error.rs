use crate::namespace::SymbolPath;
use crate::value::ValueType;

#[derive(Debug, Clone)]
#[must_use]
pub enum Error {
    FunctionEndOutsideFunction,
    StackUndeflow,
    NameNotDefined(SymbolPath),
    IntegerOverflow,
    /// ArgumentTypeError(actual, allowed)
    ArgumentTypeError(ValueType, Vec<ValueType>),
    AssertionFailed,
}
