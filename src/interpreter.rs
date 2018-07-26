use std::collections::HashMap;

use crate::error::Error;
use crate::namespace::{Namespace, SymbolPath, AbsoluteSymbolPath};
use crate::scanner::{scan_token, Token};
use crate::value::{HeapPointer, Value};

pub struct Interpreter {
    current_namespace: AbsoluteSymbolPath,

    nesting: u32,
    scan: Vec<Token>,

    data: Vec<Value>,
    call: Vec<u64>,

    heap: HashMap<HeapPointer, Value>,
    dict: Namespace,
}
impl Interpreter {
    pub fn new() -> Self {
        Self {
            current_namespace: AbsoluteSymbolPath::root(),
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
        self
    }

    pub fn in_function(&self) -> bool {
        self.nesting > 0
    }

    fn push_current_function(&mut self) {
        assert!(!self.in_function());

        self.data.push(Value::Function(self.scan.clone()));
        self.scan.clear();
    }

    fn pop_assign_to(&mut self, name: &String) -> Result<(), Error> {
        let value = self.data.pop().ok_or(Error::StackUndeflow)?;
        let path = SymbolPath::from_str(&name.to_owned()).realize(&self.current_namespace);
        self.dict.insert(path, value);
        Ok(())
    }

    pub fn execute_ident(&mut self, ident: &String) -> Result<(), Error> {
        println!("EXEC({})", ident);

        // Numeric values cannot be overridden
        if let Ok(int_value) = ident.parse::<u64>() {
            self.data.push(Value::Integer(int_value));
            Ok(())
        } else {
            let sp = SymbolPath::from_str(ident);
            let rp = sp.clone().realize(&self.current_namespace);
            if let Some(val) = self.dict.resolve(rp) {
                // TODO
                Ok(())
            } else {
                Err(Error::NameNotDefined(sp))
            }
        }
    }

    pub fn execute_token(&mut self, token: Token) -> Result<(), Error> {
        println!("TOKEN EXEC ({:?})", token);
        if self.in_function() {
            match token {
                Token::FunctionStart => self.nesting += 1,
                Token::FunctionEnd => {
                    self.nesting -= 1;
                    if !self.in_function() {
                        self.push_current_function();
                    }
                },
                token => self.scan.push(token),
            };
            Ok(())
        } else {
            match token {
                Token::FunctionStart => {
                    self.nesting += 1;
                    Ok(())
                },
                Token::FunctionEnd => Err(Error::FunctionEndOutsideFunction),
                Token::AssignIdentifier(ident) => self.pop_assign_to(&ident),
                Token::Identifier(ident) => self.execute_ident(&ident),
            }
        }
    }

    pub fn execute(&mut self, input: &str, _filepath: Option<&str>) -> Result<(), Error> {
        let mut in_stream = input.chars().peekable();
        while let Ok(token) = scan_token(&mut in_stream) {
            self.execute_token(token)?;
        }

        Ok(())
    }
}
