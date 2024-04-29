use crate::memory::Address;
use crate::memorycell::MemoryCell;
use crate::value::Value;
use crate::error::EvalError;

#[derive(Debug)]
pub struct Heap(Vec<MemoryCell>);

impl Heap {
    /// fonction prise du cours
    pub fn new() -> Self { Heap( Vec::new() ) }

    /// fonction prise du cours
    pub fn malloc(&mut self) -> Address {
        // un allocateur qui essaie d'utiliser un emplacement pris par une cellule
        // non allouée, sinon augmente la taille du tas (pour simplifier, le tas est "infini")

        for addr in 0..self.0.len() {
            if !self.0[addr].is_allocated() {
                self.0[addr] = MemoryCell::new_uninitialized();
                return Address::HeapAddress(addr)
            }
        }
        self.0.push(MemoryCell::new_uninitialized());
        return Address::HeapAddress(self.0.len() - 1);
    }

    pub fn free(&mut self, a: usize) { self.0[a] = MemoryCell::NotAllocated }

    pub fn get(&self, index: usize) -> Result<Value, EvalError> { 
        if self.0.len() <= index { return Err(EvalError::NonAllocatedCell(None)) }
        if !self.0[index].is_allocated() { return Err(EvalError::NonAllocatedCell(None)) }
        self.0[index].get_value()
    }

    pub fn set(&mut self, index: usize, value: Value) -> Result<(), EvalError> {
        if self.0.len() <= index { return Err(EvalError::NonAllocatedCell(None)) }
        self.0[index].set_value(value)
    }
}