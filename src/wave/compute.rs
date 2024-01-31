/// Raw, i.e. naive, i.e. aliasing waveforms
pub mod raw {
    use crate::{Phase, Scale, Sample};

    pub fn tri(phase: Phase) -> Sample {
        crate::check_phase_bounds!(phase);
        if phase < 0.5 {
            phase * 4.0 - 1.0
        } else {
            -4.0 * phase + 3.0
        }
    }

    pub fn pulse(phase: Phase, width: Scale) -> Sample {
        crate::check_phase_bounds!(phase);
        crate::check_phase_bounds!(width);
        if phase < width { 1.0 } else { -1.0 }
    }

    pub fn saw(phase: Phase) -> Sample {
        crate::check_phase_bounds!(phase);
        phase * 2.0 - 1.0
    }

    pub fn sine(phase: Phase) -> Sample {
        crate::check_phase_bounds!(phase);
        const PI_2: f64 = core::f64::consts::PI * 2.0;
        f64::sin(PI_2 * phase)
    }
}

/// Polyblep, i.e. non-aliasing waveforms
/// (Tri wave is still naive; working on that).
/// We don't include a sine implementation b/c sine has no need for anti-aliasing;
/// callers should always prefer the raw version.
pub mod polyblep {
    use crate::{Phase, Scale, Sample};

    pub fn tri(phase: Phase, _dphase: Phase) -> Sample {
        super::raw::tri(phase)
    }

    pub fn pulse(phase: Phase, dphase: Phase, width: Scale) -> Sample {
        super::raw::pulse(phase, width)
            + polyblep(phase, dphase)
            - polyblep((phase + 1.0 - width) % 1.0, dphase)
    }

    pub fn saw(phase: Phase, dphase: Phase) -> Sample {
        super::raw::saw(phase) - polyblep(phase, dphase)
    }

    // dphase = freq / sample_rate i.e. the oscillator inc value?
    // the idea is to subtract this value from the naive oscillator output
    // and it should work for saw waves just like that.
    // but for square waves, they do this:
    // value = naive_output;
    // value += polyblep(phase, dphase);
    // value -= polyblep(phase, fmod(phase + 0.5, 1.0));
    fn polyblep(mut phase: Scale, dphase: Phase) -> Sample {
        crate::check_float_pos!(dphase);
        if phase < dphase {
            phase /= dphase;
            phase + phase - phase * phase - 1.0
        } else if phase > 1.0 - dphase {
            phase = (phase - 1.0) / dphase;
            phase * phase + phase + phase + 1.0
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod test {
    use super::{raw, polyblep};
    #[test]
    fn test_waves_compute() {
        // raw tri
        assert_eq!(raw::tri(0.0), -1.0);
        assert_eq!(raw::tri(0.25), 0.0);
        assert_eq!(raw::tri(0.5), 1.0);
        assert_eq!(raw::tri(0.75), 0.0);
        assert_eq!(raw::tri(0.99), -0.96);
        // polyblep tri currently just fwds to raw:
        assert_eq!(raw::tri(0.99), polyblep::tri(0.99, 0.1));
        // raw pulse
        assert_eq!(raw::pulse(0.0, 0.5), 1.0);
        assert_eq!(raw::pulse(0.25, 0.5), 1.0);
        assert_eq!(raw::pulse(0.5, 0.5), -1.0);
        assert_eq!(raw::pulse(0.75, 0.5), -1.0);
        assert_eq!(raw::pulse(0.4, 0.5), 1.0);
        assert_eq!(raw::pulse(0.4, 0.25), -1.0);
        // raw saw
        assert_eq!(raw::saw(0.0), -1.0);
        assert_eq!(raw::saw(0.5), 0.0);
        assert_eq!(raw::saw(0.9), 0.8);
        // raw sine
        assert_eq!(raw::sine(0.0), 0.0);
        assert_eq!(raw::sine(0.5), f64::sin(core::f64::consts::PI));

        // polyblep pulse //
        // raw, when close to pulsewidth:
        assert_eq!(raw::pulse(0.45, 0.5), 1.0);
        // polyblep out should be same as raw when more than dphase away from pulsewidth:
        assert_eq!(polyblep::pulse(0.45, 0.01, 0.5), 1.0);
        // should be different when less than dphase away from pulsewidth:
        assert_eq!(polyblep::pulse(0.45, 0.1, 0.5), 0.7500000000000004);
        // check same thing when close to zero:
        assert_eq!(raw::pulse(0.05, 0.5), 1.0);
        assert_eq!(polyblep::pulse(0.05, 0.01, 0.5), 1.0);
        assert_eq!(polyblep::pulse(0.05, 0.1, 0.5), 0.75);
        
        // polyblep saw //
        // close to zero:
        assert_eq!(raw::saw(0.05), -0.9);
        assert_eq!(polyblep::saw(0.05, 0.01), -0.9);
        assert_eq!(polyblep::saw(0.05, 0.1), -0.65);
        // close to one:
        assert_eq!(raw::saw(0.95), 0.8999999999999999);
        assert_eq!(polyblep::saw(0.95, 0.01), 0.8999999999999999);
        assert_eq!(polyblep::saw(0.95, 0.1), 0.6500000000000004);
    }
}
