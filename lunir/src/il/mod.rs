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
    // Numbers like 0 and 1 refer to the positional parameters in the enum.

    // Stack[0] = Value
    Load(usize, Value),
    
}