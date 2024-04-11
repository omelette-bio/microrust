use crate::error::EvalError;
use crate::value::Value;

#[derive(Debug)]
pub struct MemoryCell {
    mutable: bool,
    value: Value,
}

impl MemoryCell {
    pub fn new(mutable: bool, value: Value) -> Self { MemoryCell { mutable, value } }
    pub fn is_mutable(&self) -> bool { self.mutable }
    pub fn get_value(&self) -> Result<Value, EvalError> {
        match self.value {
            Value::Unit => Ok(Value::Unit),
            Value::Boolean(b) => Ok(Value::Boolean(b)),
            Value::Integer(i) => Ok(Value::Integer(i)),
        }
    }
    pub fn set_value(&mut self, v: Value) -> Result<(), EvalError> {
        if !self.is_mutable() {
            return Err(EvalError::NotMutable(None))
        }
        self.value = v;
        Ok(())
    }
}
