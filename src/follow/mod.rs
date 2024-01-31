//! Envelope follower implementations.

use crate::{F, Scale, Seconds, Sample, Proc};
use crate::clock::{Clock, SetClock};
use crate::util::Time;

/// Engages an envelope when input crosses an amplitude threshold.
#[derive(Debug, Default)]
pub struct EnvFollower {
    /// keeps track of how long (how many samples) we've been holding.
    count: usize,
    /// current status: are we holding bc we've crossed threshold?
    holding: bool,
}

impl EnvFollower {
    fn apply(&mut self, spec: &EnvFollowerSpec, val: Sample) -> bool {
        if val > spec.threshold {
            self.holding = true;
            self.count = 0;
            true
        } else if self.holding {
            // TODO we can get rid of holding if we start count at 1.
            // then if count == 0, we know we're not holding.
            if self.count < spec.hold.get_samples() {
                self.count += 1;
                true
            } else {
                self.holding = false;
                false
            }
        } else {
            false
        }
    }
}

/// Basic implementation
impl Proc<Sample, bool> for EnvFollower {
    type Spec = EnvFollowerSpec;
    fn proc(&mut self, spec: &Self::Spec, sample: Sample) -> bool {
        let val = sample.abs();
        self.apply(spec, val)
    }
}

/// Generic implementation for `N` channels; use for stereo sources.
impl<const N: usize> Proc<[Sample; N], bool> for EnvFollower {
    type Spec = EnvFollowerSpec;
    fn proc(&mut self, spec: &Self::Spec, input: [Sample; N]) -> bool {

        let mut max_val: F = 0.0;
        for val in input {
            max_val = max_val.max(val.abs());
        }

        self.apply(spec, max_val)
    }
}

/// Spec for [EnvFollower].
#[derive(Debug, Default)]
pub struct EnvFollowerSpec {
    /// amplitude at which we start holding
    threshold: Scale,
    /// how long we should hold for (specified in seconds, measured in samples)
    hold: Time,
}

impl EnvFollowerSpec {
    pub fn get_threshold(&self) -> Scale {
        self.threshold
    }

    pub fn set_threshold(&mut self, threshold: Scale) {
        self.threshold = threshold;
    }

    pub fn get_hold(&self) -> Seconds {
        self.hold.get()
    }

    pub fn set_hold(&mut self, hold: Seconds) {
        crate::check_float_nonneg!(hold);
        self.hold.set(hold);
    }
}

impl SetClock for EnvFollowerSpec {
    fn set_clock(&mut self, clock: &Clock) {
        self.hold.set_clock(clock);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_size() {
        use std::mem::size_of;
        assert_eq!(size_of::<EnvFollower>(), 16);
        assert_eq!(size_of::<EnvFollowerSpec>(), 32);
    }
    #[test]
    fn test_env_follower() {
        let mut spec = EnvFollowerSpec::default();
        spec.set_threshold(0.5);
        // 0.00005 conveniently works out to 2 samples at 44.1kHz:
        spec.set_hold(0.00005);
        spec.set_clock(&Clock::new(44_100.0));
        assert_eq!(spec.get_threshold(), 0.5);
        assert_eq!(spec.get_hold(), 0.00005);
        assert_eq!(spec.hold.get_samples(), 2);

        let mut state = EnvFollower::default();
        assert!(!state.proc(&spec, 0.4));
        assert!(!state.proc(&spec, 0.5));
        // should cross threshold, then hold for 2 samples:
        assert!(state.proc(&spec, 0.6));
        assert!(state.proc(&spec, 0.4));
        assert!(state.proc(&spec, 0.4));
        assert!(!state.proc(&spec, 0.4));
    }
}
