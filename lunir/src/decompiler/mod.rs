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

// now we gotta make the visitor to build our lua code
pub struct Decompiler {
    settings: DecompilerSettings,
}

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

use crate::il::Instruction;

impl Decompiler {
    /// Begins a decompilation job of bytecode with a specified format.
    pub fn decompile(bytecode: impl AsRef<[u8]>, format: BytecodeFormat) -> String {
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
    pub fn decompile_with_lifter<F: Fn(Box<dyn AsRef<[u8]>>) -> Vec<Instruction>>(
        bytecode: Box<dyn AsRef<[u8]>>,
        lifter: F,
    ) -> String {
        let il = lifter(bytecode);
        todo!()
    }
}
