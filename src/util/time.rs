use crate::{Seconds, Hz, clock::{Clock, SetClock}};

fn compute_time_samples(time: Seconds, sample_rate: Hz) -> usize {
    (time * sample_rate).floor() as usize
}

/// A value representing a time in seconds.
///
/// Caches the number of samples so we don't have to compute it during processing time.
#[derive(Debug, Default)]
pub struct Time {
    time: Seconds,
    sample_rate: Hz,
    time_samples: usize,
}

impl Time {
    pub fn set(&mut self, time: Seconds) {
        crate::check_float_nonneg!(time);
        self.time = time;
        self.time_samples = compute_time_samples(self.time, self.sample_rate);
    }

    pub fn get(&self) -> Seconds {
        self.time
    }

    pub fn get_samples(&self) -> usize {
        self.time_samples
    }
}

impl SetClock for Time {
    fn set_clock(&mut self, clock: &Clock) {
        self.sample_rate = clock.sample_rate;
        self.time_samples = compute_time_samples(self.time, self.sample_rate);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_size() {
        assert_eq!(std::mem::size_of::<Time>(), 24);
    }
    #[test]
    fn test_time() {
        let mut time = Time::default();
        assert_eq!(time.get(), 0.0);
        assert_eq!(time.get_samples(), 0);

        time.set(1.0);
        assert_eq!(time.get(), 1.0);
        assert_eq!(time.get_samples(), 0);

        time.set_clock(&Clock::new(44_100.0));
        assert_eq!(time.get(), 1.0);
        assert_eq!(time.get_samples(), 44_100);

        time.set_clock(&Clock::new(48_000.0));
        assert_eq!(time.get(), 1.0);
        assert_eq!(time.get_samples(), 48_000);

        time.set(2.0);
        assert_eq!(time.get(), 2.0);
        assert_eq!(time.get_samples(), 96_000);
    }
}
