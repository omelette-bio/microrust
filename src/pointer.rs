use crate::memory::Address;
use crate::timestamp::Timestamp;
use std::fmt;


#[derive(Debug, Clone, PartialEq)]
pub struct Pointer {
  address: Address,
  timestamp: Timestamp
}

impl fmt::Display for Pointer {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.address)
  }
}

impl Pointer {
  pub fn new(add: Address) -> Self {
    Pointer { address: add, timestamp: Timestamp::generate() }
  }

  pub fn get_address(&self) -> &Address {
    &self.address
  }
}