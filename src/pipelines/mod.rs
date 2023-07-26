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

#[cfg(feature = "compile")]
pub(crate) mod compile;

#[cfg(feature = "decompile")]
pub(crate) mod decompile;

/// Defines the level of optimisation that the LUNIR pipeline should apply, in general:
/// - All includes all optimisations
/// - Moderate includes all optimisations that are safe from miscompilation
/// - None includes no deliberate optimisations
#[derive(Default, Clone, Debug)]
pub enum OptimizationLevel {
    All,
    #[default]
    Moderate,
    None,
}

#[cfg(feature = "compile")]
pub use compile::Compiler;

#[cfg(feature = "decompile")]
pub use decompile::Decompiler;
