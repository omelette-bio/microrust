// LISTE DES MODULES
mod parsing;
mod parser;
mod identifier;
mod error;
mod value;
mod namespace;
mod namespacestack;
mod eval;
mod r#type;
mod memorycell;
mod memory;
mod heap;


// LISTE DES IMPORTS
use std::io::{self, BufRead, Write};
use identifier::Identifier;
use parsing::instruction::Instruction;
use namespace::NameSpace;
use namespacestack::NameSpaceStack;
use value::Value;
use crate::parser::Parse;
use crate::error::Error;
use crate::memory::Memory;
use crate::r#type::Type;

// AFFICHAGE DU PROMPT
fn prompt() {
    print!("µRust # ");
    io::stdout().flush().unwrap();
}


// fn parse_eval(input: &str, ns: &NameSpace) -> Result<isize, Error> {
//     match Expression::parse(input) {
//         Ok(expr) => Ok(expr.eval(ns)?),
//         Err(e) => Err(Error::ParseError(e)),
//     }
// }

fn parse_exec(input: &str, nss: &mut Memory) -> Result<(Option<Identifier>, Value), Error> {
    match Instruction::parse(input) {
        Ok(instr) => {
            instr.exec(nss).map_err(|err| Error::EvalError(err))
        }
        Err(e) => Err(Error::ParseError(e)),
    }
}

// FONCTION PRINCIPALE
fn main(){
    prompt();
    let mut nss = Memory::new();
    nss.push(NameSpace::new());
    let stdin = io::stdin().lock();
    for line in stdin.lines() {
        let line = line.unwrap();
        match parse_exec(&line, &mut nss) {
            Ok((id, val)) => {
                println!("{} : {} = {}", id.unwrap_or(Identifier::from("-")), Type::from(&val), val);
            }
            Err(e) => {
                println!("{}", e);
            }
        }
        prompt();
    }
}
