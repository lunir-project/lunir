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

use crate::ast::statement::*;
use crate::il::UnaryOpKind;

use itertools::Itertools;
use std::rc::Rc;

pub trait IdentifierString {
    fn can_identify(&self) -> bool;
    fn sanitize_identifier(&self) -> String;
    fn sanitize_special(&self) -> String;
}

impl IdentifierString for String {
    fn can_identify(&self) -> bool {
        !(self.is_empty()
            || self.chars().next().unwrap().is_ascii_digit()
            || self.chars().any(|c| !c.is_alphanumeric() && c != '_'))
    }

    fn sanitize_identifier(&self) -> String {
        let mut result = self
            .chars()
            .filter_map(|c| match c {
                '-' | ' ' | '\t' | '\n' => Some('_'),
                _ if c.is_alphanumeric() || c == '_' => Some(c),
                _ => None,
            })
            .collect::<String>();

        result = if !result.is_empty() {
            result
        } else {
            format!("_{result}")
        };

        let result = result
            .chars()
            .dedup_by(|&a, &b| a == '_' && a == b)
            .collect::<String>();

        result
    }

    fn sanitize_special(&self) -> String {
        self.chars()
            .map(|c| match c {
                '\n' => r#"\n"#.to_string(),
                '\r' => r#"\r"#.to_string(),
                '\t' => r#"\t"#.to_string(),
                '\"' => r#"\""#.to_string(),
                '\\' => r#"\\"#.to_string(),
                c if (' '..='~').contains(&c) => c.to_string(),
                _ => format!(r#"\{}"#, c as u32),
            })
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Nil;

#[derive(Debug, Clone, PartialEq)]
pub struct Number(pub f64);

#[derive(Debug, Clone, PartialEq)]
pub struct Boolean(pub bool);

#[derive(Debug, Clone, PartialEq)]
pub struct Str(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct GlobalSymbol(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier(pub String);

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryExpressionKind {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Concat,

    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    ModAssign,
    PowAssign,
    ConcatAssign,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpression {
    pub kind: BinaryExpressionKind,

    pub left: Expression,
    pub right: Expression,
}

impl BinaryExpressionKind {
    pub fn to_compound(&self) -> Self {
        match self {
            Self::Add => Self::AddAssign,
            Self::Sub => Self::SubAssign,
            Self::Mul => Self::MulAssign,
            Self::Div => Self::DivAssign,
            Self::Mod => Self::ModAssign,
            Self::Pow => Self::PowAssign,
            Self::Concat => Self::ConcatAssign,
            other => other.clone(),
        }
    }
}

impl std::fmt::Display for BinaryExpressionKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Add => "+",
                Self::Sub => "-",
                Self::Mul => "*",
                Self::Div => "/",
                Self::Mod => "%",
                Self::Pow => "^",
                Self::Concat => "..",

                Self::AddAssign => "+=",
                Self::SubAssign => "-=",
                Self::MulAssign => "*=",
                Self::DivAssign => "/=",
                Self::ModAssign => "%=",
                Self::PowAssign => "^=",
                Self::ConcatAssign => "..=",
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryExpression {
    pub kind: UnaryOpKind,
    pub value: Expression,
}

impl std::fmt::Display for UnaryOpKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Len => "#",
                Self::Neg => "-",

                // space necessary
                Self::Not => "not ",
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TableExpression {
    HashMap(Vec<(Expression, Expression)>),
    Array(Vec<Expression>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ShadowExpression {
    pub is_shadowed: bool,
    pub value: Expression,
}

impl ShadowExpression {
    pub fn new(value: Expression) -> Self {
        Self {
            value,
            is_shadowed: false,
        }
    }

    pub fn shadow(mut self, value: bool) -> Self {
        self.is_shadowed = value;

        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IndexOp {
    pub key: Expression,
    pub table: Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallExpression {
    pub arguments: Vec<Expression>,
    pub function: Expression,

    pub is_self: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionExpression {
    pub body: Box<StatBlock>,
    pub has_vararg: bool,

    pub parameters: Vec<Expression>,
    pub self_arg: Option<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Boolean(Rc<Boolean>),
    BinaryOp(Rc<BinaryExpression>),
    UnaryOp(Rc<UnaryExpression>),
    String(Rc<Str>),
    Number(Rc<Number>),
    Nil(Rc<Nil>),
    IndexOp(Rc<IndexOp>),
    Call(Rc<CallExpression>),
    Function(Rc<FunctionExpression>),
    GlobalSymbol(Rc<GlobalSymbol>),
    Identifier(Rc<Identifier>),
    Table(Rc<TableExpression>),
    Shadow(Rc<ShadowExpression>),
}

impl Expression {
    pub fn to_statement(self) -> Statement {
        Statement::StatExpr(Box::new(StatExpr { value: self }))
    }
}

impl Into<Statement> for Expression {
    fn into(self) -> Statement {
        self.to_statement()
    }
}