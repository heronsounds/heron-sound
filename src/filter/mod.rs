//! Filter implementations.

/// State-variable filter implementations.
pub mod svf;

/// Enum representing a filter's type (high-pass, low-pass, etc.).
#[derive(Debug, Clone, Copy)]
pub enum FilterType {
    Low,
    High,
    Peak,
    Band,
    Notch,
    All,
    MagicPeak,
}

impl Default for FilterType {
    fn default() -> Self { Self::Low }
}
