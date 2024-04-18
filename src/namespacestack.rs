use crate::{error::EvalError, identifier::Identifier, namespace::NameSpace, value::Value, memory::Address};

#[derive(Debug)]
pub struct NameSpaceStack {
    stack: Vec<NameSpace>,
}

impl NameSpaceStack {
    pub fn new() -> Self {
        NameSpaceStack { stack: vec![] }
    }

    pub fn push(&mut self, ns: NameSpace) {
        self.stack.push(ns);
    }

    pub fn pop(&mut self) -> Option<NameSpace> {
        self.stack.pop()
    }

    pub fn declare(&mut self, id: &Identifier, mutable: bool, value: Value) -> Result<(), EvalError> {
        self.stack.last_mut().unwrap().declare(id, mutable, value)
    }

    pub fn find(&self, id: &Identifier) -> Result<Value, EvalError> {
        for ns in self.stack.iter().rev() {
            if let Ok(v) = ns.find(id) {
                return Ok(v);
            }
        }
        Err(EvalError::Undefined(id.clone()))
    }

    pub fn set(&mut self, id: &Identifier, value: Value) -> Result<(), EvalError> {
        for ns in self.stack.iter_mut().rev() {
            let res = ns.set(id, value.clone());
            match res {
                Ok(_) => return Ok(()),
                Err(EvalError::Undefined(_)) => (),
                Err(EvalError::NotMutable(_)) => return res,
                Err(EvalError::TypeMismatch{..}) => return res,
                _ => unreachable!(),
            }
        }
        Err(EvalError::Undefined(id.clone()))
    }

    pub fn get_address(&self, id: &Identifier) -> Result<Address, EvalError> {
        // renvoie l'adresse de pile d'un identifiant (cf opérateur `&x`)

        for index in  (0..self.stack.len()).rev() {
            if self.stack[index].contains(id) {
                return Ok(Address::StackAddress(index, id.clone()))
            }
        }
        Err(EvalError::Undefined(id.clone()))

    }
}

/*
#[cfg(test)]
mod test_namespace_stack {
    use super::*;

    #[test]
    fn test() {
        let mut ns = NameSpaceStack::new();
        ns.push(NameSpace::new());
        let x = Identifier::from("x");
        let y = Identifier::from("y");
        let z = Identifier::from("z");
        assert!(ns.declare(&x, false, 0).is_ok());
        assert!(ns.declare(&y, false, 0).is_ok());
        ns.push(NameSpace::new());
        assert!(ns.declare(&x, false, 1).is_ok());
        match ns.find(&x) {
            Ok(1) => (),
            _ => panic!("Expected 1"),
        }
        match ns.find(&y) {
            Ok(0) => (),
            _ => panic!("Expected 0"),
        }
        match ns.find(&z) {
            Err(EvalError::Undefined(id)) => assert_eq!(id, z),
            _ => panic!("Expected undefined"),
        }
    }
}

*/