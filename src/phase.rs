//! Phasor implementation.

use crate::Phase;

/// Keeps track of current phase, for e.g. an oscillator's waveform.
#[derive(Debug, Clone, Copy, Default)]
pub struct Phasor {
    current: Phase,
}

impl Phasor {
    #[cfg(test)]
    pub fn new(phase: Phase) -> Self {
        Self { current: phase }
    }

    pub fn reset(&mut self) {
        self.current = 0.0;
    }

    pub fn set(&mut self, phase: Phase) {
        crate::check_phase_bounds!(phase);
        self.current = phase;
    }

    pub fn peek(&self) -> Phase {
        self.current
    }

    pub fn advance(&mut self, dphase: Phase) -> Phase {
        let current = self.current;
        self.current = (current + dphase) % 1.0;
        current
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_size() {
        assert_eq!(std::mem::size_of::<Phasor>(), 8);
    }
    #[test]
    fn test_reset() {
        let mut phasor = Phasor::new(0.7);
        phasor.reset();
        assert_eq!(phasor.peek(), 0.0);
    }
    #[test]
    fn test_set() {
        let mut phasor = Phasor::new(0.2);
        phasor.set(0.5);
        assert_eq!(phasor.peek(), 0.5);
    }
    #[test]
    fn test_peek() {
        let mut phasor = Phasor::new(0.4);
        phasor.advance(0.1);
        assert_eq!(phasor.peek(), 0.5);
    }
    #[test]
    fn test_advance() {
        let mut phasor = Phasor::default();
        assert_eq!(phasor.advance(0.1), 0.0);
        assert_eq!(phasor.advance(0.3), 0.1);
        assert_eq!(phasor.advance(0.8), 0.4);
        // make sure we wrap around:
        assert!(phasor.advance(0.1) < 1.0);

        // let's test the behavior when phase gets to exactly 1.0:
        phasor.reset();
        phasor.advance(1.0);
        assert_eq!(phasor.peek(), 0.0);
    }
    #[test]
    #[should_panic]
    fn test_set_bounds_greater_than_one() {
        let mut phasor = Phasor::default();
        phasor.set(2.0);
    }
    #[test]
    #[should_panic]
    fn test_set_bounds_negative() {
        let mut phasor = Phasor::default();
        phasor.set(-1.0);
    }
}
