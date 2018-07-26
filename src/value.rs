use crate::scanner::Token;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct HeapPointer(pub usize);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct StructDefinitionIndex(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueType {
    Integer,
    Index,
    Function,
    Pointer,
    UserDefined(StructDefinitionIndex),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Index(usize),
    Integer(u64),
    Function(Vec<Token>),
    Pointer(HeapPointer, Box<Value>),
    UserDefined(StructDefinitionIndex),
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
