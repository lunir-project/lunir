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

use super::{expression::*, statement::*};

#[derive(Debug, Default)]
pub enum NodeKind {
    Expression(Expression),
    Statement(Statement),

    #[default]
    None,
}

pub trait Visitor<'a>: Sized {
    fn visit_expr(&mut self, node: &'a Expression) {
        walk_expression(self, node)
    }

    fn visit_table(&mut self, node: &'a TableExpression) {
        walk_table(self, node)
    }

    fn visit_call(&mut self, node: &'a CallExpression) {
        walk_call(self, node)
    }

    fn visit_function(&mut self, node: &'a FunctionExpression) {
        walk_function(self, node)
    }

    fn visit_binary(&mut self, node: &'a BinaryExpression) {
        walk_binary(self, node)
    }

    fn visit_unary(&mut self, node: &'a UnaryExpression) {
        walk_unary(self, node)
    }

    fn visit_shadowed(&mut self, node: &'a ShadowExpression) {
        if !node.is_shadowed {
            self.visit_expr(&node.value);
        }
    }

    fn visit_index_op(&mut self, node: &'a IndexOp) {
        walk_index_op(self, node)
    }

    fn visit_stat(&mut self, node: &'a Statement) {
        walk_statement(self, node)
    }

    fn visit_stat_block(&mut self, node: &'a StatBlock) {
        walk_stat_block(self, node)
    }

    fn visit_return(&mut self, node: &'a StatReturn) {
        walk_return(self, node)
    }

    fn visit_bool(&mut self, node: &'a Boolean);
    fn visit_string(&mut self, node: &'a Str);
    fn visit_number(&mut self, node: &'a Number);
    fn visit_nil(&mut self, node: &'a Nil);
    fn visit_identifier(&mut self, node: &'a Identifier);
    fn visit_global_symbol(&mut self, node: &'a GlobalSymbol);
}

macro_rules! walk_all {
    ($visitor: expr, $func: ident, $list: expr) => {
        for item in $list {
            $visitor.$func(item)
        }
    };
}

pub fn walk_expression<'a, V: Visitor<'a>>(visitor: &mut V, node: &'a Expression) {
    match node {
        Expression::BinaryOp(binary) => visitor.visit_binary(binary),
        Expression::Boolean(b) => visitor.visit_bool(b),
        Expression::Call(call) => visitor.visit_call(call),
        Expression::Function(func) => visitor.visit_function(func),
        Expression::GlobalSymbol(ident) => visitor.visit_global_symbol(ident),
        Expression::Identifier(ident) => visitor.visit_identifier(ident),
        Expression::IndexOp(index) => visitor.visit_index_op(index),
        Expression::Nil(n) => visitor.visit_nil(n),
        Expression::Number(n) => visitor.visit_number(n),
        Expression::String(s) => visitor.visit_string(s),
        Expression::Table(table) => visitor.visit_table(table),
        Expression::UnaryOp(unary) => visitor.visit_unary(unary),
        Expression::Shadow(shadow) => visitor.visit_shadowed(shadow),
    }
}

pub fn walk_shadowed<'a, V: Visitor<'a>>(visitor: &mut V, node: &'a ShadowExpression) {
    if !node.is_shadowed {
        walk_expression(visitor, &node.value);
    }
}

pub fn walk_statement<'a, V: Visitor<'a>>(visitor: &mut V, node: &'a Statement) {
    match node {
        Statement::StatBlock(block) => visitor.visit_stat_block(block.as_ref()),

        Statement::StatExpr(expr) => {
            visitor.visit_expr(&expr.value);
        }

        Statement::StatReturn(ret) => visitor.visit_return(ret.as_ref()),
    }
}

pub fn walk_stat_block<'a, V: Visitor<'a>>(visitor: &mut V, node: &'a StatBlock) {
    walk_all!(visitor, visit_stat, &node.body);
}

pub fn walk_return<'a, V: Visitor<'a>>(visitor: &mut V, node: &'a StatReturn) {
    walk_all!(visitor, visit_expr, &node.results);
}

pub fn walk_binary<'a, V: Visitor<'a>>(visitor: &mut V, node: &'a BinaryExpression) {
    walk_expression(&mut *visitor, &node.left);
    walk_expression(&mut *visitor, &node.right);
}

pub fn walk_index_op<'a, V: Visitor<'a>>(visitor: &mut V, node: &'a IndexOp) {
    walk_expression(visitor, &node.table);
    walk_expression(visitor, &node.key);
}

pub fn walk_unary<'a, V: Visitor<'a>>(visitor: &mut V, node: &'a UnaryExpression) {
    walk_expression(visitor, &node.value);
}

pub fn walk_table<'a, V: Visitor<'a>>(visitor: &mut V, node: &'a TableExpression) {
    match node {
        TableExpression::Array(array) => walk_all!(visitor, visit_expr, array),
        TableExpression::HashMap(map) => {
            for (k, v) in map {
                visitor.visit_expr(k);
                visitor.visit_expr(v);
            }
        }
    }
}

pub fn walk_call<'a, V: Visitor<'a>>(visitor: &mut V, node: &'a CallExpression) {
    visitor.visit_expr(&node.function);
    walk_all!(visitor, visit_expr, &node.arguments);
}

pub fn walk_function<'a, V: Visitor<'a>>(visitor: &mut V, node: &'a FunctionExpression) {
    if let Some(expr) = &node.self_arg {
        visitor.visit_expr(expr);
    }

    walk_all!(visitor, visit_expr, &node.parameters);
}

#[derive(Debug, Default)]
pub struct Node {
    pub data: Box<NodeKind>,
    pub children: Vec<Node>,
}
