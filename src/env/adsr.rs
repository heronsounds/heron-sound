use crate::{Scale, Seconds, Gen, F};
use crate::clock::{Clock, SetClock};

use super::{TimeStage, HoldRelease};

/// Marker for which stage of an ADSR env we're in.
///
/// Used by both [Adsr] and [super::ExpAdsr].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdsrStage {
    A, D, S, R,
}

impl Default for AdsrStage {
    fn default() -> Self { Self::A }
}

/// Envelope generator for ADSR envs with linear segments.
#[derive(Debug, Default)]
pub struct Adsr {
    stage: AdsrStage,
    next_val: Scale,
}

impl HoldRelease for Adsr {
    fn hold(&mut self) {
        self.stage = AdsrStage::A;
    }
    fn release(&mut self) {
        self.stage = AdsrStage::R;
    }
    fn sustain(&mut self) {
        self.stage = AdsrStage::S;
    }
}

impl Gen<Scale> for Adsr {
    type Spec = AdsrSpec;
    fn gen(&mut self, spec: &Self::Spec) -> Scale {
        use AdsrStage::*;
        let queued_val = self.next_val;
        match self.stage {
            S => queued_val,
            R => {
                if self.next_val < 0.0 {
                    0.0
                } else {
                    self.next_val -= spec.r.tick_over_time();
                    queued_val
                }
            },
            A => {
                if self.next_val > 1.0 {
                    self.stage = D;
                    self.next_val = 1.0 - spec.d.tick_over_time();
                    1.0
                } else {
                    self.next_val += spec.a.tick_over_time();
                    queued_val
                }
            },
            D => {
                if self.next_val < spec.s {
                    self.stage = S;
                    self.next_val = spec.s;
                    spec.s
                } else {
                    self.next_val -= spec.d.tick_over_time();
                    queued_val
                }
            },
        }
    }
}

/// Spec for [Adsr].
#[derive(Debug, Default)]
pub struct AdsrSpec {
    a: TimeStage,
    d: TimeStage,
    s: Scale,
    r: TimeStage,
    tick: Seconds,
}

impl SetClock for AdsrSpec {
    fn set_clock(&mut self, clock: &Clock) {
        self.tick = clock.tick;
        self.a.set_tick(self.tick);
        self.d.set_tick(self.tick);
        self.r.set_tick(self.tick);
    }
}

impl AdsrSpec {
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
            A => self.a.set(self.tick, val),
            D => self.d.set(self.tick, val),
            S => self.s = val,
            R => self.r.set(self.tick, val),
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;
    fn spec(a: F, d: F, s: F, r: F) -> AdsrSpec {
        use AdsrStage::*;
        let mut spec = AdsrSpec::default();
        spec.set(A, a);
        spec.set(D, d);
        spec.set(S, s);
        spec.set(R, r);
        spec.set_clock(&Clock::new(44100.0));
        spec
    }
    #[test]
    fn size() {
        use std::mem::size_of;
        assert_eq!(size_of::<Adsr>(), 16);
        assert_eq!(size_of::<AdsrSpec>(), 64);
    }
    #[test]
    fn a_reaches_1() {
        let mut adsr = Adsr::default();
        let spec = spec(1.0, 0.0, 1.0, 0.0);
        let mut output = adsr.gen(&spec);
        assert!(output < 1.0);
        for _ in 0..44100 {
            output = adsr.gen(&spec);
        }
        assert!(output == 1.0);
    }
    #[test]
    fn d_reaches_s() {
        let mut adsr = Adsr { stage: AdsrStage::D, next_val: 1.0, };
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
        let mut adsr = Adsr { stage: AdsrStage::R, next_val: 0.5, };
        let spec = spec(0.0, 0.0, 0.5, 1.0);
        let mut output = adsr.gen(&spec);
        assert!(output > 0.0);
        for _ in 0..44100 {
            output = adsr.gen(&spec);
        }
        assert_eq!(output, 0.0);
    }
}
