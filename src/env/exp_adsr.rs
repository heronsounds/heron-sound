use crate::{Scale, Hz, Gen, F};
use crate::clock::{Clock, SetClock};

use super::{adsr::AdsrStage, ExpTimeStage, HoldRelease};

/// Envelope generator for ADSR envs with exponential curved segments.
#[derive(Debug, Default)]
pub struct ExpAdsr {
    stage: AdsrStage,
    next_val: Scale,
}

impl ExpAdsr {
    pub fn set_stage(&mut self, stage: AdsrStage) {
        if self.next_val < 0.0 {
            self.next_val = 0.0;
        }
        self.stage = stage;
    }
    pub fn stage(&self) -> AdsrStage {
        self.stage
    }
    pub fn finished(&self) -> bool {
        self.next_val < 0.0
    }
    fn compute_current(&mut self, stage: &ExpTimeStage) -> Scale {
        let current_val = self.next_val;
        self.next_val = stage.base() + self.next_val * stage.coef();
        current_val
    }
    fn transition(&mut self, stage: AdsrStage, val: Scale) -> Scale {
        self.stage = stage;
        self.next_val = val;
        val
    }
}

impl HoldRelease for ExpAdsr {
    fn hold(&mut self) {
        self.set_stage(AdsrStage::A);
    }
    fn release(&mut self) {
        self.set_stage(AdsrStage::R);
    }
    fn sustain(&mut self) {
        self.set_stage(AdsrStage::S);
    }
}

impl Gen<Scale> for ExpAdsr {
    type Spec = ExpAdsrSpec;
    fn gen(&mut self, spec: &Self::Spec) -> Scale {
        use AdsrStage::*;
        match self.stage {
            S => self.next_val,
            R => {
                if self.next_val < 0.0 {
                    0.0
                } else {
                    self.compute_current(&spec.r)
                }
            },
            A => {
                if self.next_val < spec.s {
                    self.compute_current(&spec.a)
                } else if spec.d.time() > 0.0 {
                    if self.next_val < 1.0 {
                        self.compute_current(&spec.a)
                    } else {
                        self.transition(D, 1.0)
                    }
                } else {
                    self.transition(S, spec.s)
                }
            },
            D => {
                if self.next_val > spec.s {
                    self.compute_current(&spec.d)
                } else {
                    self.transition(S, spec.s)
                }
            },
        }
    }
}

/// Spec for [ExpAdsr].
///
/// Hardcodes sensible curves under the hood
/// based on a subjective opinion of what sounds good.
/// We could make those constants modifiable in the future if necessary.
#[derive(Debug)]
pub struct ExpAdsrSpec {
    a: ExpTimeStage,
    d: ExpTimeStage,
    s: Scale,
    r: ExpTimeStage,
    sample_rate: Hz,
}

impl Default for ExpAdsrSpec {
    fn default() -> Self {
        Self {
            a: ExpTimeStage::rising(1.0),
            d: ExpTimeStage::falling(1.0),
            s: 1.0,
            r: ExpTimeStage::falling(0.0),
            sample_rate: 0.0,
        }
    }
}

impl SetClock for ExpAdsrSpec {
    fn set_clock(&mut self, clock: &Clock) {
        self.sample_rate = clock.sample_rate;
        self.a.set_sample_rate(self.sample_rate);
        self.d.set_sample_rate(self.sample_rate);
        self.r.set_sample_rate(self.sample_rate);
    }
}

impl ExpAdsrSpec {
    pub fn get(&self, stage: AdsrStage) -> F {
        use AdsrStage::*;
        match stage {
            A => self.a.time(),
            D => self.d.time(),
            S => self.s,
            R => self.r.time(),
        }
    }

    pub fn set(&mut self, stage: AdsrStage, val: F) {
        use AdsrStage::*;
        match stage {
            A => self.a.set_time(self.sample_rate, val),
            D => self.d.set_time(self.sample_rate, val),
            S => {
                crate::check_float_01!(val);
                self.s = val;
                self.d.set_peak(self.s);
            },
            R => self.r.set_time(self.sample_rate, val),
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;
    fn spec(a: F, d: F, s: F, r: F) -> ExpAdsrSpec {
        use AdsrStage::*;
        let mut spec = ExpAdsrSpec::default();
        spec.set(A, a);
        spec.set(D, d);
        spec.set(S, s);
        spec.set(R, r);
        spec.set_clock(&Clock::new(44100.0));
        spec
    }
    #[test]
    fn test_size() {
        use std::mem::size_of;
        assert_eq!(size_of::<ExpAdsr>(), 16);
        assert_eq!(size_of::<ExpAdsrSpec>(), 160);
    }
    #[test]
    fn a_reaches_1() {
        let mut adsr = ExpAdsr::default();
        let spec = spec(1.0, 0.0, 1.0, 0.0);
        let output = adsr.gen(&spec);
        assert!(output < 1.0);
        for _ in 0..44100 {
            adsr.gen(&spec);
        }
        assert_eq!(adsr.gen(&spec), 1.0);
    }
    #[test]
    fn d_reaches_s() {
        let mut adsr = ExpAdsr { stage: AdsrStage::D, next_val: 1.0 };
        let spec = spec(0.0, 1.0, 0.5, 0.0);
        let mut output = adsr.gen(&spec);
        assert_eq!(output, 1.0);
        for _ in 0..44100 {
            output = adsr.gen(&spec);
        }
        assert_eq!(output, 0.5);
    }
    #[test]
    fn r_reaches_0() {
        let mut adsr = ExpAdsr { stage: AdsrStage::R, next_val: 0.5 };
        let spec = spec(0.0, 0.0, 0.5, 1.0);
        let mut output = adsr.gen(&spec);
        assert!(output > 0.0);
        for _ in 0..44100 {
            output = adsr.gen(&spec);
        }
        assert!(output == 0.0);
    }
}
