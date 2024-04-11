use crate::identifier::Identifier;
use crate::parsing::expression::Expression;
use crate::parsing::instruction::Instruction;
use crate::value::Value;
use Expression::*;
use crate::parsing::binop::Binop;
use crate::error::EvalError;
use crate::namespace::NameSpace;
use crate::namespacestack::NameSpaceStack;
use crate::r#type::Type;


impl Expression {

    fn eval_and_cast_to_int(&self, nss: &mut NameSpaceStack) -> Result<isize, EvalError> {
        let v = self.eval(nss)?;
        v.to_int()
         .map_err(|_| EvalError::TypeMismatch{
            expression: self.clone(), 
            expected: Type::Int, 
            found: Some(Type::from(&v))})
    }

    pub fn eval(&self, nss: &mut NameSpaceStack) -> Result<Value, EvalError> {

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
            NewPtr => todo!(),
            Deref(_) => todo!(),
            AmpersAnd(_) => todo!(),
            _ => todo!()
        }
    }
}


impl Instruction {
    pub fn exec(&self, nss: &mut NameSpaceStack) -> Result<(Option<Identifier>, Value), EvalError> {
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
            Instruction::WriteAt(e1, e2) => todo!(),
            Instruction::Free(e) => todo!(),
        }
    }
}