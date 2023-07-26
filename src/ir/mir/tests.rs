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

#![cfg(test)]
use crate::ir::il::{
    BinaryOp, BinaryOpKind, Condition, ConditionKind, ConditionalJump, Instruction, Jump,
    JumpBranch, Load, Return, Value,
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
