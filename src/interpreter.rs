use std::collections::HashMap;

use super::stack::{CallStack, DataStack, ScanStack};
use super::value::Value;
use super::scanner::scan_token;

pub struct Heap {
    data: Vec<u8>,
}
impl Heap {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
}

pub struct Interpreter {
    scan: ScanStack,
    data: DataStack,
    call: CallStack,

    heap: Heap,
    dict: HashMap<String, Vec<Value>>,
}
impl Interpreter {
    pub fn new() -> Self {
        Self {
            scan: Vec::new(),
            data: Vec::new(),
            call: Vec::new(),
            heap: Heap::new(),
            dict: HashMap::new(),
        }
    }

    /// Returns interpreter with builtin functions loaded
    pub fn with_builins(mut self) -> Self {
        self
    }

    pub fn execute(&mut self, input: &str, filepath: Option<&str>) {
        let mut in_stream = input.chars().peekable();
        while let Some(token) = scan_token(&mut in_stream) {

        }
    }
}
