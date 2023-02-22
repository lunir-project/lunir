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

// ok we need to write some expressions and statements
// ok

use crate::ast::{expression::Expression, statement::Statement};

pub enum NodeKind {
    Expression(Expression),
    Statement(Statement),
}

use super::{expression::*, statement::*};

pub trait Visitor<T: Node>: VisitorMut<T> {
    fn visit(&self, node: &T);
}

pub trait VisitorMut<T: Node> {
    fn visit_mut(&mut self, node: &T);
}

pub trait Node
where
    Self: Sized,
{
    fn accept(&self, visitor: &dyn Visitor<Self>) {
        visitor.visit(self);
    }

    fn accept_mut(&self, visitor: &mut dyn VisitorMut<Self>) {
        visitor.visit_mut(self);
    }
}

macro_rules! ast_nodes {
    ($($ty:ty),*) => {
        $(
            impl Node for $ty {}
        )*
    };
}

ast_nodes!(
    // Expressions
    Nil,
    Number,
    Boolean,
    Str,
    BinaryExpression,
    UnaryExpression,
    IndexOp,
    CallExpression,
    FunctionExpression,
    GlobalSymbol,
    Identifier,
    TableExpression,
    // Statements
    StatBlock,
    StatExpr,
    // Both
    Expression,
    Statement
);
