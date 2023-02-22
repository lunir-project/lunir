use crate::{
    ast::{expression::*, statement::*, tree::*},
    il::{Constant, Function, Instruction, Value},
};

use anyhow::{anyhow, Result};
use std::rc::Rc;

#[derive(Default)]
struct ExpressionStack(Vec<Expression>);

impl std::fmt::Debug for ExpressionStack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .enumerate()
                .rev()
                .map(|(index, expression)| format!("{index}:{expression:#?}"))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}
impl ExpressionStack {
    fn push_local(&mut self, expr: Expression) {
        // for now just push directly
        // TODO: Create a local reference here
        self.0.push(expr);
    }

    fn get_local(&mut self, idx: usize) -> Result<&mut Expression> {
        match self.0.get_mut(idx) {
            Some(expr) => Ok(expr),
            None => Err(anyhow!(
                "No local reference at index {idx} on expression stack"
            )),
        }
    }
}

pub struct Lifter {
    // Okay so we gotta just improvise as we go for the most part
    // I think we can have some pre-processing passes as well to identify locals and such
    // Well it has to be unless u just want a single giant function
    // I might have more in order to shadow certain variables and do some internals
    // might even use handlers but idk

    // im not entirely convinced with this being a struct to begin with, but we'll see
    // youll just have that function in the impl block xd
    // we'll see
    expression_stack: ExpressionStack,
    instructions: Vec<Instruction>,
    function: Function,
}

impl Lifter {
    fn make_function(&self, func: &Function) -> Rc<FunctionExpression> {
        todo!();
    }

    fn constant_to_ast(&self, constant: &Constant) -> Result<Expression> {
        match constant {
            Constant::Boolean(b) => Ok(Expression::Boolean(Rc::new(Boolean(*b)))),
            Constant::Nil => Ok(Expression::Nil(Rc::new(Nil))),
            Constant::String(ref s) => Ok(Expression::String(Rc::new(Str(s.clone())))),
            Constant::Number(n) => Ok(Expression::Number(Rc::new(Number(*n)))),
            Constant::Function(f) => Ok(Expression::Function(self.make_function(f))),
            Constant::Table(t) => {
                todo!()
            }
            _ => Err(anyhow!("Constant {:?} not implemented yet", constant)),
        }
    }

    fn constant_index_to_ast(&self, idx: usize) -> Result<Expression> {
        match self.function.constants.get(idx) {
            Some(c) => self.constant_to_ast(c),
            None => Err(anyhow!("Constant index {} out of range", idx)),
        }
    }

    fn value_to_ast(&mut self, value: &Value) -> Result<Expression> {
        match *value {
            Value::Boolean(b) => Ok(Expression::Boolean(Rc::new(Boolean(b)))),
            Value::ConstantIndex(i) => self.constant_index_to_ast(i),
            Value::Immediate(i) => Ok(Expression::Number(Rc::new(Number(i as f64)))),
            Value::Nil => Ok(Expression::Nil(Rc::new(Nil))),
            Value::StackIndex(i) => Ok(self.expression_stack.get_local(i)?.clone()),
        }
    }

    fn verify_stack_index(&self, index: usize) -> Result<()> {
        if index >= self.expression_stack.0.len() {
            return Err(anyhow!("Expression stack index {} out of range", index));
        }

        Ok(())
    }

    pub fn lift(&mut self) -> Result<Box<StatBlock>> {
        let mut result = Box::new(StatBlock::default());

        for instr in self.instructions.clone().iter() {
            match instr.clone() {
                Instruction::Load(inst) => {
                    self.verify_stack_index(inst.dest)?;

                    *self.expression_stack.get_local(inst.dest)? = self.value_to_ast(&inst.src)?;
                }

                Instruction::GetGlobal(inst) => {
                    self.verify_stack_index(inst.dest)?;

                    *self.expression_stack.get_local(inst.dest)? =
                        self.constant_index_to_ast(inst.constant)?;
                }

                _ => return Err(anyhow!("Instruction {:?} not supported", instr)),
            }
        }

        Ok(result)
    }
}
