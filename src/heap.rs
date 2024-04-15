use crate::memory::Address;
use crate::memorycell::MemoryCell;

pub struct Heap(Vec<MemoryCell>);
impl Heap {
    pub fn new() -> Self { Heap( Vec::new() ) }

    /// fonction prise du cours
    fn malloc(&mut self) -> Address {
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
}