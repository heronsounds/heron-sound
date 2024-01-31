//! Utilities for manipulating pitches (or phase increments).
//!
//! Phase increments are just pitch / sample_rate, so we generally prefer to perform that
//! calculation up front and deal with phase increments in all our processing code. Since the
//! conversion is just multiplying by a constant, this doesn't change the math at all.

mod cache;
pub use self::cache::PitchCache;

mod octave;
pub use self::octave::{OctaveSpec, Octave};

mod glide;
pub use self::glide::{GlideState, Glide, GlideSpec};

use crate::F;

/// Apply an offset in semitones to a base pitch (or phase increment).
pub fn apply_offset(base_pitch: F, offset_semis: F) -> F {
    crate::check_float_pos!(base_pitch);
    crate::check_float_finite!(offset_semis);
    const ONE_OVER_12: F = 1.0 / 12.0;
    base_pitch * 2.0_f64.powf(offset_semis * ONE_OVER_12)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_apply_offset() {
        assert_eq!(apply_offset(0.5, 0.1), 0.5028964705339267);
        assert_eq!(apply_offset(0.25, 12.0), 0.5);
    }
}
