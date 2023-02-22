
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

pub struct DecompilerSettings {}

#[derive(Default)]
pub struct DecompilerBuilder {
    settings: Option<DecompilerSettings>,
}

impl DecompilerBuilder {
    fn new() -> Self {
        Self::default()
    }

    fn with_settings(settings: DecompilerSettings) -> Self {
        Self {
            settings: Some(settings),
        }
    }

    fn settings(&mut self, settings: DecompilerSettings) -> &mut Self {
        self.settings = Some(settings);

        self
    }
}

pub struct Decompiler {
    settings: DecompilerSettings,
}

// Can be replaced with a trait of the same name to allow bytecode
// formats to be structs but I'm not sure about that, I like the simplicity
// of just passing an arbitrary function.

/// The format of bytecode specific to a certain Lua version.
pub enum BytecodeFormat {
    LuaP,
    LuaQ,
    LuaR,
    LuaS,
    LuaT,
    LuauV1,
    LuauV2,
    LuauV3,
}

use crate::il::{IlChunk};

impl Decompiler {
    /// Begins a decompilation job of bytecode with a specified format.
    pub fn decompile(_bytecode: impl AsRef<[u8]>, format: BytecodeFormat) -> String {
        match format {
            BytecodeFormat::LuaP => todo!(),
            BytecodeFormat::LuaQ => todo!(),
            BytecodeFormat::LuaR => todo!(),
            BytecodeFormat::LuaS => todo!(),
            BytecodeFormat::LuaT => todo!(),
            BytecodeFormat::LuauV1 => todo!(),
            BytecodeFormat::LuauV2 => todo!(),
            BytecodeFormat::LuauV3 => todo!(),
        }
    }

    /// Begins a decompilation job with a custom lifter to allow for
    /// decompilation of unsupported bytecode formats.
    pub fn decompile_with_lifter<F: Fn(Box<dyn AsRef<[u8]>>) -> IlChunk>(
        bytecode: Box<dyn AsRef<[u8]>>,
        lifter: F,
    ) -> String {
        let _il = lifter(bytecode);

        todo!()
    }
}
