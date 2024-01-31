use crate::{Seconds, Scale, F, Hz};

/// Represents one of the time stages in an env generator (e.g. A, D, or R).
///
/// Stores both the time in seconds, and tick over time (1/time * 1/rate) to avoid
/// calculating it at processing time.
#[derive(Debug, Default)]
pub struct TimeStage {
    time: Seconds,
    tick_over_time: Scale,
}

impl TimeStage {
    pub fn set(&mut self, tick: Seconds, time: Seconds) {
        self.time = time;
        self.set_tick(tick);
    }

    pub fn set_tick(&mut self, tick: Seconds) {
        crate::check_float_nonneg!(self.time);
        if self.time > 0.0 {
            self.tick_over_time = tick / self.time;
        } else {
            // set to 1.0 so we immediately progress past the threshold:
            self.tick_over_time = 1.0;
        }
    }

    pub fn time(&self) -> Seconds {
        self.time
    }

    pub fn tick_over_time(&self) -> Scale {
        self.tick_over_time
    }
}

// const OFFSET_RISE: f64 = (-1.5f64).exp();
const OFFSET_RISE: F = 0.22313016014;
// const OFFSET_FALL: f64 = (-4.95f64).exp();
const OFFSET_FALL: F = 0.00708340892;

/// Represents one of the time stages of an *exponential* env generator (e.g. A, D, or R).
///
/// Stores the time in seconds, as well as cached values required to iteratively compute
/// an exponential curve.
///
/// NB no default here.
#[derive(Debug)]
pub struct ExpTimeStage {
    time: Seconds,

    offset: F,
    peak: Scale,
    // TODO cd store as i8? or bool?
    dir: F,

    base: F,
    coef: F,
}

impl ExpTimeStage {

    pub fn rising(peak: Scale) -> Self {
        Self::new(0.0, 0.0, OFFSET_RISE, peak, 1.0)
    }

    pub fn falling(peak: Scale) -> Self {
        Self::new(0.0, 0.0, OFFSET_FALL, peak, -1.0)
    }

    fn new(time: Seconds, sample_rate: Hz, offset: F, peak: Scale, dir: F) -> Self {
        let mut val = Self { time, offset, peak, dir, base: 0.0, coef: 0.0, };
        val.compute(time * sample_rate);
        val
    }

    fn compute(&mut self, rate: F) {
        self.compute_coef(rate);
        self.compute_base();
    }

    fn compute_coef(&mut self, rate: F) {
        if rate > 0.0 {
            self.coef = ((-((1.0 + self.offset) / self.offset).ln()) / rate).exp()
        } else {
            self.coef = 0.0;
        }
    }

    fn compute_base(&mut self) {
        self.base = (self.peak + self.dir * self.offset) * (1.0 - self.coef);
    }

    pub fn time(&self) -> Seconds {
        self.time
    }

    pub fn base(&self) -> F {
        self.base
    }

    pub fn coef(&self) -> F {
        self.coef
    }

    pub fn set_time(&mut self, sample_rate: Hz, time: Seconds) {
        crate::check_float_nonneg!(time);
        self.time = time;
        self.compute(self.time * sample_rate);
    }

    pub fn set_sample_rate(&mut self, sample_rate: Hz) {
        self.compute(self.time * sample_rate);
    }

    pub fn set_peak(&mut self, peak: Scale) {
        crate::check_float_01!(peak);
        self.peak = peak;
        self.compute_base();
    }
}
