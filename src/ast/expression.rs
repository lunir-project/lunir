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

use std::collections::BTreeMap;
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

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Nil;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Number(pub f64);
impl Eq for Number {
    fn assert_receiver_is_total_eq(&self) {}
}

impl Ord for Number {
    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized,
    {
        use std::cmp::Ordering;

        match self.partial_cmp(&min) {
            Some(Ordering::Greater) => match self.partial_cmp(&max) {
                Some(Ordering::Less) => self,
                _ => max,
            },
            _ => min,
        }
    }

    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }

    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        std::cmp::max(self, other)
    }

    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        std::cmp::min(self, other)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Boolean(pub bool);

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Str(pub String);

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct GlobalSymbol(pub String);

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Identifier(pub String);

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
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

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct BinaryExpression {
    kind: BinaryExpressionKind,

    left: Expression,
    right: Expression,
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

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct UnaryExpression {
    kind: UnaryOpKind,
    value: Expression,
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

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum TableExpression {
    HashMap(BTreeMap<Expression, Expression>),
    Array(Vec<Expression>),
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct IndexOp {
    key: Expression,
    table: Expression,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct CallExpression {
    pub arguments: Vec<Expression>,
    pub function: Expression,
    
    pub is_self: bool,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct FunctionExpression {
    body: Box<StatBlock>,
    has_vararg: bool,

    parameters: Vec<Expression>,
    self_arg: Option<Expression>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
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
}
