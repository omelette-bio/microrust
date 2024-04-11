use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Integer(isize),
    Boolean(bool),
    Unit,
}

impl fmt::Display for Value {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Integer(i) => write!(f, "{}", i),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Unit => write!(f, "()"),
        }
    }
}

use crate::{parsing::parsedvalue::ParsedValue, r#type::Type};

impl From<ParsedValue> for Value {
    fn from(pv: ParsedValue) -> Self {
        match pv {
            ParsedValue::Integer(i) => Value::Integer(i),
            ParsedValue::Boolean(b) => Value::Boolean(b),
            ParsedValue::Unit => Value::Unit,
        }
    }
}

impl From<&Value> for Type {
    fn from(v: &Value) -> Self {
        match v {
            Value::Integer(_) => Type::Int,
            Value::Boolean(_) => Type::Bool,
            Value::Unit => Type::Unit,
        }
    }
}

impl Value {
    pub fn to_int(&self) -> Result<isize, Type> {
        match self {
            Value::Integer(i) => Ok(*i),
            _ => Err(Type::from(self)),
        }
    }
    pub fn to_bool(&self) -> Result<bool, Type> {
        match self {
            Value::Boolean(b) => Ok(*b),
            _ => Err(Type::from(self)),
        }
    }
}