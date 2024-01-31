//! Sample rate utilities.

use crate::{Seconds, Hz};

/// Stores the current sample rate.
///
/// Ideally, we want to have only one of these per plugin instance.
#[derive(Debug, Clone, Copy)]
pub struct Clock {
    pub sample_rate: Hz,
    pub tick: Seconds,
}

impl Clock {
    pub fn new(sample_rate: Hz) -> Self {
        crate::check_hz_bounds!(sample_rate);
        Self {
            sample_rate,
            tick: 1.0 / sample_rate,
        }
    }

    pub fn nyquist(&self) -> f64 {
        self.sample_rate * 0.5
    }
}

/// Trait for components that need to know current sample rate.
pub trait SetClock {
    fn set_clock(&mut self, clock: &Clock);
}

impl<T: SetClock, const N: usize> SetClock for [T; N] {
    fn set_clock(&mut self, clock: &Clock) {
        for val in self {
            val.set_clock(clock);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_clock() {
        assert_eq!(std::mem::size_of::<Clock>(), 16);
        let clock = Clock::new(44_100.0);
        assert_eq!(clock.sample_rate, 44_100.0);
        assert_eq!(clock.tick, 1.0 / 44_100.0);
        assert_eq!(clock.nyquist(), 44_100.0 * 0.5);
    }
    #[test]
    #[should_panic]
    fn test_bounds_zero() {
        Clock::new(0.0);
    }
    #[test]
    #[should_panic]
    fn test_bounds_neg() {
        Clock::new(-1.0);
    }
}
