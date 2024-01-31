use crate::Phase;

/// Cache the last-used pitch so we don't have to compute it again if it remains steady.
///
/// Use this with e.g. a modulatable oscillator, so it'll recompute the pitch if pitch
/// modulation is active, but avoid doing so if modulation is inactive.
///
/// NB: as with most of the components in this library, this struct expects phase offsets
/// rather than pitches (though in this case the math is the same either way).
#[derive(Debug, Default)]
pub struct PitchCache {
    last_input_pitch: f64,
    last_octave_offset: f64,
    last_semis_offset: f64,
    last_env_amt: f64,
    last_lfo_amt: f64,

    last_output_pitch: f64,
}

impl PitchCache {
    pub fn process(
        &mut self,
        dphase: Phase,
        octave_offset: f64,
        semis_offset: f64,
        env_amt: f64,
        lfo_amt: f64,
    ) -> Phase {
        if dphase != self.last_input_pitch
            || lfo_amt != self.last_lfo_amt
            || env_amt != self.last_env_amt
            || semis_offset != self.last_semis_offset
            || octave_offset != self.last_octave_offset
        {
            self.last_input_pitch = dphase;
            self.last_octave_offset = octave_offset;
            self.last_semis_offset = semis_offset;
            self.last_env_amt = env_amt;
            self.last_lfo_amt = lfo_amt;

            let offset = octave_offset + env_amt + semis_offset + lfo_amt;
            self.last_output_pitch = if offset == 0.0 {
                dphase
            } else {
                super::apply_offset(dphase, offset)
            };
        }

        self.last_output_pitch
    }

    pub fn get_cached_dphase(&self) -> Phase {
        self.last_output_pitch
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_pitch_cache() {
        assert_eq!(std::mem::size_of::<PitchCache>(), 48);
        let mut cache = PitchCache::default();
        let output = cache.process(
            0.1,
            1.0,
            0.01,
            0.5,
            0.2,
        );
        assert_eq!(output, 0.11038162273110462);
        assert_eq!(cache.get_cached_dphase(), 0.11038162273110462);

        let output = cache.process(
            0.1,
            1.0,
            0.01,
            0.5,
            0.2,
        );
        assert_eq!(output, 0.11038162273110462);
        assert_eq!(cache.get_cached_dphase(), 0.11038162273110462);

        let output = cache.process(
            0.1,
            1.0,
            0.01,
            0.2,
            0.3,
        );
        assert_eq!(output, 0.10911378165900086);
        assert_eq!(cache.get_cached_dphase(), 0.10911378165900086);
    }
}
