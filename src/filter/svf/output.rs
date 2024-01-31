use crate::{Sample, F};

use super::VFactors;

/// Trait for computing filter output from V factors.
pub trait SvfOutput {
    fn get(&self, v: VFactors, k: F) -> Sample;
}

/// Compute low-pass filter output.
#[derive(Debug, Default)]
pub struct LowPass;
impl SvfOutput for LowPass {
    fn get(&self, v: VFactors, _k: F) -> Sample {
        v[2]
    }
}

/// Compute high-pass filter output.
#[derive(Debug, Default)]
pub struct HighPass;
impl SvfOutput for HighPass {
    fn get(&self, v: VFactors, k: F) -> Sample {
        v[0] - k * v[1] - v[2]
    }
}

/// Compute magic-peak output (peak filter output minus input sample).
// magic peak is peak minus v0.
#[derive(Debug, Default)]
pub struct MagicPeak;
impl SvfOutput for MagicPeak {
    fn get(&self, v: VFactors, k: F) -> Sample {
        -k * v[1] - 2.0 * v[2]
    }
}

/// Compute band-pass filter output.
#[derive(Debug, Default)]
pub struct BandPass;
impl SvfOutput for BandPass {
    fn get(&self, v: VFactors, _k: F) -> Sample {
        v[1]
    }
}

/// Compute notch filter output.
#[derive(Debug, Default)]
pub struct Notch;
impl SvfOutput for Notch {
    fn get(&self, v: VFactors, k: F) -> Sample {
        v[0] - k * v[1]
    }
}

/// Compute peak filter output.
#[derive(Debug, Default)]
pub struct Peak;
impl SvfOutput for Peak {
    fn get(&self, v: VFactors, k: F) -> Sample {
        v[0] - k * v[1] - 2.0 * v[2]
    }
}

/// Compute all-pass filter output.
#[derive(Debug, Default)]
pub struct AllPass;
impl SvfOutput for AllPass {
    fn get(&self, v: VFactors, k: F) -> Sample {
        v[0] - 2.0 * k * v[1]
    }
}
