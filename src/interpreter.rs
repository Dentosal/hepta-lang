use std::collections::HashMap;

use crate::builtins;
use crate::error::Error;
use crate::namespace::{AbsoluteSymbolPath, Namespace, SymbolPath};
use crate::scanner::{scan_token, Token};
use crate::value::{BuiltinFunction, HeapPointer, Value};

#[derive(Debug, Clone)]
pub struct Interpreter {
    current_namespace: AbsoluteSymbolPath,
    skip_next: bool,

    nesting: u32,
    scan: Vec<Token>,

    pub(crate) data: Vec<Value>,
    pub call: Vec<Token>,

    heap: HashMap<HeapPointer, Value>,
    dict: Namespace,
}
impl Interpreter {
    pub fn new() -> Self {
        Self {
            current_namespace: AbsoluteSymbolPath::root(),
            skip_next: false,
            nesting: 0,
            scan: Vec::new(),
            data: Vec::new(),
            call: Vec::new(),
            heap: HashMap::new(),
            dict: Namespace::new(),
        }
    }

    /// Returns interpreter with builtin functions loaded
    pub fn with_builtins(mut self) -> Self {
        builtins::register_all(&mut self);
        self
    }

    pub fn set_skip(&mut self) {
        self.skip_next = true;
    }

    pub fn in_function(&self) -> bool {
        self.nesting > 0
    }

    pub(crate) fn register_builtin(&mut self, bf: BuiltinFunction) {
        let sp = SymbolPath::from_str(&bf.name()).realize(&AbsoluteSymbolPath::root());
        debug_assert!(self.dict.resolve(&sp) == None);
        self.dict.insert(sp, Value::BuiltinFunction(bf));
    }

    fn push_current_function(&mut self) {
        assert!(!self.in_function());

        self.data.push(Value::Function(self.scan.clone()));
        self.scan.clear();
    }

    fn pop_assign_to(&mut self, name: &str) -> Result<(), Error> {
        let value = self.data.pop().ok_or(Error::StackUndeflow)?;
        let path = SymbolPath::from_str(name).realize(&self.current_namespace);
        self.dict.insert(path, value);
        Ok(())
    }

    fn set_namespace(&mut self, name: &str) -> Result<(), Error> {
        let path = SymbolPath::from_str(name).realize(&self.current_namespace);
        self.current_namespace = path;
        Ok(())
    }

    pub(crate) fn execute_value(&mut self, value: Value) -> Result<(), Error> {
        match value {
            Value::BuiltinFunction(f) => f.call(self),
            Value::Function(f) => {
                for token in f.iter().rev().cloned() {
                    self.call.push(token);
                }
                Ok(())
            },
            v => {
                self.data.push(v);
                Ok(())
            },
        }
    }

    fn execute_ident(&mut self, ident: &str) -> Result<(), Error> {
        // println!("{:<20} |{:?}", ident, self.data);

        // Numeric values cannot be overridden
        if let Ok(int_value) = ident.parse::<u64>() {
            self.data.push(Value::Integer(int_value));
            Ok(())
        } else {
            let sp = SymbolPath::from_str(ident);
            let rp = sp.clone().realize(&self.current_namespace);
            if let Some(val) = self.dict.resolve(&rp) {
                return self.execute_value(val);
            } else if let SymbolPath::Relative(ref rsp) = sp {
                let mut cursor = rp;
                while let Some(p) = cursor.parent() {
                    let cp = p.join(rsp);
                    if let Some(val) = self.dict.resolve(&cp) {
                        return self.execute_value(val);
                    }
                    cursor = p;
                }
            }
            Err(Error::NameNotDefined(sp))
        }
    }

    fn execute_token(&mut self, token: Token) -> Result<(), Error> {
        // println!("TOKEN EXEC ({:?})", token);
        if self.in_function() {
            match token {
                Token::FunctionStart => self.nesting += 1,
                Token::FunctionEnd => {
                    self.nesting -= 1;
                    if !self.in_function() {
                        if self.skip_next {
                            self.skip_next = false;
                        } else {
                            self.push_current_function();
                        }
                    }
                },
                token => self.scan.push(token),
            };
            Ok(())
        } else if self.skip_next {
            match token {
                Token::FunctionStart => {
                    self.nesting += 1;
                    Ok(())
                },
                Token::FunctionEnd => Err(Error::FunctionEndOutsideFunction),
                _ => {
                    self.skip_next = false;
                    Ok(())
                },
            }
        } else {
            match token {
                Token::FunctionStart => {
                    self.nesting += 1;
                    Ok(())
                },
                Token::FunctionEnd => Err(Error::FunctionEndOutsideFunction),
                Token::AssignIdentifier(ident) => self.pop_assign_to(&ident),
                Token::SetNamespace(ident) => self.set_namespace(&ident),
                Token::Identifier(ident) => self.execute_ident(&ident),
            }
        }
    }

    /// Return true if ready for next for more execute_token calls (outside step)
    pub fn idle(&mut self) -> bool {
        self.call.is_empty()
    }

    pub fn step(&mut self) -> Result<(), Error> {
        if let Some(token) = self.call.pop() {
            self.execute_token(token)
        } else {
            Ok(())
        }
    }

    pub fn execute(&mut self, input: &str, _filepath: Option<&str>) -> Result<(), Error> {
        let mut in_stream = input.chars().peekable();

        while in_stream.peek().is_some() {
            match scan_token(&mut in_stream) {
                Ok(None) => break,
                Ok(Some(token)) => {
                    self.execute_token(token)?;

                    while !self.idle() {
                        self.step()?;
                    }
                },
                Err(e) => {
                    return Err(Error::InvalidSyntax(e));
                },
            }
        }

        Ok(())
    }
}
