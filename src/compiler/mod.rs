use crate::il::IlChunk;

pub struct CompilerSettings {}

#[derive(Default)]
pub struct CompilerBuilder {
    settings: Option<CompilerSettings>,
}

impl CompilerBuilder {
    fn new() -> Self {
        Self::default()
    }

    fn with_settings(settings: CompilerSettings) -> Self {
        Self {
            settings: Some(settings),
        }
    }

    fn settings(&mut self, settings: CompilerSettings) -> &mut Self {
        self.settings = Some(settings);

        self
    }
}

pub struct Compiler {
    settings: CompilerSettings,
}

/// The format of bytecode specific to a certain Lua version.
pub trait BytecodeFormat {
    fn serialize(&self, chunk: IlChunk) -> Vec<u8>;
    fn deserialize(&self, bytecode: impl AsRef<[u8]>) -> IlChunk;
}

impl Compiler {
    /// Begins a decompilation job of bytecode with a specified format.
    pub fn compile(bytecode: impl AsRef<[u8]>, fmt: impl BytecodeFormat) -> String {
        todo!()
    }
}
