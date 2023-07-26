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
