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

use super::OptimizationLevel;
use crate::{ast::tree::*, il::IlChunk};
use std::sync::{Arc, Weak};

#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct NoSerializer;
#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct WithSerializer<S: Fn(IlChunk) -> Vec<u8>>(S);

#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct NoTree;
#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct WithTree<'n>(&'n Node);

/// The interface of LUNIR's compilation pipeline. `CompilationJob` allows you to pass in parameters to the LUNIR compilation pipeline and invoke it, even across threads.
#[derive(Clone, Debug)]
pub struct CompilationJob<T, F> {
    optimization_level: OptimizationLevel,
    _reference: Weak<()>,
    serializer: F,
    tree: T,
}

impl<T, F> CompilationJob<T, F> {
    /// Sets the optimization level of this `CompilationJob`.
    fn optimization_level(mut self, level: OptimizationLevel) -> Self {
        self.optimization_level = level;

        self
    }
}
impl<'a, T> CompilationJob<T, NoSerializer> {
    /// Adds a target format serializer function to this `CompilationJob` toa llow it to produce a final bytecode.
    pub fn serializer<S: Fn(IlChunk) -> Vec<u8>>(
        self,
        serializer: S,
    ) -> CompilationJob<T, WithSerializer<S>> {
        CompilationJob {
            optimization_level: self.optimization_level,
            _reference: self._reference,
            serializer: WithSerializer(serializer),
            tree: self.tree,
        }
    }
}

impl<'a, F> CompilationJob<NoTree, F> {
    /// Adds a source abstract syntax tree to this `CompilationJob`.
    pub fn tree(self, tree: &'a Node) -> CompilationJob<WithTree<'a>, F> {
        CompilationJob {
            optimization_level: self.optimization_level,
            _reference: self._reference,
            serializer: self.serializer,
            tree: WithTree(tree),
        }
    }
}

impl<'a, S: Fn(IlChunk) -> Vec<u8>> CompilationJob<WithTree<'a>, WithSerializer<S>> {
    /// Invokes LUNIR's compilation pipeline with the parameters passed through the this `CompilationJob`. This will consume the job.
    #[must_use = "The result of compilation should be used."]
    pub fn run(self) -> Vec<u8> {
        todo!()
    }
}

/// A factory for `CompilationJob`s.
pub struct Compiler {
    handle: Arc<()>,
}

impl Compiler {
    /// Creates a new `Compiler`.
    pub fn new() -> Self {
        Self {
            handle: Arc::new(()),
        }
    }
}

impl Compiler {
    /// Constructs a `CompilationJob`.
    pub fn create_job(&self) -> CompilationJob<NoTree, NoSerializer> {
        CompilationJob {
            _reference: Arc::downgrade(&self.handle),
            optimization_level: OptimizationLevel::default(),
            serializer: NoSerializer,
            tree: NoTree,
        }
    }

    /// Returns the number of currently living `CompilationJob`s created by this compiler or by cloning `CompilationJob`s created by this compiler.
    pub fn job_count(&self) -> usize {
        Arc::weak_count(&self.handle)
    }
}
