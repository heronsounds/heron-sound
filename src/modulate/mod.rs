//! Utilities for modulating parameters.
//!
//! These APIs work fine, but they're a little messy and we're hoping for a nice clean refactor
//! soon.

mod accumulate;

mod spec;
pub use self::spec::{ModArraySpec, EnvArraySpec, LfoArraySpec};

/// Trait for specs whose parameters can be modulated.
///
/// The expected usage is that processors who want to modulate their params will take a combined
/// input consisting of the actual input value and all modulator outputs (e.g. an input sample,
/// and LFO outputs) in a single struct. Then, internally, they'll call spec.modulated() to
/// produce a new owned value that they can modify according to the modulated parameters. They
/// pass that value to any internal `Proc` or `Gen` implementations. This workflow can probably
/// be improved.
// TODO rename all this, not sure what. "Cooked".
pub trait Modulated {
    /// Type produced when [Self::modulated] is called (can be `Self`).
    // TODO call it Processed? Cooked?
    type Child;

    /// Create a new spec whose parameters can be modulated.
    // TODO call it fn start_cook()?
    fn modulated(&self) -> Self::Child;
}
