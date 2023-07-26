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
pub enum HirTyKind {
    Bool,
    Number,
    String,
    Userdata,
    Table,
    Tuple(Vec<HirTy>),
    Union(Vec<HirTy>),
    Function(Box<HirFunctionSignature>),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HirFunctionSignature {
    parameters: Vec<HirTy>,
    return_type: HirTy,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HirMetatable {
    add: Option<HirNode>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HirTy {
    ty: Option<HirTyKind>,

}

// for when ur back
// I'm considering this because it can be applied on all layers of abstraction that we have here
// and we can use it to constrain generics for simple functions that require a fold 
// we can extend the trait with a `fold_constant` function that takes a SymbolicExecution context 
pub trait ConstantEvaluable {
    fn is_constant(&self) -> bool;
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HirExpressionKind {
    Nil,
    // TODO: lifering has no PartialOrd or Ord! Implement a wrapper type? Or update lifering?
    //Number(FloatingPointComponents),
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
    Loop
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HirNodeKind {
    Expression(HirExpression),
    Statement(HirStatementKind),
}
