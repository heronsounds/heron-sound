//! Utils for accumulating modulator values from multiple sources.

use crate::Scale;

pub trait ModAccumulator {
    fn start_val(&self) -> Scale;
    fn acc(&self, acc: Scale, mod_val: Scale, mult: Scale) -> Scale;
}

// TODO call it "MultiplyAccumulator" or such?
// also, the basic logic is useful in other places too.
#[derive(Debug, Default)]
pub struct EnvAccumulator;

impl ModAccumulator for EnvAccumulator {
    fn start_val(&self) -> Scale {
        1.0
    }

    fn acc(&self, acc: Scale, mod_val: Scale, mult: Scale) -> Scale {
        acc * (mult * mod_val + (1.0 - mult))
    }
}

#[derive(Debug, Default)]
pub struct LfoAccumulator;

impl ModAccumulator for LfoAccumulator {
    fn start_val(&self) -> Scale {
        0.0
    }

    fn acc(&self, acc: Scale, mod_val: Scale, mult: Scale) -> Scale {
        acc + mod_val * mult
    }
}