use super::binop::Binop;
use super::parsedvalue::ParsedValue;

#[derive(Debug, Clone)]
pub enum Expression {
    Const(ParsedValue),
    Identifier(Identifier),
    BinOp(Box<Expression>, Binop, Box<Expression>),
    Conditional{
        cond: Box<Expression>,
        cond_true: Box<Expression>,
        cond_false: Box<Expression>,
    },
    NewPtr,
    Deref(Box<Expression>),
    AmpersAnd(Box<Expression>),
}


use std::fmt::Display;

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Expression::*;
        match self {
            Const(i) => write!(f, "{}", i),
            BinOp(lhs, op, rhs) => write!(f, "({} {} {})", lhs, op, rhs),
            Conditional { cond, cond_true, cond_false } => 
                write!(f, "({}) ? {}  : {} ", cond, cond_true, cond_false),
            Identifier(id) => write!(f, "{}", id),
            NewPtr => write!(f,  "Ptr::new()"),
            Deref(e) => write!(f, "*{}", e),
            AmpersAnd(e) => write!(f, "&{}", e),
        }
    }
}


use pest::Parser;

use super::utils::{ PestParser, Rule, parse_expr};
use crate::{identifier::Identifier, parser::{ Parse, ParseError}};


impl Parse for Expression {

    fn parse(input: &str) -> Result<Self, ParseError> {
        match PestParser::parse(Rule::start_rule_expr, &input) {
            Ok(mut pairs) => {
                let first_rule = pairs.next().unwrap();
                match first_rule.as_rule() {
                    Rule::expr => {
                        Ok(parse_expr(first_rule.into_inner()))
                    }
                    _ => { panic!("the grammar is not as expected") }
                }                
            },
            Err(_e) => { Err(ParseError::CannotParse) }
        }
    }
}