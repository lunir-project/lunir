#![cfg(test)]

use crate::prelude::{
    BinaryOp, BinaryOpKind, Condition, ConditionKind, ConditionalJump, Instruction, Jump,
    JumpBranch, Load, Return, Value, SetGlobal
};

use super::cir::*;

#[test]
fn numeric_while_loop() {
    let code = vec![
        Instruction::Load(Box::new(Load {
            dest: 0,
            src: Value::StackIndex(0),
        })),
        Instruction::Jump(Box::new(Jump {
            branch: JumpBranch {
                start: 1,
                end: 3,
                offset: 2,
            },
        })),
        Instruction::BinaryOp(Box::new(BinaryOp {
            operator: BinaryOpKind::Add,
            dest: 0,
            left: Value::ConstantIndex(0),
            right: Value::ConstantIndex(0),
        })),
        Instruction::ConditionalJump(Box::new(ConditionalJump {
            branch: JumpBranch {
                start: 3,
                end: 2,
                offset: -1,
            },
            condition: Condition {
                kind: ConditionKind::Lt,
                left: Value::ConstantIndex(0),
                right: Value::ConstantIndex(251),
            },
        })),
        Instruction::Return(Box::new(Return {
            result_count: 0,
            result_start: 0,
        })),
    ];
    into_cir_graph(code)
}

#[test]
fn some_other_code() {
    // let code2 = vec![
    //     Instruction::Load(Box::new(Load { 
    //         dest: 0, 
    //         src: Value::StackIndex(0) 
    //     })),
    //     Instruction::SetGlobal(Box::new(SetGlobal { 
    //         src: -1,
    //         constant: 0
    //     },

    // ))
    // ];
}
