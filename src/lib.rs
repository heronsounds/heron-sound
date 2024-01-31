// #![doc = include_str!("../README.md")]
//! `heron-sound` is a library of DSP utilities used in [Heron Sounds](https://heronsounds.com) audio plugins.
//! 
//! Currently supports the following DSP components:
//! 
//! - [Envelope generators](crate::env) (ADSR, Exponential ADSR, DA)
//! - [Envelope follower](crate::follow::EnvFollower)
//! - [Filters](crate::filter) (State-variable)
//! - [Basic waveforms](crate::wave) (Saw, Sine, Pulse, Triangle)
//! - [Oscillators](crate::osc) built on basic waveforms
//! - [Low-frequency oscillators](crate::lfo)
//! - Tools for [parameter modulation](crate::modulate)
//! - Tools for [pitch manipulation](crate::pitch)
//! - [Bitmask](crate::util::Bitmask) implementation
//! - Tools for [sample rate manipulation](crate::clock)
//! 
//! As well as a variety of utilities to support the above.
//!
//! ## Implementation
//! 
//! To help develop DSP components, we provide two basic traits,
//! [Gen](crate::Gen) and [Proc](crate::Proc).
//! `Gen` is for components that generate some value from their internal state
//! without any explicit input,
//! while `Proc` is for processors that take some input value and produce an output.
//! The inputs and outputs may be samples, gains, or more complex types,
//! but should not be user-modifiable parameters.
//! 
//! To pass in user-modifiable parameters,
//! both traits have a `Spec` associated type
//! which is passed by shared reference at processing time and evaluated,
//! along with the processor's internal state,
//! to produce an output.
//! 
//! For example, an envelope follower can be implemented as a [Proc](crate::Proc) that takes an input sample
//! and returns a bool indicating whether the envelope is engaged or not:
//! ```rust
//! struct EnvFollower {
//!     engaged: bool,
//!     counter: u32,
//! }
//! 
//! struct EnvFollowerSpec {
//!     threshold: f64,
//!     hold_time: u32,
//! }
//! 
//! # use heron_sound::Proc;
//! impl Proc<f64, bool> for EnvFollower {
//!     type Spec = EnvFollowerSpec;
//!     fn proc(&mut self, spec: &Self::Spec, sample: f64) -> bool {
//!         // implementation omitted
//! # return false;
//!     }
//! }
//! ```
//! 
//! Its internal state keeps track of whether it's currently engaged and for how long,
//! and its associated `Spec` type keeps track of the amplitude threshold and hold time,
//! both of which can be set by the user.
//! At processing time, it checks whether the input sample's amplitude is above the threshold,
//! increments its internal counter and checks it against the user-set hold time,
//! and computes a boolean value to return.
//! To see how this is implemented in practice,
//! take a look at
//! [EnvFollower](crate::follow::EnvFollower) and [EnvFollowerSpec](crate::follow::EnvFollowerSpec).
//! 
//! Most of the code in this library consists of implementations of `Gen` and `Proc`,
//! but we also include some miscellaneous utils in the [util](crate::util) mod.
//! 
//! ## Approach
//! 
//! The components in this library are implemented according to the following constraints:
//! - keep memory use low
//! - but prefer caching values over recomputing them
//! - avoid allocations during processing
//! - perform as few operations as possible during processing
//! - maintain a strict boundary between user-modifiable parameters and internal mutable state
//! - minimize dependencies
//! - avoid unsafe code
//! 
//! As well as a few more relaxed constraints:
//! - avoid newtypes, but use type aliases where it aids readability
//! - use `f64` as the default float type
//! - prefer use of phase offsets to pitches in Hz to minimize computations (see [pitch](crate::pitch))
//! - in *simple* code, bounds checks can be skipped in release builds to aid performance
//! - allow some unnecessary memory use if it's small and keeps code cleaner
//! - keep components as generic as feasible, but allow some opinionated decisions
//!   (see [ExpAdsrSpec](crate::env::ExpAdsrSpec))
//! 
//! ## Future
//! 
//! In general, we put code in this library as soon as we can find a way to make it generic enough
//! to be useful for more than one type of plugin (and write adequate tests for it).
//! Expect the functionality to expand as we clean up application-specific code
//! and make it more modular.
//! 
//! The following additions are in the works, but not fully tested or cleaned up:
//! 
//! - First class support for plugin parameters
//! - Ring buffers, delay lines, and pitch-shifting
//! - Distortion algorithms
//! - Additional filter types

pub mod clock;
pub mod env;
pub mod filter;
pub mod follow;
pub mod lfo;
pub mod modulate;
pub mod osc;
pub mod phase;
pub mod pitch;
pub mod trigger;
pub mod util;
pub mod wave;

// TYPE ALIASES for clarity //

/// Basic float type (currently `f64` but this may change).
pub type F = f64;
/// Anything measured in Hz; positive only.
pub type Hz = F;
/// Phase position of a wave; 0.0..1.0.
pub type Phase = F;
/// Amplitude of a sample; -1.0..=1.0.
pub type Sample = F;
/// Anything measured in fractions.
pub type Scale = F;
/// Anything measured in seconds; positive only, probably.
pub type Seconds = F;

/// A midi note; 0..128.
pub type Note = u8;
/// Midi velocity; 0..128.
pub type Vel = u8;

/// A processor that generates a value, with no input.
///
/// Notable implementations:
/// - [Adsr](env::Adsr): generate a gain
/// - [BasicLfo](lfo::BasicLfo): generate a sample
///
/// To take a value as input, use [Proc] instead.
pub trait Gen<O> {
    /// Type that holds this component's user-modifiable parameters
    type Spec;
    fn gen(&mut self, spec: &Self::Spec) -> O;
}

/// A processor that transforms a value.
///
/// Notable implementations:
/// - [SvfProc](filter::svf::SvfProc): filter a sample
/// - [OscCore](osc::OscCore): transform a pitch into a sample
/// - [EnvFollower](follow::EnvFollower): transform a sample into a bool representing
///   whether the envelope is on or off
///
/// If the processor has no meaningful processing-time input value, use [Gen] instead.
pub trait Proc<I, O> {
    /// Type that holds this component's user-modifiable parameters
    type Spec;
    fn proc(&mut self, spec: &Self::Spec, input: I) -> O;
}

// works out to about 2 samples at 44.1kHz; useful for tests:
#[cfg(test)]
const SHORT_TIME: Seconds = 0.00005;
