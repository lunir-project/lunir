use lazy_static::__Deref;

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

impl VisitorMut<Str> for SourceReconstructor {
    fn visit_mut(&mut self, node: &mut Str) {
        self.source.push_str(&format!(r#""{}""#, node.0).as_str())
    }
}

impl VisitorMut<CallExpression> for SourceReconstructor {
    fn visit_mut(&mut self, node: &mut CallExpression) {
        node.function.accept_mut(self);

        self.source.push('(');

        let last = node.arguments.len();
        for (i, arg) in node.arguments.iter_mut().enumerate() {
            arg.accept_mut(self);

            if i < last {
                self.source.push(',');
            }
        }

        self.source.push(')');
    }
}

impl VisitorMut<Expression> for SourceReconstructor {
    fn visit_mut(&mut self, node: &mut Expression) {
        node.accept_mut(self);
    }
}
