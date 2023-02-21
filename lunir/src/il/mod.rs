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

use serde::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
enum Table {
    Map(HashMap<Value, Value>),
    Array(Vec<Value>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
enum Value {
    Nil,
    Boolean(bool),
    ConstantIndex(usize),
    Immediate(i32),
    StackIndex(usize),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum Constant {
    Nil,
    Boolean(bool),
    Function(Function),
    Number(f64),
    String(String),
    Table(Table),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Load {
    dest: usize,
    src: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GetGlobal {
    dest: usize,
    constant: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GetTable {
    dest: usize,
    source: usize,
    index: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum BinaryOpKind {
    Add,
    Concat,
    Div,
    Mod,
    Mul,
    Pow,
    Sub,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BinaryOp {
    kind: BinaryOpKind,
    dest: usize,

    left: Value,
    right: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum UnaryOpKind {
    Len,
    Not,
    Neg,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UnaryOp {
    kind: UnaryOpKind,
    dest: usize,

    value: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum ConditionalKind {
    Eq,
    Ge,
    Gt,
    Ne,
    Lt,
    Le,

    And,
    Or,
}

// Not gonna be an instruction, its own thing
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Conditional {
    kind: ConditionalKind,
    dest: usize,

    left: Value,
    right: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct JumpBranch {
    start: usize,
    end: usize,
    offset: isize,
}

// Unconditional jump
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Jump {
    branch: JumpBranch,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ConditionalJump {
    branch: JumpBranch,
    condition: Conditional,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct NewTable {
    dest: usize,

    table_size: usize,
    array_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Call {
    callee: usize,

    // 0 means variable
    num_args: usize,
    num_returns: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Return {
    from: usize,
    count: usize,
}

#[repr(u8)]
#[derive(Debug, Clone, Serialize, Deserialize)]
enum VarargTag {
    HasArg = 1,
    IsVararg = 2,
    NeedsArg = 4,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Function {
    constants: Vec<Constant>,
    code: Vec<u8>,
    is_variadic: VarargTag,
    lineinfo: Vec<u32>,
    name: Option<String>,
    num_upvalues: u8,
    num_parameters: u8,
    max_stack_size: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum Instruction {
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
