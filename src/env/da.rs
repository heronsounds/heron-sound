use crate::{Scale, Seconds, Gen, F};
use crate::clock::{Clock, SetClock};

use super::{TimeStage, HoldRelease};

#[derive(Debug)]
enum DaStage {
    // Delay, Attack, Sustain:
    D, A, S,
}

impl Default for DaStage {
    fn default() -> Self { Self::D }
}

/// Envelope generator for DA envs.
#[derive(Debug, Default)]
pub struct DaEnv {
    stage: DaStage,
    // this val measures either time or output scale, depending on stage:
    val: F,
}

impl HoldRelease for DaEnv {
    fn hold(&mut self) {
        self.stage = DaStage::D;
        self.val = 0.0;
    }
    fn release(&mut self) {
        self.stage = DaStage::S;
    }
    fn sustain(&mut self) {
        self.stage = DaStage::S;
    }
}

impl Gen<Scale> for DaEnv {
    type Spec = DaEnvSpec;
    fn gen(&mut self, spec: &Self::Spec) -> Scale {
        use DaStage::*;
        match self.stage {
            S => self.val,
            D => {
                if self.val > spec.d {
                    self.stage = A;
                    self.val = 0.0;
                } else {
                    self.val += spec.tick;
                }
                0.0
            },
            A => {
                if self.val > 1.0 {
                    self.stage = S;
                    self.val = 1.0;
                    1.0
                } else {
                    let val = self.val;
                    self.val += spec.a.tick_over_time();
                    val
                }
            },
        }
    }
}

/// Spec for [DaEnv].
///
/// The interface only allows setting the total time, not D and A individually.
/// D is hardcoded to be `2/3` of the total, and A `1/3`.
#[derive(Debug, Default)]
pub struct DaEnvSpec {
    d: Seconds,
    a: TimeStage,
    tick: Seconds,
}

impl SetClock for DaEnvSpec {
    fn set_clock(&mut self, clock: &Clock) {
        self.tick = clock.tick;
        self.a.set_tick(self.tick);
    }
}

// TODO or set D and A independently.
impl DaEnvSpec {
    pub fn get_total(&self) -> Seconds {
        self.d + self.a.time()
    }

    pub fn set_total(&mut self, total: Seconds) {
        crate::check_float_nonneg!(total);
        self.d = total * 0.66;
        self.a.set(self.tick, total * 0.34);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_sizes() {
        use std::mem::size_of;
        assert_eq!(size_of::<DaEnv>(), 16);
        assert_eq!(size_of::<DaEnvSpec>(), 32);
    }
    #[test]
    fn test_da() {
        let mut env = DaEnv::default();
        let mut spec = DaEnvSpec::default();
        spec.set_clock(&Clock::new(44_100.0));
        spec.set_total(0.0001);
        assert_eq!(env.gen(&spec), 0.0);
        assert_eq!(env.gen(&spec), 0.0);
        assert_eq!(env.gen(&spec), 0.0);
        assert_eq!(env.gen(&spec), 0.0);
        assert_eq!(env.gen(&spec), 0.0);
        assert_eq!(env.gen(&spec), 0.6669334400426836);
        assert_eq!(env.gen(&spec), 1.0);
        assert_eq!(env.gen(&spec), 1.0);
        env.release();
        assert_eq!(env.gen(&spec), 1.0);
        env.hold();
        assert_eq!(env.gen(&spec), 0.0);

        // check when we're set to 0:
        spec.set_total(0.0);
        env.hold();
        // current implementation allows for 3 samples of 0 before jumping to 1.
        // this is negligible, but we should fix it eventually.
        assert_eq!(env.gen(&spec), 0.0);
        assert_eq!(env.gen(&spec), 0.0);
        assert_eq!(env.gen(&spec), 0.0);
        assert_eq!(env.gen(&spec), 1.0);
    }
}
