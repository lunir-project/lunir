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

pub mod expression;
pub mod statement;
pub mod tree;

pub mod visitors;

#[cfg(test)]
mod tests {
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
}
