use crate::error::EvalError;
use crate::value::Value;

#[derive(Debug)]
pub enum MemoryCell {
    NotAllocated,
    AllocatedCell(AllocatedCell)
}

#[derive(Debug)]
pub struct AllocatedCell {
    mutable: bool,
    value: Option<Value>,
}

impl MemoryCell {

    /// a modifier
    // pub fn new() -> Self { MemoryCell::NotAllocated }

    pub fn new_initialized(mutable: bool, value: Value) -> Self {
        MemoryCell::AllocatedCell( AllocatedCell { mutable, value: Some(value) } )
    }

    pub fn new_uninitialized() -> Self {
        MemoryCell::AllocatedCell( AllocatedCell { mutable: true, value: None } )
    }

    pub fn is_mutable(&self) -> bool {
        match self {
            MemoryCell::NotAllocated => false,
            MemoryCell::AllocatedCell(ac) => ac.is_mutable()
        }
    }

    pub fn get_value(&self) -> Result<Value, EvalError> {
        match self {
            MemoryCell::NotAllocated => Err(EvalError::NonAllocatedCell(None)),
            MemoryCell::AllocatedCell(ac) =>
                match &ac.value {
                    None => todo!(),
                    Some(Value::Unit) => Ok(Value::Unit),
                    Some(Value::Integer(i)) => Ok(Value::Integer(*i)),
                    Some(Value::Boolean(b)) => Ok(Value::Boolean(*b)),
                    Some(Value::Pointer(a)) => Ok(Value::Pointer(a.clone()))
                }
        }
    }

    pub fn set_value(&mut self, v: Value) -> Result<(), EvalError> {
        match self {
            MemoryCell::NotAllocated => Err(EvalError::NonAllocatedCell(None)),
            MemoryCell::AllocatedCell(ac) => {
                if !ac.is_mutable() { return Err(EvalError::NotMutable(None)) }
                ac.value = Some(v);
                Ok(())
            }
        }
    }

    pub fn is_allocated(&self) -> bool {
        match self {
            MemoryCell::NotAllocated => false,
            MemoryCell::AllocatedCell(_) => true
        }
    }
}

impl AllocatedCell {
    pub fn is_mutable(&self) -> bool { self.mutable }
}