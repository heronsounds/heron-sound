use crate::{Seconds, Gen, F, Phase};
use crate::clock::{Clock, SetClock};

/// Current state of a pitch glide.
#[derive(Debug)]
pub enum GlideState {
    Init,
    Steady(Phase),
    Glide(Glide),
}

impl Default for GlideState {
    fn default() -> Self {
        Self::Init
    }
}

impl Gen<Phase> for GlideState {
    type Spec = GlideSpec;
    fn gen(&mut self, spec: &Self::Spec) -> Phase {
        match self {
            Self::Steady(pitch) => *pitch,
            Self::Glide(glide) => {
                if glide.is_crossed() {
                    let output = glide.tgt;
                    *self = Self::Steady(output);
                    output
                } else {
                    let output = glide.current;
                    glide.current += glide.diff * spec.tick_over_time;
                    output
                }
            },
            Self::Init => unimplemented!("Getting pitch for voice in Init state"),
        }
    }
}

impl GlideState {
    pub fn update(&mut self, spec: &GlideSpec, from_silence: bool, tgt_dphase: Phase) {
        crate::check_float_pos!(tgt_dphase);
        if spec.should_glide(from_silence) {
            self.update_glide_to(tgt_dphase);
        } else {
            *self = Self::Steady(tgt_dphase);
        }
    }

    fn update_glide_to(&mut self, tgt_dphase: Phase) {
        match self {
            Self::Steady(pitch) => *self = Self::Glide(Glide::new(*pitch, tgt_dphase)),
            Self::Glide(glide) => {
                glide.tgt = tgt_dphase;
                glide.diff = glide.tgt - glide.current;
            },
            // can't glide from init, so we just start steady:
            Self::Init => *self = Self::Steady(tgt_dphase),
        }
    }
}

#[derive(Debug)]
pub struct Glide {
    current: Phase,
    tgt: Phase,
    diff: Phase,
}

impl Glide {
    fn new(src: Phase, tgt: Phase) -> Self {
        Self {
            current: src,
            tgt,
            diff: tgt - src,
        }
    }

    fn is_crossed(&self) -> bool {
        if self.diff > 0.0 {
            self.current > self.tgt
        } else {
            self.current < self.tgt
        }
    }
}

/// Spec for [GlideState].
///
/// Controls glide behavior, including glide time and what to do when a note is played
/// from silence.
#[derive(Debug, Default)]
pub struct GlideSpec {
    tick: Seconds,
    time: Seconds,
    tick_over_time: F,
    glide_from_silence: bool,
}

impl GlideSpec {
    pub fn get_time(&self) -> Seconds {
        self.time
    }

    pub fn set_time(&mut self, time: Seconds) {
        self.time = time;
        self.compute_tick_over_time();
    }

    fn should_glide(&self, from_silence: bool) -> bool {
        self.time > 0.0 && (self.glide_from_silence || !from_silence)
    }

    fn compute_tick_over_time(&mut self) {
        // time value of zero indicates we should not glide.
        crate::check_float_nonneg!(self.time);
        if self.time > 0.0 {
            self.tick_over_time = self.tick / self.time;
        } else {
            self.tick_over_time = 0.0;
        }
    }

    crate::accessors!(glide_from_silence, get_glide_from_silence, set_glide_from_silence, bool);
}

impl SetClock for GlideSpec {
    fn set_clock(&mut self, clock: &Clock) {
        self.tick = clock.tick;
        self.compute_tick_over_time();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_size() {
        use std::mem::size_of;
        assert_eq!(size_of::<GlideState>(), 32);
        assert_eq!(size_of::<GlideSpec>(), 32);
    }
    #[test]
    fn test_glide() {
        let mut glide = GlideState::default();
        let mut spec = GlideSpec::default();
        spec.set_clock(&Clock::new(44_100.0));
        // test no glide:
        glide.update(&spec, false, 0.2);
        assert_eq!(glide.gen(&spec), 0.2);
        // test w/ glide:
        spec.set_time(crate::SHORT_TIME);
        assert_eq!(glide.gen(&spec), 0.2);
        glide.update(&spec, false, 0.3);
        assert_eq!(glide.gen(&spec), 0.2);
        assert_eq!(glide.gen(&spec), 0.2453514739229025);
        assert_eq!(glide.gen(&spec), 0.29070294784580497);
        assert_eq!(glide.gen(&spec), 0.3);
        // now glide down:
        glide.update(&spec, false, 0.01);
        assert_eq!(glide.gen(&spec), 0.3);
        assert_eq!(glide.gen(&spec), 0.16848072562358277);
        assert_eq!(glide.gen(&spec), 0.036961451247165544);
        assert_eq!(glide.gen(&spec), 0.01);
        // now go from silence:
        glide.update(&spec, true, 0.4);
        assert_eq!(glide.gen(&spec), 0.4);
        // now try from silence with glide:
        spec.set_glide_from_silence(true);
        glide.update(&spec, true, 0.2);
        assert_eq!(glide.gen(&spec), 0.4);
        assert_eq!(glide.gen(&spec), 0.309297052154195);
        // if we turn off glide, the current glide should continue...
        spec.set_time(0.0);
        assert_eq!(glide.gen(&spec), 0.21859410430839002);
        // but whoops, it doesn't. TODO fix that!
        assert_eq!(glide.gen(&spec), 0.21859410430839002);
        assert_eq!(glide.gen(&spec), 0.21859410430839002);
        // anyway, if we update again, it'll just jump straight to the new note:
        glide.update(&spec, false, 0.3);
        assert_eq!(glide.gen(&spec), 0.3);
    }
}
