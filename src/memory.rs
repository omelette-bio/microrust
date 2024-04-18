use std::fmt::{Display, Formatter};
use crate::error::EvalError;
use crate::identifier::Identifier;
use crate::namespacestack::NameSpaceStack;
use crate::heap::Heap;
use crate::namespace::NameSpace;
use crate::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub enum Address {
    StackAddress(usize, Identifier),
    HeapAddress(usize),
}

impl Display for Address {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Address::StackAddress(num, var) => write!(f, "@[{},{}]", num, var),
            Address::HeapAddress(num) => write!(f, "@{}", num),
        }
    }
}

pub struct Memory {
    stack: NameSpaceStack,
    heap: Heap,
}

impl Memory {
    pub fn new() -> Self { Memory { stack: NameSpaceStack::new(), heap: Heap::new() } }

    pub fn pop(&mut self) -> Option<NameSpace> { self.stack.pop() }

    pub fn push(&mut self, ns: NameSpace) { self.stack.push(ns) }

    pub fn declare(&mut self, id: &Identifier, mutable: bool, value: Value) -> Result<(), EvalError> { self.stack.declare(id, mutable, value) }

    pub fn get_address(&self, id: &Identifier) -> Result<Address, EvalError> { self.stack.get_address(id) }

    pub fn find(&self, id: &Identifier) -> Result<Value, EvalError> { self.stack.find(id) }

    pub fn malloc(&mut self) -> Address { self.heap.malloc() }
}