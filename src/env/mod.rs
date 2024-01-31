//! Envelope generator implementations.

/// Linear ADSR envelope generator.
mod adsr;
pub use self::adsr::{Adsr, AdsrSpec, AdsrStage};

/// Exponential ADSR envelope generator.
mod exp_adsr;
pub use self::exp_adsr::{ExpAdsr, ExpAdsrSpec};

/// Linear DA envelope generator.
mod da;
pub use self::da::{DaEnv, DaEnvSpec};

/// Utility structs for setting env times.
mod time_stage;
use self::time_stage::{TimeStage, ExpTimeStage};

/// Trait for Env generators to receive state-change messages.
// TODO these can just be a single fn with a stage arg, I think.
pub trait HoldRelease {
    fn hold(&mut self);
    fn release(&mut self);
    fn sustain(&mut self);
}
