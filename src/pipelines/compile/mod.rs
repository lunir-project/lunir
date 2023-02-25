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

use crate::{ast::tree::Node, il::IlChunk};

use super::OptimizationLevel;

#[derive(Default)]
pub struct CompilerBuilder {
    optimization_level: Option<OptimizationLevel>,
}

impl CompilerBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_optimization_level(level: OptimizationLevel) -> Self {
        Self {
            optimization_level: Some(level),
        }
    }

    #[must_use]
    pub fn optimization_level(mut self, level: OptimizationLevel) -> Self {
        self.optimization_level = Some(level);

        self
    }

    fn build(self) -> Compiler {
        Compiler {
            optimization_level: self.optimization_level.unwrap_or_default(),
        }
    }
}

pub struct Compiler {
    job_count: 0,
    optimization_level: OptimizationLevel,
}

impl Compiler {
    /// Begins a compilation job of a source AST with a specified serializer
    /// for the resulting bytecode format.
    pub fn create_job<F: Fn(IlChunk) -> Vec<u8>>(source: &dyn Node, serializer: F) -> Vec<u8> {
        todo!()
    }
}
