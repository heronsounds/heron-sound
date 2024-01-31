//! Low-level computations for oscillator waveforms.

use crate::{Scale, Phase, Sample};

mod compute;

mod wave_set;
pub use wave_set::WaveSet;

mod wave_shape;
pub use wave_shape::WaveShape;

const TRI: u8 = 0;
const PULSE: u8 = 1;
const SAW: u8 = 2;
const SINE: u8 = 3;

const SHAPES: [WaveShape; 4] =
    [WaveShape::Tri, WaveShape::Pulse, WaveShape::Saw, WaveShape::Sine];

/// Trait for values that act like waveforms.
pub trait WaveCompute {
    fn compute_aliasing(&self, phase: Phase, tone: Scale) -> Sample;
    fn compute_polyblep(&self, phase: Phase, dphase: Phase, tone: Scale) -> Sample;
}
