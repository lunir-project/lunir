#![cfg(test)]

use crate::ast::{expression::*, tree::*, visitors::*};
use std::rc::Rc;

#[test]
fn ast_to_lua() {
    let mut expression = CallExpression {
        is_self: false,
        function: Expression::GlobalSymbol(Rc::new(GlobalSymbol("print".to_string()))),
        arguments: vec![Expression::String(Rc::new(Str("it worked!".to_string())))],
    };

    let mut reconstructor = source_reconstructor::SourceReconstructor::default();
    reconstructor.visit_mut(&mut expression);

    assert_eq!(dbg!(reconstructor.source()), r#"print("it worked!")"#);

    let mut reconstructor = source_reconstructor::SourceReconstructor::default();
    let mut wrapped_expression = Expression::Call(Rc::new(expression));

    reconstructor.visit_mut(&mut wrapped_expression);
    assert_eq!(dbg!(reconstructor.source()), r#"print("it worked!")"#);
}
