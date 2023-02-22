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

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Table {
    Map(HashMap<Value, Value>),
    Array(Vec<Value>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Value {
    Nil,
    Boolean(bool),
    ConstantIndex(usize),
    Immediate(i32),
    StackIndex(usize),
}

#[derive(Debug, Clone)]
pub enum Constant {
    Nil,
    Boolean(bool),
    Function(Function),
    Number(f64),
    String(String),
    Table(Table),
}

#[derive(Debug, Clone)]
pub struct Load {
    pub dest: usize,
    pub src: Value,
}

#[derive(Debug, Clone)]
pub struct GetGlobal {
    pub dest: usize,
    pub constant: usize,
}

#[derive(Debug, Clone)]
pub struct GetTable {
    pub dest: usize,
    pub source: usize,
    pub index: Value,
}

#[derive(Debug, Clone)]
pub enum BinaryOpKind {
    Add,
    Concat,
    Div,
    Mod,
    Mul,
    Pow,
    Sub,
}

#[derive(Debug, Clone)]
pub struct BinaryOp {
    pub ind: BinaryOpKind,
    pub dest: usize,

    pub left: Value,
    pub right: Value,
}

#[derive(Debug, Clone)]
pub enum UnaryOpKind {
    Len,
    Not,
    Neg,
}

#[derive(Debug, Clone)]
pub struct UnaryOp {
    pub kind: UnaryOpKind,
    pub dest: usize,

    pub value: Value,
}

#[derive(Debug, Clone)]
pub enum ConditionalKind {
    Eq,
    Ge,
    Gt,
    Ne,
    Lt,
    Le,

    And,
    Or,
}

#[derive(Debug, Clone)]
pub struct Conditional {
    pub kind: ConditionalKind,
    pub dest: usize,

    pub left: Value,
    pub right: Value,
}

#[derive(Debug, Clone)]
pub struct JumpBranch {
    pub start: usize,
    pub end: usize,
    pub offset: isize,
}

// Unconditional jump
#[derive(Debug, Clone)]
pub struct Jump {
    pub branch: JumpBranch,
}

#[derive(Debug, Clone)]
pub struct ConditionalJump {
    pub branch: JumpBranch,
    pub condition: Conditional,
}

#[derive(Debug, Clone)]
pub struct NewTable {
    pub dest: usize,

    pub table_size: usize,
    pub array_size: usize,
}

#[derive(Debug, Clone)]
pub enum OptVariable {
    Variable,
    Number(usize),
}

#[derive(Debug, Clone)]
pub struct Call {
    pub callee: usize,

    pub num_args: OptVariable,
    pub num_returns: OptVariable,
}

#[derive(Debug, Clone)]
pub struct Return {
    pub from: usize,
    pub count: usize,
}

#[repr(u8)]
#[derive(Debug, Clone)]
pub enum VarargTag {
    HasArg = 1,
    IsVararg = 2,
    NeedsArg = 4,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub constants: Vec<Constant>,
    pub code: Vec<u8>,
    pub is_variadic: VarargTag,
    pub lineinfo: Vec<u32>,
    pub name: Option<String>,
    pub num_upvalues: u8,
    pub num_parameters: u8,
    pub max_stack_size: u8,
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Load(Box<Load>),

    GetGlobal(Box<GetGlobal>),
    GetTable(Box<GetTable>),

    BinaryOp(Box<BinaryOp>),
    UnaryOp(Box<UnaryOp>),

    Jump(Box<Jump>),
    ConditionalJump(Box<ConditionalJump>),

    NewTable(Box<NewTable>),
    Return(Box<Return>),

    Call(Box<Call>),
}
