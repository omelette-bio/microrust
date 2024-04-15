use std::fmt::{Display, Formatter};
use crate::identifier::Identifier;
use crate::namespacestack::NameSpaceStack;
use crate::heap::Heap;

#[derive(Debug, Clone, PartialEq)]
pub enum Address {
    StackAddress(usize, Identifier),
    HeapAddress(usize),
}

impl Display for Address {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Address::StackAddress(num, var) => write!(f, "@[{},{}]", num, var),
            Address::HeapAddress(num) => write!(f, "@[{}]", num),
        }
    }
}

pub struct Memory {
    stack: NameSpaceStack,
    heap: Heap,
}

impl Memory {
    pub fn new() -> Self { Memory { stack: NameSpaceStack::new(), heap: Heap::new() } }
}