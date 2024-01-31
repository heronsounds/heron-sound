<!-- cargo-rdme start -->

`heron-sound` is a library of DSP utilities used in [Heron Sounds](https://heronsounds.com) audio plugins.

Currently supports the following DSP components:

- Envelope generators (ADSR, Exponential ADSR, DA)
- Envelope follower
- Filters (State-variable)
- Basic waveforms (Saw, Sine, Pulse, Triangle)
- Oscillators built on basic waveforms
- Low-frequency oscillators
- Tools for parameter modulation
- Tools for pitch manipulation
- Bitmask implementation
- Tools for sample rate manipulation

As well as a variety of utilities to support the above.

## Implementation

To help develop DSP components, we provide two basic traits,
Gen and Proc.
`Gen` is for components that generate some value from their internal state
without any explicit input,
while `Proc` is for processors that take some input value and produce an output.
The inputs and outputs may be samples, gains, or more complex types,
but should not be user-modifiable parameters.

To pass in user-modifiable parameters,
both traits have a `Spec` associated type
which is passed by shared reference at processing time and evaluated,
along with the processor's internal state,
to produce an output.

For example, an envelope follower can be implemented as a Proc that takes an input sample
and returns a bool indicating whether the envelope is engaged or not:
```rust
struct EnvFollower {
    engaged: bool,
    counter: u32,
}

struct EnvFollowerSpec {
    threshold: f64,
    hold_time: u32,
}

impl Proc<f64, bool> for EnvFollower {
    type Spec = EnvFollowerSpec;
    fn proc(&mut self, spec: &Self::Spec, sample: f64) -> bool {
        // implementation omitted
    }
}
```

Its internal state keeps track of whether it's currently engaged and for how long,
and its associated `Spec` type keeps track of the amplitude threshold and hold time,
both of which can be set by the user.
At processing time, it checks whether the input sample's amplitude is above the threshold,
increments its internal counter and checks it against the user-set hold time,
and computes a boolean value to return.
To see how this is implemented in practice,
take a look at
EnvFollower and EnvFollowerSpec.

Most of the code in this library consists of implementations of `Gen` and `Proc`,
but we also include some miscellaneous utils in the util mod.

## Approach

The components in this library are implemented according to the following constraints:
- keep memory use low
- but prefer caching values over recomputing them
- avoid allocations during processing
- perform as few operations as possible during processing
- maintain a strict boundary between user-modifiable parameters and internal mutable state
- minimize dependencies
- avoid unsafe code

As well as a few more relaxed constraints:
- avoid newtypes, but use type aliases where it aids readability
- use `f64` as the default float type
- prefer use of phase offsets to pitches in Hz to minimize computations (see pitch)
- in *simple* code, bounds checks can be skipped in release builds to aid performance
- allow some unnecessary memory use if it's small and keeps code cleaner
- keep components as generic as feasible, but allow some opinionated decisions
  (see ExpAdsrSpec)

## Future

In general, we put code in this library as soon as we can find a way to make it generic enough
to be useful for more than one type of plugin (and write adequate tests for it).
Expect the functionality to expand as we clean up application-specific code
and make it more modular.

The following additions are in the works, but not fully tested or cleaned up:

- First class support for plugin parameters
- Ring buffers, delay lines, and pitch-shifting
- Distortion algorithms
- Additional filter types

<!-- cargo-rdme end -->
