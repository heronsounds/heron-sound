use core::f64::consts::PI;

use crate::{F, Hz, Sample, Seconds};
use crate::clock::{Clock, SetClock};
use crate::modulate::Modulated;

use super::VFactors;

pub type CoefVals = [F; 3];

#[derive(Debug, Default, Clone, Copy)]
pub struct SvfState {
    hist: [F; 2],
}

impl SvfState {
    pub fn apply(&mut self, c: &CoefVals, v0: Sample) -> VFactors {
        let h = &self.hist;

        let v3 = v0 - h[1];
        let v1 = c[0] * h[0] + c[1] * v3;
        let v2 = h[1] + c[1] * h[0] + c[2] * v3;

        self.hist[0] = 2.0 * v1 - self.hist[0];
        self.hist[1] = 2.0 * v2 - self.hist[1];

        [v0, v1, v2]
    }
}

#[derive(Debug, Default)]
pub struct SvfCoef {
    coef: CoefVals,
    /// g_prime = cutoff * pi * tick
    g_prime: F,
    g: F,
    k: F,
}

impl SvfCoef {
    fn compute(&mut self) {
        self.coef[0] = 1.0 / (1.0 + self.g * (self.g + self.k));
        self.coef[1] = self.g * self.coef[0];
        self.coef[2] = self.g * self.coef[1];
    }

    pub fn apply(&mut self, g_prime: F, k: F) -> CoefVals {
        if self.g_prime != g_prime {
            self.g_prime = g_prime;
            self.g = self.g_prime.tan();
            self.k = k;
            self.compute();
        } else if self.k != k {
            self.k = k;
            self.compute();
        }
        self.coef
    }
}

/// Type produced by [SvfSpec]'s `Modulated` impl.
///
/// This is the type actually used by our Svf implementations.
#[derive(Debug, Default, Clone)]
pub struct CookedSvfSpec {
    pi_tick: F,
    g_prime: F,
    k: F,
}

// TODO gonna have to add a set_q and store q here too...
impl CookedSvfSpec {
    pub fn get_k(&self) -> F {
        self.k
    }

    pub fn get_g_prime(&self) -> F {
        self.g_prime
    }

    pub fn set_cutoff(&mut self, cutoff: Hz) {
        self.g_prime = self.pi_tick * cutoff;
    }
}

/// Spec of a state-variable filter.
#[derive(Debug, Default)]
pub struct SvfSpec {
    pi_tick: Seconds,
    cutoff: Hz,
    /// g_prime = cutoff * pi * tick
    g_prime: F,
    q: F,
    k: F,
}

impl SetClock for SvfSpec {
    fn set_clock(&mut self, clock: &Clock) {
        self.pi_tick = PI * clock.tick;
        self.compute_g_prime();
        self.compute_k();
    }
}

impl SvfSpec {
    pub fn get_cutoff(&self) -> Hz {
        self.cutoff
    }

    pub fn set_cutoff(&mut self, cutoff: Hz) {
        crate::check_float_nonneg!(cutoff);
        self.cutoff = cutoff;
        self.compute_g_prime();
    }

    pub fn get_res(&self) -> F {
        self.q
    }

    pub fn set_res(&mut self, res: F) {
        crate::check_float_01!(res);
        self.q = res;
        self.compute_k();
    }

    fn compute_g_prime(&mut self) {
        self.g_prime = self.cutoff * self.pi_tick;
    }

    fn compute_k(&mut self) {
        // we found that doubling the q scale feels best in use:
        self.k = 2.0 - 1.85 * self.q;
    }
}

impl Modulated for SvfSpec {
    type Child = CookedSvfSpec;
    fn modulated(&self) -> Self::Child {
        CookedSvfSpec {
            pi_tick: self.pi_tick,
            g_prime: self.g_prime,
            k: self.k,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_sizes() {
        use std::mem::size_of;
        assert_eq!(size_of::<SvfSpec>(), 40);
        assert_eq!(size_of::<SvfCoef>(), 48);
        assert_eq!(size_of::<SvfState>(), 16);
    }
    #[test]
    fn test_svf_core() {
        let mut spec = SvfSpec::default();
        let mut coef = SvfCoef::default();
        let mut state = SvfState::default();

        spec.set_clock(&Clock::new(44_100.0));
        spec.set_cutoff(22_000.0);
        spec.set_res(0.5);

        let coef_vals = coef.apply(spec.g_prime, spec.k);
        assert_eq!(coef_vals, [1.2638659037921946e-5, 0.0035482799198535164, 0.9961729604271197]);

        assert_eq!(state.apply(&coef_vals, 1.0), [1.0, 0.0035482799198535164, 0.9961729604271197]);
        assert_eq!(state.apply(&coef_vals, -1.0), [-1.0, -0.01061759125322252, -0.9885229931643646]);
        assert_eq!(state.apply(&coef_vals, 1.0), [1.0, 0.017632435442908434, 0.9808814115655986]);
        assert_eq!(state.apply(&coef_vals, -1.0), [-1.0, -0.024592873375060866, -0.9732485379756293]);
    }
}
