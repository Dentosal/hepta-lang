#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueType {
    Integer,
    Index,
    FunctionPointer,
    HeapPointer,
    UserDefined,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TypeTag {
    tag: u32,
}
impl TypeTag {
    fn get(&self) -> ValueType {
        match self.tag {
            0 => ValueType::Integer,
            1 => ValueType::Index,
            2 => ValueType::FunctionPointer,
            3 => ValueType::HeapPointer,
            _ => ValueType::UserDefined,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Value {
    ttag: TypeTag,
    value: u64,
}

pub struct UserStructMetaField {
    pub name: String,
    pub size: usize,
    pub ttag: TypeTag,
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
