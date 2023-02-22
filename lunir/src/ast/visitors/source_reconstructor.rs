use crate::ast::{expression::*, statement::*, tree::*};

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
        self.source.push_str(&node.0)
    }
}

impl VisitorMut<Identifier> for SourceReconstructor {
    fn visit_mut(&mut self, node: &Identifier) {
        self.source.push_str(&node.0)
    }
}

impl VisitorMut<Str> for SourceReconstructor {
    fn visit_mut(&mut self, node: &Str) {
        self.source.push_str(&format!(r#""{}""#, node.0).as_str())
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
