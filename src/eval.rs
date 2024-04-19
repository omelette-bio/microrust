// use std::any::Any;
use crate::identifier::Identifier;
use crate::parsing::expression::Expression;
use crate::parsing::instruction::Instruction;
use crate::value::Value;
use Expression::*;
use crate::parsing::binop::Binop;
use crate::error::EvalError;
use crate::memory::{ Address, Memory };
use crate::namespace::NameSpace;
use crate::r#type::Type;


impl Expression {

    fn eval_and_cast_to_int(&self, nss: &mut Memory) -> Result<isize, EvalError> {
        let v = self.eval(nss)?;
        v.to_int()
         .map_err(|_| EvalError::TypeMismatch{
            expression: self.clone(), 
            expected: Type::Int, 
            found: Some(Type::from(&v))})
    }

    pub fn eval(&self, nss: &mut Memory) -> Result<Value, EvalError> {
        match self {
            Const(v) => Ok(Value::from(*v)),
            Expression::Identifier(id) => Ok(nss.find(id)?),
            BinOp(e1, Binop::Add, e2) => {
                let v1 = e1.eval_and_cast_to_int(nss)?;
                let v2 = e2.eval_and_cast_to_int(nss)?;
                Ok(Value::Integer(v1 + v2))
            },
            BinOp(_,_,_) => todo!(),
            Conditional{cond, cond_true, cond_false} => todo!(),
            NewPtr => Ok(Value::Pointer(Expression::NewPtr.eval_to_address(nss)?)),

            Deref(id) => { 
                let val = id.eval(nss)?;
                match val {
                    Value::Pointer(addr) => nss.value_at(&addr),
                    _ => Err(EvalError::TypeMismatch{expression: self.clone(), expected: Type::Pointer, found: Some(Type::from(&val))})
                }
            },
            
            AmpersAnd(p) => Ok(Value::Pointer(p.eval_to_address(nss)?)),
            _ => todo!()
        }
    }

    fn eval_to_address(&self, nss: &mut Memory) -> Result<Address, EvalError> {
        match self {
            Expression::NewPtr => Ok(nss.malloc()),
            Expression::Identifier(i) => nss.get_address(i),
            _ => todo!()
        }
    }
}


impl Instruction {
    pub fn exec(&self, nss: &mut Memory) -> Result<(Option<Identifier>, Value), EvalError> {
        match self {
            Instruction::Let{id, mutable, expr} => {
                let v_temp = expr.eval(nss)?;
                nss.declare(id, *mutable, v_temp.clone())?;
                Ok((Some(id.clone()), v_temp))
            },
            Instruction::Expr(expr) => {
                Ok((None, expr.eval(nss)?))
            }
            Instruction::Block(instrs) => {
                nss.push(NameSpace::new());
                let mut return_value = Value::Unit;
                for instr in instrs {
                    let (_id, val) = instr.exec(nss).map_err(|err| {nss.pop(); err})?;
                    return_value = val;
                };
                nss.pop();
                Ok((None, return_value))
            }
            Instruction::IfElse { cond, cond_true, cond_false } => todo!(),
            Instruction::While(e, instr) => todo!(),

            Instruction::WriteAt(e1, e2) => {
                match e1 {
                    Expression::Deref(id) => {
                        let val = id.eval(nss)?;
                        match val {
                            Value::Pointer(addr) => { let r_val = e2.eval(nss)?; nss.write_at(&addr, r_val)?; },
                            _ => todo!()
                        }
                    }
                    Expression::Identifier(id) => {
                        let val = e2.eval(nss)?;
                        nss.write_var(&id, &val)?;
                    }
                    _ => todo!()
                }
                Ok((None, Value::Unit))
            },
            
            Instruction::Free(e) => {
                let id_val = e.eval(nss)?;
                nss.free(&id_val)?;
                Ok((None, Value::Unit))
            },
        }
    }
}