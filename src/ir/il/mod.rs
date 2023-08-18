//    Copyright 2023 lunir-project

//    Licensed under the Apache License, Version 2.0 (the "License");
//    you may not use this file except in compliance with the License.
//    You may obtain a copy of the License at

//        http://www.apache.org/licenses/LICENSE-2.0

//    Unless required by applicable law or agreed to in writing, software
//    distributed under the License is distributed on an "AS IS" BASIS,
//    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//    See the License for the specific language governing permissions and
//    limitations under the License.

// TODO: remove once everything is used
#![allow(unused)]

use std::{collections::HashMap, fmt::Debug};

use itertools::Itertools;

/// Represents the two states of a table, array (index-value pairs) and hashmap
/// (key-value pairs).
#[derive(Clone)]
pub enum Table {
    // Maps must not use HashMap.
    // This is because in Lua, you can have the same key multiple times, but with a different value.
    // Using a Vec prohibits the hashmap from overwriting data, allowing you to see all refs to the keys
    // and their values.
    Map(Vec<(Value, Value)>),
    Array(Vec<Value>),
}

impl Debug for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = String::with_capacity(32);

        buf.push('{');
        match self {
            Self::Map(hash_map) => {
                buf.push_str(
                    &hash_map
                        .iter()
                        .map(|(k, v)| format!("[{k:?}] {_eq:>4} {v:?}", _eq = "="))
                        .join(", "),
                );
            }
            Self::Array(array) => {
                buf.push_str(&array.iter().map(|v| format!("{v:?}")).join(", "));
            }
        }
        buf.push('}');
        write!(f, "{buf}")
    }
}

/// Represents the types of operands that can be used within an IL instruction.
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Value {
    Nil,
    Boolean(bool),
    ConstantIndex(usize),
    Immediate(i32),
    StackIndex(usize),
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Nil => "nil".into(),
                Self::Boolean(b) => format!("{b}"),
                Self::ConstantIndex(i) | Value::StackIndex(i) => format!("{i}"),
                Self::Immediate(v) => format!("{v}"),
            }
        )
    }
}

/// Represents the types of values that can be present in the constant table.
#[derive(Clone)]
pub enum Constant {
    Nil,
    Boolean(bool),
    Function(Function),
    Number(f64),
    String(String),
    Table(Table),
}
// L
impl Debug for Constant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Nil => "nil".into(),
                Self::Boolean(b) => format!("{b}"),
                Self::Function(s) => format!("{s:?}"),
                Self::Number(n) => format!("{n}"),
                Self::String(s) => s.into(),
                Self::Table(t) => format!("{t:?}"),
            }
        )
    }
}

/// A load operation with a destination index and a source value.
#[derive(PartialEq, Clone)]
pub struct Load {
    pub dest: usize,
    pub src: Value,
}

impl Debug for Load {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {_eq:>4} {:?}", self.dest, self.src, _eq = "=")
    }
}

/// A get global operation with a destination index and a source constant table index.
#[derive(PartialEq, Clone)]
pub struct GetGlobal {
    pub dest: usize,
    pub constant: usize,
}

impl Debug for GetGlobal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {_eq:>4} {}", self.dest, self.constant, _eq = "=")
    }
}

#[derive(PartialEq, Clone)]
pub struct SetGlobal {
    pub src: usize,
    pub constant: usize,
}

impl Debug for SetGlobal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {_eq:>4} {}", self.constant, self.src, _eq = "=")
    }
}

/// Perfoms a table index operation on the value at stack index `source` using the value
/// at stack index `key`, then stores the result in stack index `dest`.
#[derive(PartialEq, Clone)]
pub struct GetTable {
    pub dest: usize,
    pub source: usize,
    pub key: Value,
}

impl Debug for GetTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {_eq:>4} {}[{:?}]",
            self.dest,
            self.source,
            self.key,
            _eq = "="
        )
    }
}

/// Represents the kinds of supported binary operations.
#[derive(PartialEq, Clone)]
pub enum BinaryOpKind {
    Add,
    Concat,
    Div,
    Mod,
    Mul,
    Pow,
    Sub,
}

impl Debug for BinaryOpKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                BinaryOpKind::Add => "+",
                BinaryOpKind::Sub => "-",
                BinaryOpKind::Mul => "*",
                BinaryOpKind::Div => "/",
                BinaryOpKind::Mod => "%",
                BinaryOpKind::Pow => "^",
                BinaryOpKind::Concat => "..",
            }
        )
    }
}

/// A binary operation with an operator, a destination index, and left and right operands.
#[derive(PartialEq, Clone)]
pub struct BinaryOp {
    pub operator: BinaryOpKind,
    pub dest: usize,

    pub left: Value,
    pub right: Value,
}

impl Debug for BinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {_eq:>4} {:?} {:?} {:?}",
            self.dest,
            self.left,
            self.operator,
            self.right,
            _eq = "="
        )
    }
}

/// Represents the kinds of supported unary operations.
#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum UnaryOpKind {
    Len,
    Not,
    Neg,
}

impl Debug for UnaryOpKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                UnaryOpKind::Len => '#',
                UnaryOpKind::Not => '!',
                UnaryOpKind::Neg => '-',
            }
        )
    }
}

/// A unary operation with an operator, a destination index, and a single operand.
#[derive(PartialEq, Clone)]
pub struct UnaryOp {
    pub operator: UnaryOpKind,
    pub dest: usize,

    pub left: Value,
}

impl Debug for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {_eq:>4} {:?}{:?}",
            self.dest,
            self.operator,
            self.left,
            _eq = "="
        )
    }
}

/// Represents the kinds of supported conditions.
#[derive(PartialEq, Clone)]
pub enum ConditionKind {
    Eq,
    Ge,
    Gt,
    Ne,
    Lt,
    Le,

    And,
    Or,
}

impl Debug for ConditionKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                ConditionKind::Eq => "eq",
                ConditionKind::Ge => "ge",
                ConditionKind::Gt => "gt",
                ConditionKind::Ne => "ne",
                ConditionKind::Lt => "lt",
                ConditionKind::Le => "le",
                ConditionKind::And => "and",
                ConditionKind::Or => "or",
            }
        )
    }
}

// A condition with a kind, a destination index, and left and right operands.
#[derive(PartialEq, Clone)]
pub struct Condition {
    pub kind: ConditionKind,

    pub left: Value,
    pub right: Value,
}

impl Debug for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {:?} {:?}", self.left, self.kind, self.right)
    }
}

/// All possible intrinsic operations
#[derive(PartialEq, Clone)]
pub enum IntrinsicKind {
    BitAnd(Value, Value),
    BitOr(Value, Value),
    BitXor(Value, Value),
    BitNot(Value),

    LeftShift(Value, Value),
    RightShift(Value, Value),
}

impl IntrinsicKind {
    fn is_unary(&self) -> bool {
        match self {
            Self::BitNot(_) => true,
            _ => false,
        }
    }

    fn is_binary(&self) -> bool {
        !self.is_unary()
    }
}

impl Debug for IntrinsicKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            if self.is_unary() {
                if let Self::BitNot(op) = self {
                    format!("!{op:?}")
                } else {
                    unreachable!()
                }
            } else {
                let (op, lhs, rhs) = match self {
                    Self::BitAnd(lhs, rhs) => ("&", lhs, rhs),
                    Self::BitOr(lhs, rhs) => ("|", lhs, rhs),
                    Self::BitXor(lhs, rhs) => ("^", lhs, rhs),
                    Self::LeftShift(lhs, rhs) => ("<<", lhs, rhs),
                    Self::RightShift(lhs, rhs) => (">>", lhs, rhs),
                    _ => unreachable!(),
                };

                format!("{lhs:?} {op} {rhs:?}")
            }
        )
    }
}

// Instruction to declare intrinsic.
#[derive(PartialEq, Clone)]
pub struct Intrinsic {
    pub kind: IntrinsicKind,
    pub dest: usize,
}

impl Debug for Intrinsic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {_eq:>4} {:?}", self.dest, self.kind, _eq = "=")
    }
}

/// Information about a program-counter-relative-jump, `start` is the program counter,
/// `end` is the offset added to the counter, and `offset` is the total number of
/// instructions skipped for the branch.
#[derive(PartialEq, Debug, Clone)]
pub struct JumpBranch {
    pub start: usize,
    pub end: usize,
    pub offset: isize,
}

/// An unconditional jump.
#[derive(PartialEq, Debug, Clone)]
pub struct Jump {
    pub branch: JumpBranch,
}

/// A conditional jump that only jumps if the `NOT` of the value at stack index `cond`
/// evaluates to true.
#[derive(PartialEq, Debug, Clone)]
pub struct JumpNot {
    pub branch: JumpBranch,
    pub cond: usize,
}

/// A jump with an attached condition.
#[derive(PartialEq, Clone)]
pub struct ConditionalJump {
    pub branch: JumpBranch,
    pub condition: Condition,
}

impl Debug for ConditionalJump {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "jump{:?} {} {_com:>12} {}",
            self.condition,
            self.branch.end,
            self.branch.offset,
            _com = ";"
        )
    }
}

/// Creates a new table at stack index `dest` with an initial size of `table_size` and
/// `array_size` array elements.
#[derive(PartialEq, Clone)]
pub struct NewTable {
    pub dest: usize,

    pub array_size: usize,
    pub table_size: usize,
}

impl Debug for NewTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {_eq:>4} newtable {arraysize} {tablesize} {_com:>12} {arraysize}/{tablesize}",
            self.dest,
            arraysize = self.array_size,
            tablesize = self.table_size,
            _eq = "=",
            _com = ";"
        )
    }
}

/// Represents either a number or a variable number of values that approach the top of the
///  stack.
#[derive(PartialEq, Clone)]
pub enum OptVariable {
    Variable,
    Number(usize),
}

impl Debug for OptVariable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Variable => write!(f, "vararg"),
            Self::Number(n) => write!(f, "{n}..top"),
        }
    }
}

/// Calls the function at stack index `callee` with `num_args` ahead of it on the stack,
/// then returns a `num_returns` number of results.
#[derive(PartialEq, Clone)]
pub struct Call {
    pub callee: usize,

    pub self_call: bool,
    pub num_args: OptVariable,
    pub num_returns: OptVariable,
}

impl Debug for Call {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} {_eq:>4} {}({}{:?})",
            self.num_returns,
            self.callee,
            if self.self_call { "self, " } else { "" },
            self.num_args,
            _eq = "="
        )
    }
}

/// Performs a return from the current chunk, passing all values fr om stack index
/// `result_start` up to `result_start + result_count` to the caller.
#[derive(PartialEq, Clone)]
pub struct Return {
    pub result_start: usize,
    pub result_count: usize,
}

impl Debug for Return {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "return {}..{}", self.result_start, self.result_count)
    }
}

/// Describes the arity of a function.
#[derive(Clone)]
pub enum Vararg {
    HasArg,
    IsVararg,
    NeedsArg,
}

impl Debug for Vararg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::HasArg => "[[vararg::has_arg]]",
                Self::IsVararg => "[[vararg::is_vararg]]",
                Self::NeedsArg => "[[vararg::needs_arg]]",
            }
        )
    }
}

/// A function in LUNIR intermediate language.
#[derive(Clone)]
pub struct Function {
    pub constants: Vec<Constant>,
    pub code: Vec<u8>,
    pub is_variadic: Vararg,
    pub lineinfo: Vec<u32>,
    pub name: Option<String>,
    pub upvalue_count: u8,
    pub param_count: u8,
    pub max_stack_size: u8,
}

impl Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} fn {:?}", self.is_variadic, self.name)
    }
}

/// All possible LUNIR intermediate language instructions.
#[derive(PartialEq, Debug, Clone)]
pub enum Instruction {
    Load(Box<Load>),

    Intrinsic(Box<Intrinsic>),

    GetGlobal(Box<GetGlobal>),
    GetTable(Box<GetTable>),

    BinaryOp(Box<BinaryOp>),
    UnaryOp(Box<UnaryOp>),

    Jump(Box<Jump>),
    JumpNot(Box<JumpNot>),
    ConditionalJump(Box<ConditionalJump>),

    NewTable(Box<NewTable>),
    Return(Box<Return>),

    Call(Box<Call>),
    SetGlobal(Box<SetGlobal>),
}

/// A chunk of code in LUNIR's intermediate language.
#[derive(PartialEq, Clone)]
pub struct IlChunk(Vec<Instruction>);

impl IlChunk {
    pub fn inner(&self) -> &Vec<Instruction> {
        &self.0
    }

    pub fn new(inner: Vec<Instruction>) -> Self {
        Self(inner)
    }
}

impl IlChunk {
    pub(crate) fn inner_mut(&mut self) -> &mut Vec<Instruction> {
        &mut self.0
    }
}

impl Debug for IlChunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0.iter().map(|item| format!("{item:?}\n")).join("\n")
        )
    }
}

impl From<&[Instruction]> for IlChunk {
    fn from(slice: &[Instruction]) -> Self {
        IlChunk(slice.to_vec())
    }
}
