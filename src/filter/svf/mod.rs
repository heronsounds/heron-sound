mod core;
use self::core::{SvfCoef, SvfState};
pub use self::core::{SvfSpec, CookedSvfSpec};

mod output;
use self::output::SvfOutput;
pub use self::output::{HighPass, LowPass, MagicPeak, Peak, BandPass, Notch, AllPass};

use crate::{Sample, Proc, F};

const TWO_OVER_PI: F = 2.0 / ::core::f64::consts::PI;

type VFactors = [F; 3];

/// 1st-order Svf.
pub type Svf<O> = SvfProc<O, SimpleSampling, 1>;
/// Oversampled 1st-order Svf.
pub type OversamplingSvf<O> = SvfProc<O, Oversampling, 1>;

/// 2nd-order Svf.
pub type Svf2<O> = SvfProc<O, SimpleSampling, 2>;
/// Oversampled 2nd-order Svf.
pub type OversamplingSvf2<O> = SvfProc<O, Oversampling, 2>;

/// Basic type of an Svf filter.
///
/// Combines an underlying Svf with a sampling wrapper
/// (either [SimpleSampling] or [Oversampling]).
#[derive(Debug, Default)]
pub struct SvfProc<O, S, const N: usize> {
    state: SvfStages<O, N>,
    sampling: S,
}

impl<O, S, const N: usize> Proc<Sample, Sample> for SvfProc<O, S, N>
where
    S: SvfSampling<O, N>,
    O: SvfOutput,
{
    type Spec = CookedSvfSpec;
    fn proc(&mut self, spec: &Self::Spec, sample: Sample) -> Sample {
        self.sampling.apply(&mut self.state, spec, sample)
    }
}

impl<O, S, const N: usize> SvfProc<O, S, N> {
    pub fn reset(&mut self) {
        self.state.reset();
    }
}

#[derive(Debug)]
struct SvfStages<O, const N: usize> {
    coef: SvfCoef,
    state: [SvfState; N],
    output: O,
}

impl<O, const N: usize> Default for SvfStages<O, N>
where
    O: Default,
{
    fn default() -> Self {
        Self { coef: Default::default(), state: [SvfState::default(); N], output: O::default() }
    }
}

impl<O, const N: usize> SvfStages<O, N> {
    pub fn reset(&mut self) {
        for state in &mut self.state {
            *state = SvfState::default();
        }
    }
}

impl<O, const N: usize> SvfStages<O, N>
where
    O: SvfOutput,
{
    pub fn apply(&mut self, spec: &CookedSvfSpec, sample: Sample) -> Sample {
        let coef = self.coef.apply(spec.get_g_prime(), spec.get_k());
        let mut output = sample;
        for state in &mut self.state {
            let v = state.apply(&coef, output);
            output = self.output.get(v, spec.get_k());
        }
        output
    }

    // experiment:
    pub fn apply_oversample(&mut self, spec: &CookedSvfSpec, sample: Sample) -> Sample {
        let (g_prime, k) = (spec.get_g_prime(), spec.get_k());
        let oversample = (g_prime * TWO_OVER_PI).trunc() + 1.0;
        let coef = self.coef.apply(g_prime / oversample, k);
        let mut output = sample;
        for state in &mut self.state {
            let mut v = Default::default();
            for _ in 0..(oversample as u8) {
                v = state.apply(&coef, output);
            }
            output = self.output.get(v, k);
        }
        output
    }
}

/// this trait allows us to be generic over the sampling behavior
/// (either simple sampling or oversampling).
trait SvfSampling<O, const N: usize> {
    fn apply(&self, state: &mut SvfStages<O, N>, spec: &CookedSvfSpec, sample: Sample) -> Sample;
}

/// No-op sampling behavior (just uses samples as they come).
#[derive(Debug, Default)]
pub struct SimpleSampling;

impl<O, const N: usize> SvfSampling<O, N> for SimpleSampling
where
    O: SvfOutput,
{
    fn apply(&self, state: &mut SvfStages<O, N>, spec: &CookedSvfSpec, sample: Sample) -> Sample {
        state.apply(spec, sample)
    }
}

/// Start oversampling whenever cutoff frequency is greater than Nyquist.
#[derive(Debug, Default)]
pub struct Oversampling;

impl<O, const N: usize> SvfSampling<O, N> for Oversampling
where
    O: SvfOutput,
{
    fn apply(&self, state: &mut SvfStages<O, N>, spec: &CookedSvfSpec, sample: Sample) -> Sample {
        state.apply_oversample(spec, sample)
    }
}

#[cfg(test)]
mod test {
    use crate::clock::{Clock, SetClock};
    use crate::modulate::Modulated;
    use super::*;
    #[test]
    fn test_sizes() {
        use std::mem::size_of;
        assert_eq!(size_of::<Svf<LowPass>>(), 64);
        assert_eq!(size_of::<Svf2<LowPass>>(), 80);
    }
    #[test]
    fn test_svf() {
        let mut svf = Svf::<LowPass>::default();
        let mut spec = SvfSpec::default();

        spec.set_clock(&Clock::new(44_100.0));
        spec.set_cutoff(22_000.0);
        spec.set_res(0.5);

        assert_eq!(svf.proc(&spec.modulated(), 1.0), 0.9961729604271197);
        assert_eq!(svf.proc(&spec.modulated(), -1.0), -0.9885229931643646);
        assert_eq!(svf.proc(&spec.modulated(), 1.0), 0.9808814115655986);
        assert_eq!(svf.proc(&spec.modulated(), -1.0), -0.9732485379756293);

        let mut svf = Svf2::<LowPass>::default();
        assert_eq!(svf.proc(&spec.modulated(), 1.0), 0.9923605670861317);
        assert_eq!(svf.proc(&spec.modulated(), -1.0), -0.9771191860155131);
        assert_eq!(svf.proc(&spec.modulated(), 1.0), 0.9619530340874576);
        assert_eq!(svf.proc(&spec.modulated(), -1.0), -0.9468626252242185);
    }
}
