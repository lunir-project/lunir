
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

use crate::ast::{expression::*, tree::*};

#[derive(Default)]
pub struct SourceReconstructor {
    indents: usize,
    source: String,
}

impl SourceReconstructor {
    fn indent(&mut self) {
        assert!(self.indents > 0);
        self.source
            .push_str(&format!("{:1$}", "", self.indents - 1));
    }

    pub fn source(&self) -> &String {
        &self.source
    }
}

impl VisitorMut<GlobalSymbol> for SourceReconstructor {
    fn visit_mut(&mut self, node: &GlobalSymbol) {
        self.source.push_str(&node.0);
    }
}

impl VisitorMut<Identifier> for SourceReconstructor {
    fn visit_mut(&mut self, node: &Identifier) {
        self.source.push_str(&node.0);
    }
}

impl VisitorMut<Str> for SourceReconstructor {
    fn visit_mut(&mut self, node: &Str) {
        self.source.push_str(format!(r#""{}""#, node.0).as_str());
    }
}

impl VisitorMut<CallExpression> for SourceReconstructor {
    fn visit_mut(&mut self, node: &CallExpression) {
        node.function.accept_mut(self);

        self.source.push('(');

        let last = node.arguments.len();
        for (i, arg) in node.arguments.iter().enumerate() {
            arg.accept_mut(self);

            if i < last - 1 {
                self.source.push(',');
            }
        }

        self.source.push(')');
    }
}

impl VisitorMut<Expression> for SourceReconstructor {
    fn visit_mut(&mut self, node: &Expression) {
        macro_rules! visit {
            ($node: expr, $($t: ident,)* ) => {
                match $node {
                    $(
                        Expression::$t(x) => x.accept_mut(self),
                    )*

                    _ => unimplemented!(),
                }
            };

            ($node: expr, $($t: ident),*) => {
                visit!($node, $($t,)*)
            };
        }

        visit!(node, String, Call, GlobalSymbol, Identifier);
    }
}
