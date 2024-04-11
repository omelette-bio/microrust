use std::collections::HashMap;
use crate::{error::EvalError, identifier::Identifier, parsing::expression::Expression, value::Value, memorycell::MemoryCell};

#[derive(Debug)]
pub struct NameSpace(HashMap<Identifier, MemoryCell>);



impl NameSpace {
    pub fn new() -> Self {
        NameSpace(HashMap::new())
    }

    /// declare a new memory cell
    pub fn declare(&mut self, id: &Identifier, mutable: bool, value: Value) -> Result<(), EvalError> {
//        self.0.try_insert(id, value).map_err(|_| EvalError::AlreadyDefined(id))
        if self.0.contains_key(&id) {
            Err(EvalError::AlreadyDefined(id.clone()))
        } else {
            self.0.insert(id.clone(), MemoryCell::new(mutable, value));
            Ok(())
        }
    }

    /// search the value of a specific memory cell, return a result with either the value or an error
    pub fn find(&self, id: &Identifier) -> Result<Value, EvalError> {
        match self.0.get(id) {
            Some(mc) => Ok(mc.get_value()?),
            None => Err(EvalError::Undefined(id.clone())),
        }
    }

    /// set the value of a memory cell
    pub fn set(&mut self, id: &Identifier, value: Value) -> Result<(), EvalError> {
        match self.0.get_mut(id) {
            Some(mc) => {
                if mc.is_mutable() {
                    mc.set_value(value).expect("");
                    Ok(())
                } else {
                    Err(EvalError::NotMutable(Some(Expression::Identifier(id.clone()))))
                }
            }
            None => Err(EvalError::Undefined(id.clone())),
        }
    }

    /// returns true if the namespace has an identifier
    pub fn contains(&self, id: &Identifier) -> bool {
        self.0.contains_key(id)
    }
}


/* 
#[cfg(test)]
mod test_namespace {

    use super::*;

    #[test]
    fn test_declare() {
        let mut ns = NameSpace::new();
        let id = Identifier::from("x");
        assert!(ns.declare(&id, false, 42).is_ok());
        match ns.declare(&id, false, 42) {
            Ok(_) => panic!("Identifier should not be declared twice"),
            Err(EvalError::AlreadyDefined(id2)) => assert_eq!(id, id2),
            Err(_) => panic!("Unexpected error"),
        }
    }

    #[test]
    fn test_find() {
        let mut ns = NameSpace::new();
        let id = Identifier::from("x");
        match ns.find(&id) {
            Ok(_) => panic!("Identifier should not be found"),
            Err(EvalError::Undefined(id2)) => assert_eq!(id, id2),
            Err(_) => panic!("Unexpected error"),
        }
        assert!(ns.declare(&id, false, 42).is_ok());
        match ns.find(&id) {
            Ok(42) => (),
            _ => panic!("Identifier should be found"),
        }
    }

}

*/
