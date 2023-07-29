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

use lifering::FloatingPointComponents;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HirNode {
    children: Vec<HirNode>,
    kind: HirNodeKind,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HirFunctionSignature {
    parameters: Vec<HirTy>,
    return_type: HirTy,
}
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HirTy {
    data: (),
}

pub trait ConstantEvaluable {
    fn is_constant(&self) -> bool;
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub enum HirExpressionKind {
    Nil,
    Number(FloatingPointComponents<f64>),
    LuaString(String),
    Boolean(bool),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HirExpression {
    data: (),
    ty: HirTy,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HirStatementKind {
    IfStatement,
    Loop,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HirNodeKind {
    Expression(HirExpression),
    Statement(HirStatementKind),
}
