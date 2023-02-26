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

use crate::ast::{expression::*, statement::*, tree::*};
use derive_builder::Builder;

#[derive(Default)]
pub struct SourceReconstructor {
    indents: usize,
    source: String,
    settings: SourceReconstructorSettings,
}

#[derive(Debug, Builder, Clone)]
#[builder(default)]
pub struct SourceReconstructorSettings {
    /// Appends semicolons to the end of statements. Redundant if use_newline is false
    use_semicolons: bool,

    /// Appends a newline character (\n) to the end of statements. If false, all of the code will be on the same line.
    use_newline: bool,

    /// Uses tabs instead of spaces for indentation
    use_tabs: bool,

    /// This field is invalidated if the `use_tabs` field is set
    space_count: usize,

    /// This string will be added to the top of the file. If this field is `None`, then nothing will
    custom_header: Option<String>,
}

impl Default for SourceReconstructorSettings {
    fn default() -> Self {
        Self {
            space_count: 4,
            use_semicolons: true,
            use_newline: true,
            use_tabs: false,
            custom_header: Some(
                "Decompiled with LUNIR (https://github.com/lunir-project/lunir)".to_string(),
            ),
        }
    }
}

impl SourceReconstructor {
    fn indent(&mut self) {
        assert!(self.indents > 0);
        self.source.push_str(
            if self.settings.use_tabs {
                "\t".repeat(self.indents)
            } else {
                " ".repeat(4 * self.settings.space_count)
            }
            .as_str(),
        );
    }

    pub fn source(self) -> String {
        match self.settings.custom_header {
            Some(header) => format!("// {}\n{}", header, self.source),
            None => self.source,
        }
    }

    pub fn with_settings(settings: SourceReconstructorSettings) -> Self {
        Self {
            settings,
            ..Default::default()
        }
    }
}

impl Visitor<'_> for SourceReconstructor {
    fn visit_bool(&mut self, node: &'_ Boolean) {
        self.source.push_str(format!("{}", node.0).as_str());
    }

    fn visit_string(&mut self, node: &'_ Str) {
        self.source
            .push_str(format!(r#""{}""#, node.0.sanitize_special()).as_str());
    }

    fn visit_number(&mut self, node: &'_ Number) {
        self.source.push_str(node.0.to_string().as_str());
    }

    fn visit_nil(&mut self, _: &'_ Nil) {
        self.source.push_str("nil");
    }

    fn visit_identifier(&mut self, node: &'_ Identifier) {
        self.source.push_str(node.0.as_str());
    }

    fn visit_global_symbol(&mut self, node: &'_ GlobalSymbol) {
        self.source.push_str(node.0.as_str());
    }

    fn visit_return(&mut self, node: &'_ StatReturn) {
        self.source.push_str("return ");

        let last = node.results.len() - 1;
        for (i, value) in node.results.iter().enumerate() {
            self.visit_expr(value);

            if i < last {
                self.source.push_str(", ");
            }
        }
    }

    fn visit_call(&mut self, node: &'_ CallExpression) {
        self.visit_expr(&node.function);

        self.source.push('(');

        let last = node.arguments.len() - 1;
        for (i, value) in node.arguments.iter().enumerate() {
            self.visit_expr(value);

            if i < last {
                self.source.push_str(", ");
            }
        }

        self.source.push(')');
    }

    fn visit_unary(&mut self, node: &'_ UnaryExpression) {
        self.source.push_str(node.kind.to_string().as_str());
        self.visit_expr(&node.value);
    }

    fn visit_stat(&mut self, node: &'_ Statement) {
        walk_statement(self, node);

        if self.settings.use_newline {
            if self.settings.use_semicolons {
                self.source.push(';');
            }
            self.source.push('\n');
        } else {
            self.source.push(';');
        }
    }

    fn visit_binary(&mut self, node: &'_ BinaryExpression) {
        self.visit_expr(&node.left);

        self.source.push_str(format!(" {} ", node.kind).as_str());

        self.visit_expr(&node.right);
    }

    fn visit_index_op(&mut self, node: &'_ IndexOp) {
        self.visit_expr(&node.table);

        match &node.key {
            Expression::String(s) => {
                if s.0.can_identify() {
                    self.source.push('.');
                    self.source.push_str(s.0.as_str());
                } else {
                    self.source.push_str(r#"[""#);
                    self.source.push_str(s.0.sanitize_special().as_str());
                    self.source.push_str(r#""]"#);
                }
            }

            _ => {
                self.source.push('[');
                self.visit_expr(&node.key);
                self.source.push(']');
            }
        }
    }
}
