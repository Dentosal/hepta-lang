use std::collections::HashMap;

use crate::value::Value;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct AbsoluteSymbolPath(pub Vec<String>);
impl AbsoluteSymbolPath {
    pub fn root() -> Self {
        AbsoluteSymbolPath(Vec::new())
    }
    pub fn join(&self, other: &RelativeSymbolPath) -> Self {
        AbsoluteSymbolPath(self.0.iter().chain(other.0.iter()).cloned().collect())
    }
}

#[derive(Debug, Clone)]
pub struct RelativeSymbolPath(pub Vec<String>);

#[derive(Debug, Clone)]
pub enum SymbolPath {
    Absolute(AbsoluteSymbolPath),
    Relative(RelativeSymbolPath),
}
impl SymbolPath {
    /// Removes relativity information
    fn as_vec(&self) -> Vec<String> {
        match self {
            SymbolPath::Absolute(p) => p.0.clone(),
            SymbolPath::Relative(p) => p.0.clone(),
        }
    }

    pub fn is_absolute(&self) -> bool {
        if let SymbolPath::Absolute(_) = self {
            true
        } else {
            false
        }
    }

    pub fn from_str(s: &str) -> SymbolPath {
        assert!(!s.is_empty());

        let mut fields = s.split('.').peekable();
        let absolute = fields.peek().unwrap().is_empty();

        if absolute {
            fields.next();
            SymbolPath::Absolute(AbsoluteSymbolPath(fields.map(|s| s.to_owned()).collect()))
        } else {
            SymbolPath::Relative(RelativeSymbolPath(fields.map(|s| s.to_owned()).collect()))
        }
    }

    /// Create absolute path
    pub fn realize(self, other: &AbsoluteSymbolPath) -> AbsoluteSymbolPath {
        match self {
            SymbolPath::Absolute(p) => p,
            SymbolPath::Relative(p) => other.join(&p),
        }
    }
}

pub struct Namespace {
    /// Only store absolute paths as keys
    values: HashMap<AbsoluteSymbolPath, Value>,
}
impl Namespace {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: AbsoluteSymbolPath, value: Value) {
        self.values.insert(key, value);
    }

    pub fn remove(&mut self, key: AbsoluteSymbolPath) -> Option<Value> {
        self.values.remove(&key)
    }

    pub fn resolve(&self, key: AbsoluteSymbolPath) -> Option<Value> {
        self.values.get(&key).cloned()
    }
}
