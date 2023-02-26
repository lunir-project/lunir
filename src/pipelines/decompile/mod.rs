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
use std::{
    marker::PhantomData,
    sync::{Arc, Weak},
};

#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct NoReconstructor;

#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct WithReconstructor<'a, V: Visitor<'a>> {
    pub visitor: V,
    _marker: PhantomData<&'a V>,
}

#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct NoChunk;

#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct WithChunk(IlChunk);

/// The interface of LUNIR's decompilation pipeline. `DecompilationJob` allows you to pass in parameters to the LUNIR decompilation pipeline and invoke it, even across threads.
#[derive(Clone, Debug)]
pub struct DecompilationJob<C, F> {
    chunk: C,
    optimization_level: OptimizationLevel,
    _reference: Weak<()>,
    reconstructor: F,
}

impl<C, F> DecompilationJob<C, F> {
    /// Sets the optimization level of this `DecompilationJob`.
    pub fn optimization_level(mut self, level: OptimizationLevel) -> Self {
        self.optimization_level = level;

        self
    }
}

impl<'a, C> DecompilationJob<C, NoReconstructor> {
    /// Adds a target format source reconstruction visitor to this `DecompilationJob` to allow it to produce a final source string.
    pub fn reconstructor<V: Visitor<'a>>(
        self,
        visitor: V,
    ) -> DecompilationJob<C, WithReconstructor<'a, V>> {
        DecompilationJob {
            chunk: self.chunk,
            optimization_level: self.optimization_level,
            _reference: self._reference,
            reconstructor: WithReconstructor {
                visitor,
                _marker: PhantomData,
            },
        }
    }
}

impl<'a, F> DecompilationJob<NoChunk, F> {
    /// Adds a source LUNIR intermediate language chunk to this `DecompilationJob`.
    pub fn chunk(self, chunk: IlChunk) -> DecompilationJob<WithChunk, F> {
        DecompilationJob {
            chunk: WithChunk(chunk),
            optimization_level: self.optimization_level,
            _reference: self._reference,
            reconstructor: self.reconstructor,
        }
    }
}

impl<'a, V: Visitor<'a>> DecompilationJob<WithChunk, WithReconstructor<'a, V>> {
    /// Invokes LUNIR's decompilation pipeline with the parameters passed through the this `DecompilationJob`. This will consume the job.
    #[must_use = "The result of decompilation should be used."]
    pub fn run(self) -> String {
        todo!()
    }
}

/// A factory for `DecompilationJob`s.
pub struct Decompiler {
    handle: Arc<()>,
}

impl Decompiler {
    /// Creates a new `Decompiler`.
    pub fn new() -> Self {
        Self {
            handle: Arc::new(()),
        }
    }
}

impl Decompiler {
    /// Constructs a `DecompilationJob`.
    pub fn create_job(&self) -> DecompilationJob<NoChunk, NoReconstructor> {
        DecompilationJob {
            chunk: NoChunk,
            _reference: Arc::downgrade(&self.handle),
            optimization_level: OptimizationLevel::default(),
            reconstructor: NoReconstructor,
        }
    }

    /// Returns the number of currently living `DecompilationJob`s created by this `Decompiler` or by cloning `DecompilationJob`s created by this `Decompiler`.
    pub fn job_count(&self) -> usize {
        Arc::weak_count(&self.handle)
    }
}
