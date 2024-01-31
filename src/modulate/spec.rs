use crate::Scale;

use super::accumulate::{ModAccumulator, EnvAccumulator, LfoAccumulator};

/// Spec controlling the intensities of an array of env generators.
pub type EnvArraySpec<const M: usize, const N: usize> =
    ModArraySpec<M, N, EnvAccumulator>;
/// Spec controlling the intensities of an array of Lfos.
pub type LfoArraySpec<const M: usize, const N: usize> =
    ModArraySpec<M, N, LfoAccumulator>;

/// Spec for parameter modulation.
///
/// - `M` is the number of modulators (of the same type)
/// - `N` is the number of output values
/// - `T` is the type of accumulation.
// TODO add invert, and some sort of bipolar switch... call it dc? or just offset?
#[derive(Debug)]
pub struct ModArraySpec<const M: usize, const N: usize, T> {
    intensities: [[Scale; M]; N],
    accumulator: T,
}

impl<const M: usize, const N: usize, T: Default> Default for ModArraySpec<M, N, T> {
    fn default() -> Self {
        Self {
            intensities: [[0.0; M]; N],
            accumulator: T::default(),
        }
    }
}

impl <const M: usize, const N: usize, T> ModArraySpec<M, N, T> {
    fn apply_intensities_acc<U: ModAccumulator>(&self, vals: [Scale; M], acc: &U) -> [Scale; N] {
        let mut output = [acc.start_val(); N];
        for (mod_idx, &mod_out) in vals.iter().enumerate() {
            for (out_idx, out) in output.iter_mut().enumerate() {
                let multiplier = self.intensities[out_idx][mod_idx];
                *out = acc.acc(*out, mod_out, multiplier);
            }
        }
        output
    }
}

impl<const M: usize, const N: usize, T: ModAccumulator> ModArraySpec<M, N, T> {
    pub fn apply_intensities(&self, modulator_vals: [Scale; M]) -> [Scale; N] {
        self.apply_intensities_acc(modulator_vals, &self.accumulator)
    }

    pub fn apply_intensities_add(&self, vals: [Scale; M]) -> [Scale; N] {
        self.apply_intensities_acc(vals, &LfoAccumulator::default())
    }

    pub fn apply_intensities_mul(&self, vals: [Scale; M]) -> [Scale; N] {
        self.apply_intensities_acc(vals, &EnvAccumulator::default())
    }

    // TODO add accessors that take m and n args:
    pub fn get_intensities(&self) -> &[[Scale; M]; N] {
        &self.intensities
    }

    pub fn get_intensities_mut(&mut self) -> &mut [[Scale; M]; N] {
        &mut self.intensities
    }
}

// shortcut when N==1.
impl<const M: usize, T: ModAccumulator> ModArraySpec<M, 1, T> {
    pub fn apply_intensity(&self, modulator_vals: [Scale; M]) -> Scale {
        self.apply_intensities(modulator_vals)[0]
    }

    pub fn get_intensity(&self, mod_idx: usize) -> Scale {
        self.intensities[0][mod_idx]
    }

    pub fn set_intensity(&mut self, mod_idx: usize, val: Scale) {
        self.intensities[0][mod_idx] = val;
    }
}
