use crate::{Hz, Seconds, Sample, Phase, Gen, Proc};
use crate::clock::{Clock, SetClock};
use crate::phase::Phasor;

/// An Lfo.
///
/// Internally it's just an oscillator and a Phasor. The expected usage is the main difference;
/// where oscs expect to have their phase increment passed in based on a pitch calculation,
/// Lfos store a rate in the spec and use that to compute the next phase increment.
// TODO just use PhasorOsc for this w/ a different spec, and impl Gen instead of Proc.
#[derive(Debug, Default)]
pub struct BasicLfo<O> {
    phasor: Phasor,
    osc: O,
}

impl<O: Proc<Phase, Sample>> Gen<Sample> for BasicLfo<O> {
    type Spec = BasicLfoSpec<O::Spec>;
    fn gen(&mut self, spec: &Self::Spec) -> Sample {
        if spec.rate == 0.0 {
            return 0.0;
        }

        let phase = self.phasor.advance(spec.dphase);
        self.osc.proc(&spec.osc, phase)
    }
}

impl<O> BasicLfo<O> {
    pub fn reset(&mut self) {
        self.phasor.set(0.0);
    }
}

/// Spec for [BasicLfo].
#[derive(Debug, Default)]
pub struct BasicLfoSpec<O> {
    /// osc spec
    osc: O,
    /// tick time, based on sample rate
    tick: Seconds,
    /// user-specified rate in Hz
    rate: Hz,
    /// phase increment (rate * tick)
    dphase: Phase,
}

impl<O> SetClock for BasicLfoSpec<O> {
    fn set_clock(&mut self, clock: &Clock) {
        self.tick = clock.tick;
        self.dphase = self.rate * self.tick;
    }
}

impl<O> BasicLfoSpec<O> {
    pub fn get_osc(&self) -> &O {
        &self.osc
    }

    pub fn get_osc_mut(&mut self) -> &mut O {
        &mut self.osc
    }

    pub fn get_rate(&self) -> Hz {
        self.rate
    }

    pub fn set_rate(&mut self, rate: Hz) {
        crate::check_float_nonneg!(rate);
        self.rate = rate;
        self.dphase = self.tick * self.rate;
    }
}

#[cfg(test)]
mod test {
    use crate::osc::{SingleOsc, SingleOscSpec};
    use crate::wave::WaveShape;
    use super::*;
    #[test]
    fn test_sizes() {
        use std::mem::size_of;
        assert_eq!(size_of::<BasicLfo<SingleOsc>>(), 8);
        assert_eq!(size_of::<BasicLfoSpec<SingleOscSpec>>(), 40);
    }
    #[test]
    fn test_lfo() {
        let mut lfo = BasicLfo::<SingleOsc>::default();
        let mut spec = BasicLfoSpec::<SingleOscSpec>::default();

        spec.set_clock(&Clock::new(44_100.0));
        *spec.osc.get_wave_mut() = WaveShape::Pulse;
        spec.set_rate(22_000.0);

        // always some weirdness when we're first starting out:
        assert_eq!(lfo.gen(&spec), 1.0);
        assert_eq!(lfo.gen(&spec), 1.0);
        assert_eq!(lfo.gen(&spec), -1.0);
        assert_eq!(lfo.gen(&spec), 1.0);
        assert_eq!(lfo.gen(&spec), -1.0);
        assert_eq!(lfo.gen(&spec), 1.0);
    }
}
