use std::marker::PhantomData;

use crate::{Sample, Proc, Phase, Scale};
use crate::wave::WaveCompute;
use crate::modulate::Modulated;

const DEFAULT_PULSEWIDTH: Scale = 0.5;

/// Base type of oscillators, generic over [crate::wave::WaveShape] or [crate::wave::WaveSet].
#[derive(Debug, Default)]
pub struct OscCore<W> {
    _phantom: PhantomData<W>,
}

impl<W: WaveCompute> Proc<Phase, Sample> for OscCore<W> {
    type Spec = OscCoreSpec<W>;
    fn proc(&mut self, spec: &Self::Spec, phase: Phase) -> Sample {
        spec.wave.compute_aliasing(phase, spec.tone)
    }
}

impl<W: WaveCompute> Proc<(Phase, Phase), Sample> for OscCore<W> {
    type Spec = OscCoreSpec<W>;
    fn proc(&mut self, spec: &Self::Spec, (phase, dphase): (Phase, Phase)) -> Sample {
        spec.wave.compute_polyblep(phase, dphase, spec.tone)
    }
}

/// Spec for [OscCore].
#[derive(Debug, Clone)]
pub struct OscCoreSpec<W> {
    wave: W,
    tone: Scale,
}

impl<W: Default> Default for OscCoreSpec<W> {
    fn default() -> Self {
        Self {
            wave: W::default(),
            tone: DEFAULT_PULSEWIDTH,
        }
    }
}

impl<W> OscCoreSpec<W> {
    pub const fn new(wave: W, tone: Scale) -> Self {
        Self { wave, tone }
    }
    pub fn get_wave(&self) -> &W {
        &self.wave
    }

    pub fn get_wave_mut(&mut self) -> &mut W {
        &mut self.wave
    }

    pub fn get_tone(&self) -> Scale {
        self.tone
    }

    pub fn set_tone(&mut self, tone: Scale) {
        self.tone = tone;
    }
}

impl<W: Clone> Modulated for OscCoreSpec<W> {
    type Child = Self;
    fn modulated(&self) -> Self::Child {
        self.clone()
    }
}

#[cfg(test)]
mod test {
    use crate::wave::{WaveShape, WaveSet};
    use super::*;
    #[test]
    fn test_sizes() {
        use std::mem::size_of;
        assert_eq!(size_of::<OscCore<WaveShape>>(), 0);
        assert_eq!(size_of::<OscCoreSpec<WaveShape>>(), 16);
        assert_eq!(size_of::<OscCoreSpec<WaveSet>>(), 16);
    }
    #[test]
    fn test_osc_wave_shape() {
        let mut osc = OscCore::<WaveShape>::default();
        let mut spec = OscCoreSpec::default();

        // default is tri:
        assert_eq!(osc.proc(&spec, 0.75), 0.0);
        assert_eq!(osc.proc(&spec, 0.8), -0.20000000000000018);
        // tone should have no effect:
        spec.set_tone(0.1);
        assert_eq!(osc.proc(&spec, 0.75), 0.0);
        assert_eq!(osc.proc(&spec, 0.8), -0.20000000000000018);
        // try a pulse wave (where tone does have an effect):
        *spec.get_wave_mut() = WaveShape::Pulse;
        assert_eq!(osc.proc(&spec, 0.05), 1.0);
        assert_eq!(osc.proc(&spec, 0.15), -1.0);
        spec.set_tone(0.4);
        assert_eq!(osc.proc(&spec, 0.15), 1.0);
        assert_eq!(osc.proc(&spec, 0.5), -1.0);
        // compare raw and polyblep:
        assert_eq!(osc.proc(&spec, 0.35), 1.0);
        assert_eq!(osc.proc(&spec, (0.35, 0.1)), 0.7499999999999993);

    }
    #[test]
    fn test_osc_wave_set() {
        let mut osc = OscCore::<WaveSet>::default();
        let mut spec = OscCoreSpec::default();

        // default is nothing:
        assert_eq!(osc.proc(&spec, 0.3), 0.0);
        assert_eq!(osc.proc(&spec, 0.8), 0.0);

        // add a tri:
        spec.get_wave_mut().set_shape(WaveShape::Tri, true);
        let tri_out = osc.proc(&spec, 0.8);
        assert_eq!(tri_out, -0.20000000000000018);
        // try a sine:
        spec.get_wave_mut().set_shape(WaveShape::Tri, false);
        spec.get_wave_mut().set_shape(WaveShape::Sine, true);
        let sine_out = osc.proc(&spec, 0.8);
        assert_eq!(sine_out, -0.9510565162951536);
        // now do both at once:
        spec.get_wave_mut().set_shape(WaveShape::Tri, true);
        assert_eq!(osc.proc(&spec, 0.8), sine_out + tri_out);
        // add a pulse and compare polyblep output:
        spec.get_wave_mut().set_shape(WaveShape::Pulse, true);
        spec.set_tone(0.7);
        // yikes, hope the gain is under control somewhere else...
        assert_eq!(osc.proc(&spec, 0.75), -1.7);
        assert_eq!(osc.proc(&spec, (0.75, 0.1)), -1.5250000000000004);
    }
}
