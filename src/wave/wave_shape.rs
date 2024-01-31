use crate::{Phase, Sample, Scale};
use super::{compute, WaveCompute, TRI, PULSE, SAW, SINE};

/// Enum for basic waveform types.
#[derive(Debug, Clone, Copy)]
pub enum WaveShape {
    Tri, Pulse, Saw, Sine,
}

impl Default for WaveShape {
    fn default() -> Self { Self::Tri }
}

impl WaveShape {
    pub const fn into_u8(self) -> u8 {
        match self {
            Self::Tri   => TRI,
            Self::Pulse => PULSE,
            Self::Saw   => SAW,
            Self::Sine  => SINE,
        }
    }

    pub const fn into_usize(self) -> usize {
        self.into_u8() as usize
    }

    pub fn from_u8(idx: u8) -> Self {
        match idx {
            TRI     => Self::Tri,
            PULSE   => Self::Pulse,
            SAW     => Self::Saw,
            SINE    => Self::Sine,
            _ => unimplemented!("WaveShape index > 3"),
        }
    }
}

impl WaveCompute for WaveShape {
    fn compute_aliasing(&self, phase: Phase, tone: Scale) -> Sample {
        match self {
            Self::Tri   => compute::raw::tri(phase),
            Self::Pulse => compute::raw::pulse(phase, tone),
            Self::Saw   => compute::raw::saw(phase),
            Self::Sine  => compute::raw::sine(phase),
        }
    }

    fn compute_polyblep(&self, phase: Phase, dphase: Phase, tone: Scale) -> Sample {
        match self {
            Self::Tri   => compute::polyblep::tri(phase, dphase),
            Self::Pulse => compute::polyblep::pulse(phase, dphase, tone),
            Self::Saw   => compute::polyblep::saw(phase, dphase),
            Self::Sine  => compute::raw::sine(phase),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_size() {
        assert_eq!(std::mem::size_of::<WaveShape>(), 1);
    }
    #[test]
    fn test_default() {
        assert!(matches!(WaveShape::default(), WaveShape::Tri));
    }
    #[test]
    fn test_into_u8() {
        assert_eq!(WaveShape::Tri.into_u8(), TRI);
        assert_eq!(WaveShape::Pulse.into_u8(), PULSE);
        assert_eq!(WaveShape::Saw.into_u8(), SAW);
        assert_eq!(WaveShape::Sine.into_u8(), SINE);
    }
    #[test]
    fn test_into_usize() {
        assert_eq!(WaveShape::Tri.into_usize(), TRI as usize);
        assert_eq!(WaveShape::Pulse.into_usize(), PULSE as usize);
        assert_eq!(WaveShape::Saw.into_usize(), SAW as usize);
        assert_eq!(WaveShape::Sine.into_usize(), SINE as usize);
    }
    #[test]
    fn test_from_u8() {
        assert!(matches!(WaveShape::from_u8(TRI), WaveShape::Tri));
        assert!(matches!(WaveShape::from_u8(PULSE), WaveShape::Pulse));
        assert!(matches!(WaveShape::from_u8(SAW), WaveShape::Saw));
        assert!(matches!(WaveShape::from_u8(SINE), WaveShape::Sine));
    }
    #[test]
    fn test_compute_aliasing() {
        assert_eq!(WaveShape::Tri.compute_aliasing(0.61, 0.5), compute::raw::tri(0.61));
        assert_eq!(WaveShape::Pulse.compute_aliasing(0.75, 0.5), -1.0);
        assert_eq!(WaveShape::Saw.compute_aliasing(0.42, 0.5), compute::raw::saw(0.42));
        assert_eq!(WaveShape::Sine.compute_aliasing(0.78, 0.5), compute::raw::sine(0.78));
    }
    #[test]
    fn test_compute_polyblep() {
        assert_eq!(WaveShape::Tri.compute_polyblep(0.99, 0.1, 0.5), compute::polyblep::tri(0.99, 0.1));
        assert_eq!(WaveShape::Pulse.compute_polyblep(0.99, 0.1, 0.5), compute::polyblep::pulse(0.99, 0.1, 0.5));
        assert_eq!(WaveShape::Saw.compute_polyblep(0.99, 0.1, 0.5), compute::polyblep::saw(0.99, 0.1));
        assert_eq!(WaveShape::Sine.compute_polyblep(0.99, 0.1, 0.5), compute::raw::sine(0.99));
    }
}
