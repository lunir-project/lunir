// MIT License

// Copyright (c) 2023 lunir-project

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

// TODO: Remove
#![allow(unused)]

use crate::{
    ast::{expression::*, statement::*},
    il::{Constant, Function, Instruction, Table, Value},
    prelude::OptVariable,
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
                .map(|(index, expression)| format!("{index}:{expression:8?}"))
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

    fn shadow(&self, idx: usize) -> Result<Expression> {
        self.verify_stack_index(idx)?;

        let expression = self.0.get(idx).unwrap();

        Ok(Expression::Shadow(Rc::new(
            ShadowExpression::new(expression.clone()).shadow(true),
        )))
    }

    fn verify_stack_index(&self, index: usize) -> Result<()> {
        if index >= self.0.len() {
            return Err(anyhow!("Expression stack index {index} out of range"));
        }

        Ok(())
    }
}

pub(crate) struct Lifter {
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
                    let mut result = Vec::<(Expression, Expression)>::with_capacity(map.len());

                    for (k, v) in map.iter() {
                        result.push((self.value_to_ast(k)?, self.value_to_ast(v)?));
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

    pub fn lift(&mut self) -> Result<Box<StatBlock>> {
        let mut result = Box::<StatBlock>::default();

        for instr in self.instructions.clone().iter() {
            match instr.clone() {
                Instruction::Load(inst) => {
                    self.expression_stack.verify_stack_index(inst.dest)?;

                    *self.expression_stack.get_local_mut(inst.dest)? =
                        self.value_to_ast(&inst.src)?;
                }

                Instruction::GetGlobal(inst) => {
                    self.expression_stack.verify_stack_index(inst.dest)?;

                    *self.expression_stack.get_local_mut(inst.dest)? =
                        self.constant_index_to_ast(inst.constant)?;
                }

                Instruction::Call(inst) => {
                    let function = self.expression_stack.get_local(inst.callee)?;
                    let nargs = if let OptVariable::Number(n) = inst.num_args {
                        n
                    } else {
                        self.function.max_stack_size as usize - inst.callee
                    };

                    let mut arguments = Vec::<Expression>::with_capacity(nargs);
                    for i in (inst.callee + 1 + inst.self_call as usize..inst.callee + nargs) {
                        arguments.push(self.expression_stack.get_local(i)?.clone());
                    }

                    for i in (inst.callee + inst.self_call as usize
                        ..self.function.max_stack_size as usize)
                    {
                        self.expression_stack.shadow(i)?;
                    }

                    let call_expression = Rc::new(CallExpression {
                        arguments,
                        function: function.clone(),
                        is_self: inst.self_call,
                    });

                    if let OptVariable::Variable = inst.num_returns {
                        result
                            .body
                            .push(Expression::Call(call_expression).to_statement());
                    } else {
                        *self.expression_stack.get_local_mut(inst.callee)? =
                            Expression::Call(call_expression);
                    }
                }

                Instruction::Return(inst) => {
                    self.expression_stack
                        .verify_stack_index(inst.result_start + inst.result_count)?;
                    let mut return_stat = Box::<StatReturn>::default();

                    for stack_index in inst.result_start..(inst.result_start + inst.result_count) {
                        return_stat
                            .results
                            .push(self.expression_stack.get_local(stack_index)?.clone());
                    }

                    for idx in inst.result_start + 1..self.function.max_stack_size as usize {
                        self.expression_stack.shadow(idx)?;
                    }

                    result.body.push(Statement::StatReturn(return_stat));
                }

                _ => return Err(anyhow!("Instruction {instr:?} not supported")),
            }
        }

        Ok(result)
    }
}
