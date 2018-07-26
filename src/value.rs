#![allow(stutter)]

use std::cmp::PartialEq;
use std::fmt;
use std::hash::{Hash, Hasher};

use crate::error::Error;
use crate::interpreter::Interpreter;
use crate::scanner::Token;

#[derive(Clone)]
pub struct BuiltinFunction {
    /// Name MUST be unique
    name: String,
    /// The actual wrapped function
    f: fn(&mut Interpreter) -> Result<(), Error>,
}
impl BuiltinFunction {
    pub(crate) fn new(name: &str, f: fn(&mut Interpreter) -> Result<(), Error>) -> Self {
        Self {
            name: name.to_owned(),
            f,
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn call(&self, interp: &mut Interpreter) -> Result<(), Error> {
        (self.f)(interp)
    }
}
impl PartialEq for BuiltinFunction {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl Eq for BuiltinFunction {}
impl Hash for BuiltinFunction {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}
impl fmt::Debug for BuiltinFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<builtin:{}>", self.name)
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct HeapPointer(pub usize);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct StructDefinitionIndex(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueType {
    Boolean,
    Index,
    Integer,
    Pointer,
    Function,
    BuiltinFunction,
    UserDefined,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Value {
    Boolean(bool),
    Index(usize),
    Integer(u64),
    Pointer(HeapPointer),
    Function(Vec<Token>),
    BuiltinFunction(BuiltinFunction),
    UserDefined(StructDefinitionIndex),
}
impl Value {
    pub fn type_(&self) -> ValueType {
        use self::Value::*;

        match self {
            Boolean(_) => ValueType::Boolean,
            Index(_) => ValueType::Index,
            Integer(_) => ValueType::Integer,
            Pointer(_) => ValueType::Pointer,
            Function(_) => ValueType::Function,
            BuiltinFunction(_) => ValueType::BuiltinFunction,
            UserDefined(_) => ValueType::UserDefined,
        }
    }
}

pub struct UserStructMetaField {
    pub name: String,
    pub size: usize,
    pub ttag: ValueType,
}

pub struct UserStructMeta {
    pub fields: Vec<UserStructMetaField>,
}
impl UserStructMeta {
    /// Size, in bytes
    pub fn size(&self) -> usize {
        self.fields.iter().map(|item| item.size).sum()
    }
}

struct UserStruct {
    meta: UserStructMeta,
    data: Vec<u8>,
}

struct UserFunction {
    meta: UserStructMeta,
    data: Vec<u8>,
}
