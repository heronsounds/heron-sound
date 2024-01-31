//! Oscillator implementations.

mod core;
pub use self::core::{OscCore, OscCoreSpec};

use crate::{Phase, Sample, Proc};
use crate::phase::Phasor;
use crate::wave::{WaveShape, WaveSet};

/// Oscillator that only ever computes one waveform at a time.
pub type SingleOsc = OscCore<WaveShape>;
/// Oscillator that can stack multiple waveforms at once.
pub type MultiOsc = OscCore<WaveSet>;

/// Spec for [SingleOsc].
pub type SingleOscSpec = OscCoreSpec<WaveShape>;
/// Spec for [MultiOsc].
pub type MultiOscSpec = OscCoreSpec<WaveSet>;

// PhasorOscs store a Phasor in their state, and are useful as sound sources.
// Lfos might want more control over the Phasor
// (tho honestly I forget why I originally did that - something to do with lag)

/// An oscillator tied to a [Phasor] so it can keep track of phase internally.
#[derive(Debug, Default)]
pub struct PhasorOsc<O> {
    phasor: Phasor,
    osc: O,
}

impl<O: Proc<Phase, Sample>> Proc<Phase, Sample> for PhasorOsc<O> {
    type Spec = O::Spec;
    fn proc(&mut self, spec: &O::Spec, dphase: Phase) -> Sample {
        let phase = self.phasor.advance(dphase);
        self.osc.proc(spec, phase)
    }
}

/// An oscillator, tied to a [Phasor], that produces anti-aliased output where applicable.
#[derive(Debug, Default)]
pub struct PolyblepPhasorOsc<O> {
    phasor: Phasor,
    osc: O,
}

impl<O> PolyblepPhasorOsc<O> {
    pub fn reset(&mut self, phase: Phase) {
        self.phasor.set(phase);
    }
}

impl<O: Proc<(Phase, Phase), Sample>> Proc<Phase, Sample> for PolyblepPhasorOsc<O> {
    type Spec = O::Spec;
    fn proc(&mut self, spec: &O::Spec, dphase: Phase) -> Sample {
        let phase = self.phasor.advance(dphase);
        self.osc.proc(spec, (phase, dphase))
    }
}

#[cfg(test)]
mod test {
    use crate::wave::WaveShape;
    use super::*;
    #[test]
    fn test_size() {
        assert_eq!(std::mem::size_of::<PhasorOsc<SingleOsc>>(), 8);
    }
    #[test]
    fn test_osc() {
        let mut osc = PhasorOsc::<SingleOsc>::default();
        let mut spec = SingleOscSpec::default();
        // will be tri by default:
        assert_eq!(osc.proc(&spec, 0.1), -1.0);
        assert_eq!(osc.proc(&spec, 0.1), -0.6);
        assert_eq!(osc.proc(&spec, 0.1), -0.19999999999999996);
        assert_eq!(osc.proc(&spec, 0.1), 0.20000000000000018);
        assert_eq!(osc.proc(&spec, 0.1), 0.6000000000000001);
        assert_eq!(osc.proc(&spec, 0.1), 1.0);
        assert_eq!(osc.proc(&spec, 0.1), 0.6000000000000001);
        // set to pulse and try a polyblep:
        *spec.get_wave_mut() = WaveShape::Pulse;
        let mut osc = PolyblepPhasorOsc::<SingleOsc>::default();
        osc.phasor.set(0.48124);
        assert_eq!(osc.proc(&spec, 0.01), 1.0);
        assert_eq!(osc.proc(&spec, 0.01), 0.9846240000000025);
        assert_eq!(osc.proc(&spec, 0.01), -0.2326240000000228);
        assert_eq!(osc.proc(&spec, 0.01), -1.0);
    }
}
