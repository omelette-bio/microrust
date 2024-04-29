// use std::any::Any;
use crate::identifier::Identifier;
use crate::parsing::expression::Expression;
use crate::parsing::instruction::Instruction;
use crate::pointer::Pointer;
use crate::value::Value;
use Expression::*;
use crate::parsing::binop::Binop;
use crate::error::EvalError;
use crate::error::EvalError::TypeMismatch;
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
    fn eval_and_cast_to_bool(&self, mem: &mut Memory) -> Result<bool, EvalError> {
        let v = self.eval(mem)?;
        v.to_bool()
            .map_err(|_| EvalError::TypeMismatch{
                expression: self.clone(),
                expected: Type::Bool,
                found: Some(Type::from(&v))})
    }

    pub fn eval(&self, mem: &mut Memory) -> Result<Value, EvalError> {
        match self {
            Const(v) => Ok(Value::from(*v)),
            Expression::Identifier(id) => Ok(mem.find(id)?),

            BinOp(lhs, Binop::Add, rhs) => {
                let v1 = lhs.eval_and_cast_to_int(mem)?;
                let v2 = rhs.eval_and_cast_to_int(mem)?;
                Ok(Value::Integer(v1 + v2))
            },
            BinOp(lhs, Binop::Sub, rhs) => {
                let v1 = lhs.eval_and_cast_to_int(mem)?;
                let v2 = rhs.eval_and_cast_to_int(mem)?;
                Ok(Value::Integer(v1 - v2))
            }
            BinOp(lhs, Binop::Mul, rhs) => {
                let v1 = lhs.eval_and_cast_to_int(mem)?;
                let v2 = rhs.eval_and_cast_to_int(mem)?;
                Ok(Value::Integer(v1 * v2))
            }
            BinOp(lhs, Binop::Div, rhs) => {
                let v1 = lhs.eval_and_cast_to_int(mem)?;
                let v2 = rhs.eval_and_cast_to_int(mem)?;
                if v2 == 0 { return Err(EvalError::DivisionByZero(*rhs.clone())) }
                Ok(Value::Integer(v1 / v2))
            }
            BinOp(lhs, Binop::Mod, rhs) => {
                let v1 = lhs.eval_and_cast_to_int(mem)?;
                let v2 = rhs.eval_and_cast_to_int(mem)?;
                Ok(Value::Integer(v1 % v2))
            }

            BinOp(lhs, Binop::Eq, rhs) => {
                let v1 = lhs.eval(mem)?;
                let v2 = rhs.eval(mem)?;
                if Type::from(&v1) != Type::from(&v2) { return Err(EvalError::TypeMismatch {expression: *rhs.clone(), expected: Type::from(&v1), found: Some(Type::from(&v2))}) }
                match (v1, v2) {
                    (Value::Boolean(b1), Value::Boolean(b2)) => Ok(Value::Boolean(b1 == b2)),
                    (Value::Integer(i1), Value::Integer(i2)) => Ok(Value::Boolean(i1 == i2)),
                    _ => unreachable!()
                }
            }
            BinOp(lhs, Binop::Neq, rhs) => {
                let v1 = lhs.eval(mem)?;
                let v2 = rhs.eval(mem)?;
                if Type::from(&v1) != Type::from(&v2) { return Err(EvalError::TypeMismatch {expression: *rhs.clone(), expected: Type::from(&v1), found: Some(Type::from(&v2))}) }
                match (v1, v2) {
                    (Value::Boolean(b1), Value::Boolean(b2)) => Ok(Value::Boolean(b1 != b2)),
                    (Value::Integer(i1), Value::Integer(i2)) => Ok(Value::Boolean(i1 != i2)),
                    _ => unreachable!()
                }
            }


            BinOp(lhs, Binop::Leq, rhs) => {
                let v1 = lhs.eval_and_cast_to_int(mem)?;
                let v2 = rhs.eval_and_cast_to_int(mem)?;
                Ok(Value::Boolean(v1 <= v2))
            }
            BinOp(lhs, Binop::Geq, rhs) => {
                let v1 = lhs.eval_and_cast_to_int(mem)?;
                let v2 = rhs.eval_and_cast_to_int(mem)?;
                Ok(Value::Boolean(v1 >= v2))
            }
            BinOp(lhs, Binop::Lt, rhs) => {
                let v1 = lhs.eval_and_cast_to_int(mem)?;
                let v2 = rhs.eval_and_cast_to_int(mem)?;
                Ok(Value::Boolean(v1 < v2))
            }
            BinOp(lhs, Binop::Gt, rhs) => {
                let v1 = lhs.eval_and_cast_to_int(mem)?;
                let v2 = rhs.eval_and_cast_to_int(mem)?;
                Ok(Value::Boolean(v1 > v2))
            }
            BinOp(lhs, Binop::And, rhs) => {
                let v1 = lhs.eval_and_cast_to_bool(mem)?;
                let v2 = rhs.eval_and_cast_to_bool(mem)?;
                Ok(Value::Boolean(v1 && v2))
            }
            BinOp(lhs, Binop::Or, rhs) => {
                let v1 = lhs.eval_and_cast_to_bool(mem)?;
                let v2 = rhs.eval_and_cast_to_bool(mem)?;
                Ok(Value::Boolean(v1 || v2))
            }

            Conditional{ cond, cond_true, cond_false } => {
                let res = cond.eval_and_cast_to_bool(mem)?;
                if res { Ok(cond_true.eval(mem)?) }
                else { Ok(cond_false.eval(mem)?) }
            }
            NewPtr => Ok(Value::Pointer(Pointer::new(NewPtr.eval_to_address(mem)?))),

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
            NewPtr => Ok(mem.malloc()),
            Expression::Identifier(i) => mem.get_address(i),
            _ => Err(EvalError::TypeMismatch {expression: self.clone(), expected: Type::Pointer, found: None})
        }
    }
}


impl Instruction {
    #[allow(unused)]
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

            Instruction::IfElse { cond, cond_true, cond_false } => {
                let res = cond.eval_and_cast_to_bool(mem)?;
                if res { cond_true.exec(mem) }
                else { cond_false.exec(mem) }
            },

            Instruction::While(cond, inst) => {
                let mut res = cond.eval_and_cast_to_bool(mem)?;
                while res {
                    inst.exec(mem)?;
                    res = cond.eval_and_cast_to_bool(mem)?;
                }
                Ok((None, Value::Unit))
            },

            Instruction::WriteAt(e1, e2) => {
                let mut res_final: Result<(Option<Identifier>, Value), EvalError> = Ok((None, Value::Unit));
                match e1 {
                    Expression::Deref(id) => {
                        let val = id.eval(mem)?;
                        match val {
                            Value::Pointer(addr) => { 
                                let r_val = e2.eval(mem)?; 
                                let res = mem.write_at(addr.get_address(), r_val.clone());
                                match res {
                                    Ok(_) => res_final = Ok((None, r_val)),
                                    Err(EvalError::NonAllocatedCell(_)) => res_final = Err(EvalError::NonAllocatedCell(Some(e1.clone()))),
                                    Err(EvalError::NotMutable(_)) => res_final = Err(EvalError::NotMutable(Some(e1.clone()))),
                                    _ => unreachable!()
                                }
                            },
                            _ => res_final = Err(TypeMismatch {expression: *id.clone(), expected: Type::Pointer, found: Some(Type::from(&val))})
                        }
                    }
                    Expression::Identifier(id) => {
                        let val = e2.eval(mem)?;
                        let res = mem.write_var(&id, &val);
                        match res {
                            Ok(_) => res_final = Ok((Some(id.clone()), val.clone())),
                            Err(EvalError::NonAllocatedCell(_)) => res_final = Err(EvalError::NonAllocatedCell(Some(e1.clone()))),
                            Err(EvalError::NotMutable(_)) => res_final = Err(EvalError::NotMutable(Some(e1.clone()))),
                            Err(EvalError::Undefined(id)) => res_final = Err(EvalError::Undefined(id.clone())),
                            _ => unreachable!()
                        }
                    }
                    _ => unreachable!()
                }
                res_final
            },
            
            Instruction::Free(e) => {
                let id_val = e.eval(mem)?;
                match mem.free(&id_val) {
                    Ok(()) => Ok((None, Value::Unit)),
                    Err( EvalError::TypeMismatch { expression: _, expected, found }) =>
                        Err( EvalError::TypeMismatch {
                            expression: e.clone(),
                            expected,
                            found
                        }),
                    Err(EvalError::CannotFreeOwnedValue(_)) => Err( EvalError::CannotFreeOwnedValue(Some(e.clone()))),
                    _ => unreachable!()
                }
            },
        }
    }
}