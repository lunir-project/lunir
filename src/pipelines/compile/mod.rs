//    Copyright 2023 lunir-project

//    Licensed under the Apache License, Version 2.0 (the "License");
//    you may not use this file except in compliance with the License.
//    You may obtain a copy of the License at

//        http://www.apache.org/licenses/LICENSE-2.0

//    Unless required by applicable law or agreed to in writing, software
//    distributed under the License is distributed on an "AS IS" BASIS,
//    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//    See the License for the specific language governing permissions and
//    limitations under the License.

use super::OptimizationLevel;
use crate::ir::{ast::tree::*, il::IlChunk};
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
    pub fn optimization_level(mut self, level: OptimizationLevel) -> Self {
        self.optimization_level = level;

        self
    }
}
impl<'a, T> CompilationJob<T, NoSerializer> {
    /// Adds a target format serializer function to this `CompilationJob` to allow it to produce a final bytecode.
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
    /// Invokes LUNIR's compilation pipeline with the parameters passed through the `CompilationJob`. This will consume the job.
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
