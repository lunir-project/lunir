#![cfg(test)]
use super::expression::{BinaryExpressionKind, IdentifierString};

#[test]
fn sanitization() {
    assert_eq!(
        String::from("test\n\n\n\x12").sanitize_special(),
        "test\\n\\n\\n\\18"
    );
    assert_eq!(
        String::from("hello-world I'm  0x90__!").sanitize_identifier(),
        "hello_world_Im_0x90_"
    );
}

#[test]
fn compound_conversion() {
    assert_eq!(
        BinaryExpressionKind::Add.to_compound(),
        BinaryExpressionKind::AddAssign
    );
    assert_eq!(
        BinaryExpressionKind::DivAssign.to_compound(),
        BinaryExpressionKind::DivAssign
    );
}
