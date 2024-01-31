//! Miscellaneous useful stuff.

mod bitmask;
pub use self::bitmask::Bitmask;

mod bitmask_impls;

mod inverse;
pub use self::inverse::{AddInv, MulInv};

mod midi_note_pitches;
pub use self::midi_note_pitches::MIDI_NOTE_PITCHES;

mod time;
pub use self::time::Time;

mod accessor_macros;
mod bounds_check_macros;

use crate::F;

/// Split a float into its whole part (as usize) and fractional part (as float).
pub fn split_f(f: F) -> (usize, F) {
    let whole_part = f.floor();
    let frac_part = f - whole_part;
    (whole_part as usize, frac_part)
}

/// Linearly interpolate between two values, according to a fractional offset.
///
/// frac is assumed to be between 0.0 and 1.0.
pub fn lirp(y0: F, y1: F, frac: F) -> F {
    crate::check_float_01!(frac);
    y0 + frac * (y1 - y0)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_split_f() {
        assert_eq!(split_f(0.0), (0, 0.0));
        assert_eq!(split_f(1.0), (1, 0.0));
        assert_eq!(split_f(3.65), (3, 0.6499999999999999));
    }
    #[test]
    fn test_lirp() {
        assert_eq!(lirp(0.0, 1.0, 0.5), 0.5);
        assert_eq!(lirp(0.6, 1.2, 0.25), 0.75);
    }
}
