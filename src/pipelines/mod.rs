pub(crate) mod compile;
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

pub use compile::Compiler;
