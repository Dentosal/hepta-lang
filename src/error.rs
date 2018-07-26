use crate::namespace::SymbolPath;

#[derive(Debug, Clone)]
#[must_use]
pub enum Error {
    FunctionEndOutsideFunction,
    StackUndeflow,
    NameNotDefined(SymbolPath),
}
