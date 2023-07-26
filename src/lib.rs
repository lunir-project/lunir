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

#![doc = include_str!("../README.md")]

#[cfg(feature = "ir")]
pub mod ir;

#[cfg(not(feature = "ir"))]
pub(crate) mod ir;

/// The LUNIR compilation and decompilation pipelines, requires the `compile` or `decompile` features to be enabled..
#[cfg(any(feature = "compile", feature = "decompile"))]
pub mod pipelines;

/// Commonly used types and functions.
pub mod prelude;
