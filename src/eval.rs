// use std::any::Any;
use crate::identifier::Identifier;
use crate::parsing::expression::Expression;
use crate::parsing::instruction::Instruction;
use crate::pointer::Pointer;
use crate::value::Value;
use Expression::*;
use crate::parsing::binop::Binop;
use crate::error::EvalError;
use crate::memory::{ Address, Memory };
use crate::namespace::NameSpace;
use crate::r#type::Type;


impl Expression {

    fn eval_and_cast_to_int(&self, mem: &mut Memory) -> Result<isize, EvalError> {
        let v = self.eval(mem)?;
        v.to_int()
         .map_err(|_| EvalError::TypeMismatch{
            expression: self.clone(), 
            expected: Type::Int, 
            found: Some(Type::from(&v))})
    }

    pub fn eval(&self, mem: &mut Memory) -> Result<Value, EvalError> {
        match self {
            Const(v) => Ok(Value::from(*v)),
            Expression::Identifier(id) => Ok(mem.find(id)?),
            BinOp(e1, Binop::Add, e2) => {
                let v1 = e1.eval_and_cast_to_int(mem)?;
                let v2 = e2.eval_and_cast_to_int(mem)?;
                Ok(Value::Integer(v1 + v2))
            },
            BinOp(_,_,_) => todo!(),
            Conditional{ .. } => todo!(),
            NewPtr => Ok(Value::Pointer(Pointer::new(Expression::NewPtr.eval_to_address(mem)?))),

            Deref(id) => { 
                let val = id.eval(mem)?;
                match val {
                    Value::Pointer(addr) => {
                        let res = mem.value_at(addr.get_address());
                        match res {
                            Ok(_) => res,
                            Err(EvalError::NonInitializedValue(_)) => Err(EvalError::NonInitializedValue(Some(*id.clone()))),
                            Err(EvalError::NonAllocatedCell(_)) => Err(EvalError::NonAllocatedCell(Some(*id.clone()))),
                            _ => unreachable!()
                        }
                    },
                    _ => Err(EvalError::TypeMismatch{expression: self.clone(), expected: Type::Pointer, found: Some(Type::from(&val))})
                }
            },
            
            AmpersAnd(p) => Ok(Value::Pointer(Pointer::new(p.eval_to_address(mem)?))),
        }
    }

    fn eval_to_address(&self, mem: &mut Memory) -> Result<Address, EvalError> {
        match self {
            Expression::NewPtr => Ok(mem.malloc()),
            Expression::Identifier(i) => mem.get_address(i),
            _ => todo!()
        }
    }
}


impl Instruction {
    pub fn exec(&self, mem: &mut Memory) -> Result<(Option<Identifier>, Value), EvalError> {
        match self {
            Instruction::Let{id, mutable, expr} => {
                let v_temp = expr.eval(mem)?;
                mem.declare(id, *mutable, v_temp.clone())?;
                Ok((Some(id.clone()), v_temp))
            },
            Instruction::Expr(expr) => {
                Ok((None, expr.eval(mem)?))
            }
            Instruction::Block(instrs) => {
                mem.push(NameSpace::new());
                let mut return_value = Value::Unit;
                for instr in instrs {
                    let (_, val) = instr.exec(mem).map_err(|err| {mem.pop(); err})?;
                    return_value = val;
                };
                mem.pop();
                Ok((None, return_value))
            }
            Instruction::IfElse { .. } => todo!(),
            Instruction::While(_, _) => todo!(),

            Instruction::WriteAt(e1, e2) => {
                let mut res_final: Result<(Option<Identifier>, Value), EvalError> = Ok((None, Value::Unit));
                match e1 {
                    Expression::Deref(id) => {
                        let val = id.eval(mem)?;
                        match val {
                            Value::Pointer(addr) => { 
                                let r_val = e2.eval(mem)?; 
                                let res = mem.write_at(addr.get_address(), r_val);
                                match res {
                                    Ok(_) => (),
                                    Err(EvalError::NonAllocatedCell(_)) => res_final = Err(EvalError::NonAllocatedCell(Some(e1.clone()))),
                                    Err(EvalError::NotMutable(_)) => res_final = Err(EvalError::NotMutable(Some(e1.clone()))),
                                    _ => unreachable!()
                                }
                            },
                            _ => todo!()
                        }
                    }
                    Expression::Identifier(id) => {
                        let val = e2.eval(mem)?;
                        let res = mem.write_var(&id, &val);
                        match res {
                            Ok(_) => (),
                            Err(EvalError::NonAllocatedCell(_)) => res_final = Err(EvalError::NonAllocatedCell(Some(e1.clone()))),
                            Err(EvalError::NotMutable(_)) => res_final = Err(EvalError::NotMutable(Some(e1.clone()))),
                            Err(EvalError::Undefined(id)) => res_final = Err(EvalError::Undefined(id.clone())),
                            Err(EvalError::TypeMismatch { .. }) => todo!(),
                            _ => unreachable!()
                        }
                    }
                    _ => todo!()
                }
                res_final
            },
            
            Instruction::Free(e) => {
                let id_val = e.eval(mem)?;
                mem.free(&id_val)?;
                Ok((None, Value::Unit))
            },
        }
    }
}