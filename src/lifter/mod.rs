use crate::{
    ast::{expression::*, statement::{*, self}},
    il::{Constant, Function, Instruction, Table, Value, Return},
};

use anyhow::{anyhow, Result};
use std::{
    collections::{BTreeMap},
    rc::Rc,
};

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

    fn get_local_mut(&mut self, idx: usize) -> Result<&mut Expression> {
        self.0.get_mut(idx).ok_or(anyhow!(
            "No local reference at index {idx} on expression stack"
        ))
    }
    
    fn get_local(&self, idx: usize) -> Result<&Expression> {
        self.0.get(idx).ok_or(anyhow!(
            "No local reference at index {idx} on expression stack"
        ))
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
    fn make_function(&self, _func: &Function) -> Rc<FunctionExpression> {
        todo!();
    }

    fn constant_to_ast(&self, constant: &Constant) -> Result<Expression> {
        match constant {
            Constant::Boolean(b) => Ok(Expression::Boolean(Rc::new(Boolean(*b)))),
            Constant::Nil => Ok(Expression::Nil(Rc::new(Nil))),
            Constant::String(ref s) => Ok(Expression::String(Rc::new(Str(s.clone())))),
            Constant::Number(n) => Ok(Expression::Number(Rc::new(Number(*n)))),
            Constant::Function(f) => Ok(Expression::Function(self.make_function(f))),
            Constant::Table(t) => match t {
                Table::Array(a) => {
                    let vec = a
                        .iter()
                        .map(|c| self.value_to_ast(c).map_err(|e| anyhow!("{e:?}")))
                        .collect::<Vec<_>>();

                    if vec.iter().any(|e| e.is_err()) {
                        return Err(anyhow!("Unimplemented value in table"));
                    }

                    let vec = vec.into_iter().map(|c| c.unwrap()).collect::<Vec<_>>();
                    Ok(Expression::Table(Rc::new(TableExpression::Array(vec))))
                }

                Table::Map(map) => {
                    let mut result = BTreeMap::new();

                    for (k, v) in map.iter() {
                        result.insert(self.value_to_ast(k)?, self.value_to_ast(v)?);
                    }

                    Ok(Expression::Table(Rc::new(TableExpression::HashMap(result))))
                }
            },
        }
    }

    fn constant_index_to_ast(&self, idx: usize) -> Result<Expression> {
        match self.function.constants.get(idx) {
            Some(c) => self.constant_to_ast(c),
            None => Err(anyhow!("Constant index {idx} out of range")),
        }
    }

    fn value_to_ast(&self, value: &Value) -> Result<Expression> {
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
        let  mut result = Box::<StatBlock>::default();

        for instr in self.instructions.clone().iter() {
            match instr.clone() {
                Instruction::Load(inst) => {
                    self.verify_stack_index(inst.dest)?;

                    *self.expression_stack.get_local_mut(inst.dest)? =
                        self.value_to_ast(&inst.src)?;
                }

                Instruction::GetGlobal(inst) => {
                    self.verify_stack_index(inst.dest)?;

                    *self.expression_stack.get_local_mut(inst.dest)? =
                        self.constant_index_to_ast(inst.constant)?;
                }
                
                Instruction::Return(inst) => {
                    self.verify_stack_index(inst.result_start + inst.result_count)?;
                    let mut return_stat = Box::<StatReturn>::default();

                    for stack_index in inst.result_start..(inst.result_start + inst.result_count) {
                        return_stat.results.push(self.expression_stack.get_local(stack_index)?.clone());
                    }
                    result.body.push(Statement::StatReturn(return_stat));
                }

                _ => return Err(anyhow!("Instruction {:?} not supported", instr)),
            }
        }

        Ok(result)
    }
}
