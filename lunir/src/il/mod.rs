use serde::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
enum Value {
    StackIndex(usize),
    ConstantIndex(usize),
    Boolean(bool),
    Immediate(i32),
    Nil,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum Instruction {
    // P(n) refers to the positional parameters in the enum.

    // Stack[P(0)] = P(1)
    Load(usize, Value),

}