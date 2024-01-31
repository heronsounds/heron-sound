use crate::{Sample, Scale, Phase};
use crate::util::Bitmask;

use super::{WaveShape, WaveCompute, SHAPES};

// in our synth implementations, we noticed that the perceived volume of the different
// wave shapes was vastly different, so we added these constants to normalize them.
// TODO these should really be passed in by the caller rather than hardcoded, though.
const SHAPE_VOLUMES: [Scale; 4] = [1.0, 0.7, 0.6, 1.0];

/// A set of [WaveShape]s that can compute multiple waveform outputs at once.
#[derive(Debug, Default, Clone)]
pub struct WaveSet {
    shapes: u8,
}

impl WaveSet {
    pub fn has_shape(&self, shape: WaveShape) -> bool {
        self.shapes.get_bit(shape.into_usize())
    }

    pub fn set_shape(&mut self, shape: WaveShape, set_or_unset: bool) {
        self.shapes.set_bit_if(shape.into_usize(), set_or_unset);
    }
}

impl WaveCompute for WaveSet {
    fn compute_aliasing(&self, phase: Phase, tone: Scale) -> Sample {
        let mut output = 0.0;
        for i in 0..SHAPES.len() {
            let shape = SHAPES[i];
            if self.has_shape(shape) {
                output += shape.compute_aliasing(phase, tone) * SHAPE_VOLUMES[i];
            }
        }
        output
    }
    fn compute_polyblep(&self, phase: Phase, dphase: Phase, tone: Scale) -> Sample {
        let mut output = 0.0;
        for i in 0..SHAPES.len() {
            let shape = SHAPES[i];
            if self.has_shape(shape) {
                output += shape.compute_polyblep(phase, dphase, tone) * SHAPE_VOLUMES[i];
            }
        }
        output
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::{SAW, PULSE, TRI, SINE};
    #[test]
    fn test_size() {
        assert_eq!(std::mem::size_of::<WaveSet>(), 1);
    }
    #[test]
    fn test_has_shape() {
        let mut set = WaveSet { shapes: u8::only_bit(PULSE as usize) };
        assert!(set.has_shape(WaveShape::Pulse));
        assert!(!set.has_shape(WaveShape::Saw));
        set.shapes.set_bit(SAW as usize);
        assert!(set.has_shape(WaveShape::Saw));
    }
    #[test]
    fn test_set_shape() {
        let mut set = WaveSet::default();
        assert!(!set.has_shape(WaveShape::Sine));
        set.set_shape(WaveShape::Sine, true);
        assert!(set.has_shape(WaveShape::Sine));
        set.set_shape(WaveShape::Sine, false);
        assert!(!set.has_shape(WaveShape::Sine));
    }
    #[test]
    fn test_compute_aliasing() {
        let mut set = WaveSet::default();
        set.set_shape(WaveShape::Tri, true);
        let tri_out = WaveShape::Tri.compute_aliasing(0.6, 0.5);
        let sine_out = WaveShape::Sine.compute_aliasing(0.6, 0.5);
        assert_eq!(set.compute_aliasing(0.6, 0.5), tri_out * SHAPE_VOLUMES[TRI as usize]);

        set.set_shape(WaveShape::Sine, true);
        assert_eq!(set.compute_aliasing(0.6, 0.5),
            tri_out * SHAPE_VOLUMES[TRI as usize] + sine_out * SHAPE_VOLUMES[SINE as usize])
    }
    #[test]
    fn test_polyblep() {
        let mut set = WaveSet::default();
        set.set_shape(WaveShape::Pulse, true);
        let pulse_out = WaveShape::Pulse.compute_polyblep(0.95, 0.1, 0.5);
        let saw_out = WaveShape::Saw.compute_polyblep(0.95, 0.1, 0.5);
        assert_eq!(set.compute_polyblep(0.95, 0.1, 0.5), pulse_out * SHAPE_VOLUMES[PULSE as usize]);

        set.set_shape(WaveShape::Saw, true);
        assert_eq!(set.compute_polyblep(0.95, 0.1, 0.5),
            pulse_out * SHAPE_VOLUMES[PULSE as usize] + saw_out * SHAPE_VOLUMES[SAW as usize])
    }
}
