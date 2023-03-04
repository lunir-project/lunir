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

/// Represents the two states of a table, array (index-value pairs) and hashmap (key-value pairs).
#[derive(Debug, Clone)]
pub enum Table {
    Map(HashMap<Value, Value>),
    Array(Vec<Value>),
}

/// Represents the types of operands that can be used within an IL instruction.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Value {
    Nil,
    Boolean(bool),
    ConstantIndex(usize),
    Immediate(i32),
    StackIndex(usize),
}

/// Represents the types of values that can be present in the constant table.
#[derive(Debug, Clone)]
pub enum Constant {
    Nil,
    Boolean(bool),
    Function(Function),
    Number(f64),
    String(String),
    Table(Table),
}

/// A load operation with a destination index and a source value.
#[derive(Debug, Clone)]
pub struct Load {
    pub dest: usize,
    pub src: Value,
}

/// A get global operation with a destination index and a source constant table index.
#[derive(Debug, Clone)]
pub struct GetGlobal {
    pub dest: usize,
    pub constant: usize,
}

/// Perfoms a table index operation on the value at stack index `source` using the value at stack index `key`, then stores the result in stack index `dest`.
#[derive(Debug, Clone)]
pub struct GetTable {
    pub dest: usize,
    pub source: usize,
    pub key: Value,
}

/// Represents the kinds of supported binary operations.
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

/// A binary operation with an operator, a destination index, and left and right operands.
#[derive(Debug, Clone)]
pub struct BinaryOp {
    pub operator: BinaryOpKind,
    pub dest: usize,

    pub left: Value,
    pub right: Value,
}

/// Represents the kinds of supported unary operations.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum UnaryOpKind {
    Len,
    Not,
    Neg,
}

/// A unary operation with an operator, a destination index, and a single operand.
#[derive(Debug, Clone)]
pub struct UnaryOp {
    pub operator: UnaryOpKind,
    pub dest: usize,

    pub left: Value,
}

/// Represents the kinds of supported conditions.
#[derive(Debug, Clone)]
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

// A condition with a kind, a destination index, and left and right operands.
#[derive(Debug, Clone)]
pub struct Condition {
    pub kind: ConditionKind,
    pub dest: usize,

    pub left: Value,
    pub right: Value,
}

/// List of all of the compiler built-ins
#[derive(Debug, Clone)]
pub enum IntrinsicKind {
    BitAnd,
    BitNot,
    BitOr,
    BitTest,
    BitXor,

    LeftShift,
    LeftRotate,

    RightShift,
    RightRotate,

    ArithShift,

    Extract,
}

// Instruction to declare intrinsic.
#[derive(Debug, Clone)]
pub struct Intrinsic {
    pub kind: IntrinsicKind,
}

/// Information about a program-counter-relative-jump, `start` is the program counter, `end` is the offset added to the base index, and `offset` is the total number of instructions skipped for the branch.
#[derive(Debug, Clone)]
pub struct JumpBranch {
    pub start: usize,
    pub end: usize,
    pub offset: isize,
}

/// An unconditional jump.
#[derive(Debug, Clone)]
pub struct Jump {
    pub branch: JumpBranch,
}

/// A conditional jump that only jumps if the `NOT` of the value at stack index `cond` evaluates to true.
#[derive(Debug, Clone)]
pub struct JumpNot {
    pub branch: JumpBranch,
    pub cond: usize,
}

/// A jump with an attached condition.
#[derive(Debug, Clone)]
pub struct ConditionalJump {
    pub branch: JumpBranch,
    pub condition: Condition,
}

/// Creates a new table at stack index `dest` with an initial size of `table_size` and `array_size` array elements.
#[derive(Debug, Clone)]
pub struct NewTable {
    pub dest: usize,

    pub table_size: usize,
    pub array_size: usize,
}

/// Represents either a number or a variable number of values that approach the top of the stack.
#[derive(Debug, Clone)]
pub enum OptVariable {
    Variable,
    Number(usize),
}

/// Calls the function at stack index `callee` with `num_args` ahead of it on the stack, then returns a `num_returns` number of results.
#[derive(Debug, Clone)]
pub struct Call {
    pub callee: usize,

    pub self_call: bool,
    pub num_args: OptVariable,
    pub num_returns: OptVariable,
}

/// Performs a return from the current chunk, passing all values fr om stack index `result_start` up to `result_start + result_count` to the caller.
#[derive(Debug, Clone)]
pub struct Return {
    pub result_start: usize,
    pub result_count: usize,
}

/// Describes the arity of a function.
#[derive(Debug, Clone)]
pub enum Vararg {
    HasArg,
    IsVararg,
    NeedsArg,
}

/// A function in LUNIR intermediate language.
#[derive(Debug, Clone)]
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

/// All possible LUNIR intermediate language instructions.
#[derive(Debug, Clone)]
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
}

/// A chunk of code in LUNIR's intermediate language.
#[derive(Clone, Debug)]
pub struct IlChunk(Vec<Instruction>);
