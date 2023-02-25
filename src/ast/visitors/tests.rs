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

#![cfg(test)]

use crate::ast::{expression::*, tree::*, visitors::*};
use std::rc::Rc;

#[test]
fn ast_to_lua() {
    let expression = CallExpression {
        is_self: false,
        function: Expression::GlobalSymbol(Rc::new(GlobalSymbol("print".to_string()))),
        arguments: vec![Expression::String(Rc::new(Str("it worked!".to_string())))],
    };

    let mut reconstructor = source_reconstructor::SourceReconstructor::default();
    reconstructor.visit_call(&expression);

    assert_eq!(dbg!(reconstructor.source()), r#"print("it worked!")"#);

    let mut reconstructor = source_reconstructor::SourceReconstructor::default();
    let wrapped_expression = Expression::Call(Rc::new(expression));

    reconstructor.visit_expr(&wrapped_expression);
    assert_eq!(dbg!(reconstructor.source()), r#"print("it worked!")"#);
}
