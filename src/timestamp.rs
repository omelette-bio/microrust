use std::time::SystemTime;

#[derive(Debug, Clone, PartialEq)]
pub struct Timestamp { 
  tstamp: SystemTime
}

impl Timestamp {
  pub fn generate() -> Self {
    Timestamp { tstamp: SystemTime::now() }
  }
}