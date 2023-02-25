pub(crate) mod compile;
pub(crate) mod decompile;

#[derive(Default)]
pub enum OptimizationLevel {
    All,
    #[default]
    Moderate,
    None,
}

pub use compile::Compiler;
