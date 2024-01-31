//! These macros perform bounds checks for common types in debug builds.
//!
//! In release builds, they do nothing.
//! This may be a premature optimization.

/// Check that float values representing phases are in range `0..1`.
#[macro_export]
macro_rules! check_phase_bounds {
    ($phase:expr) => {
        debug_assert!(
            $phase.is_finite() && $phase >= 0.0 && $phase < 1.0,
            "Phase value {} is not in valid range 0..1",
            $phase,
        )
    };
}

/// Check that float values are in range `0..=1`.
#[macro_export]
macro_rules! check_float_01 {
    ($val:expr) => {
        debug_assert!(
            $val.is_finite() && $val >= 0.0 && $val <= 1.0,
            "Float value {} is not in valid range 0..=1",
            $val,
        )
    }
}

/// Check that float values representing Hz are positive and non-zero.
#[macro_export]
macro_rules! check_hz_bounds {
    ($hz:expr) => {
        debug_assert!(
            $hz.is_finite() && $hz > 0.0,
            "Hz value {} is not in valid range (positive and nonzero)",
            $hz,
        )
    };
}

/// Check that float values are nonnegative.
#[macro_export]
macro_rules! check_float_nonneg {
    ($val:expr) => {
        debug_assert!(
            $val.is_finite() && $val >= 0.0,
            "Float value {} should be finite and nonnegative",
            $val,
        )
    };
}

/// Check that float values are positive and nonzero.
#[macro_export]
macro_rules! check_float_pos {
    ($val:expr) => {
        debug_assert!(
            $val.is_finite() && $val > 0.0,
            "Float value {} should be positive and nonzero",
            $val,
        )
    };
}

/// Check that float values are nonzero.
#[macro_export]
macro_rules! check_float_nonzero {
    ($val:expr) => {
        debug_assert!(
            $val.is_finite() && $val != 0.0 && $val != -0.0,
            "Float value {} should be nonzero",
            $val,
        )
    };
}

/// Check that float values are finite.
#[macro_export]
macro_rules! check_float_finite {
    ($val:expr) => {
        debug_assert!(
            $val.is_finite(),
            "Float value {} should be finite",
            $val,
        )
    };
}

/// Check that int values are less than a constant.
#[macro_export]
macro_rules! check_int_less_than {
    ($val:expr, $bound:expr) => {
        debug_assert!(
            $val < $bound,
            "Int value {} should be less than {}",
            $val,
            $bound,
        )
    };
}
