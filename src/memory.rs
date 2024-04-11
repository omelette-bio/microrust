use std::fmt::{Display, Formatter};
use crate::identifier::Identifier;

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