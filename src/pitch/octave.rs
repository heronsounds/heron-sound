use crate::{Note, F};

// anything past this will go out of midi note range.
const MIDI_NOTE_CEILING: u8 = 128;

/// Represents an octave offset.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Octave {
    Base, Up(u8), Down(u8),
}

impl Default for Octave {
    fn default() -> Self {
        Self::Base
    }
}

impl From<i8> for Octave {
    fn from(val: i8) -> Self {
        match val {
            0           => Self::Base,
            v if v < 0  => Self::Down(val.unsigned_abs()),
            _           => Self::Up(val as u8),
        }
    }
}

impl From<Octave> for i8 {
    fn from(val: Octave) -> Self {
        match val {
            Octave::Base    => 0,
            Octave::Up(x)   => x as i8,
            Octave::Down(x) => -(x as i8),
        }
    }
}

/// Useful methods built on top of [Octave].
#[derive(Debug, Default)]
pub struct OctaveSpec {
    status: Octave,
}

impl OctaveSpec {

    pub fn matches(&self, status: Octave) -> bool {
        self.status == status
    }

    pub fn set(&mut self, status: Octave) {
        self.status = status;
    }

    pub fn get(&self) -> Octave {
        self.status
    }

    pub fn multiplier(&self) -> F {
        use Octave::*;
        match self.status {
            Base    => 1.0,
            // let's const some easy numbers to cut down on computations:
            Up(1)   => 2.0,
            Down(1) => 0.5,
            // outside of single octave range, we can compute the multiplier:
            Up(x)   => 2.0 * x as F,
            Down(x) => 1.0 / (2.0 * x as F),
        }
    }

    pub fn offset(&self) -> F {
        use Octave::*;
        match self.status {
            Base    => 0.0,
            Up(x)   => 12.0 * x as F,
            Down(x) => -12.0 * x as F,
        }
    }

    // TODO this could be a Proc?
    pub fn apply_note(&self, note: Note) -> Option<Note> {
        use Octave::*;
        match self.status {
            Base    => Some(note)
                        .filter(|&n| n < MIDI_NOTE_CEILING),
            Up(x)   => note.checked_add(12 * x)
                        .filter(|&n| n < MIDI_NOTE_CEILING),
            Down(x) => note.checked_sub(12 * x),
        }
    }

    pub fn apply_pitch(&self, pitch: F) -> F {
        use Octave::*;
        match self.status {
            Base    => pitch,
            Up(x)   => pitch * 2.0 * x as F,
            Down(x) => pitch / (2.0 * x as F),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_size() {
        assert_eq!(std::mem::size_of::<OctaveSpec>(), 2);
    }
    #[test]
    fn test_octave() {
        assert_eq!(Octave::default(), Octave::Base);

        assert_eq!(Octave::from(-1), Octave::Down(1));
        assert_eq!(Octave::from(0), Octave::Base);
        assert_eq!(Octave::from(1), Octave::Up(1));
        assert_eq!(Octave::from(2), Octave::Up(2));

        assert_eq!(-1i8, Octave::Down(1).into());
        assert_eq!(0i8, Octave::Base.into());
        assert_eq!(1i8, Octave::Up(1).into());
        assert_eq!(2i8, Octave::Up(2).into());
    }
    #[test]
    fn test_octave_spec() {
        let mut spec = OctaveSpec::default();
        assert!(spec.matches(Octave::Base));
        assert_eq!(spec.get(), Octave::Base);
        assert_eq!(spec.multiplier(), 1.0);
        assert_eq!(spec.offset(), 0.0);
        assert_eq!(spec.apply_note(40), Some(40));
        assert_eq!(spec.apply_note(5), Some(5));
        assert_eq!(spec.apply_pitch(0.6), 0.6);

        spec.set(Octave::Up(1));
        assert!(spec.matches(Octave::Up(1)));
        assert_eq!(spec.get(), Octave::Up(1));
        assert_eq!(spec.multiplier(), 2.0);
        assert_eq!(spec.offset(), 12.0);
        assert_eq!(spec.apply_note(40), Some(52));
        assert_eq!(spec.apply_note(120), None);
        assert_eq!(spec.apply_pitch(0.6), 1.2);

        spec.set(Octave::Down(1));
        assert!(spec.matches(Octave::Down(1)));
        assert_eq!(spec.get(), Octave::Down(1));
        assert_eq!(spec.multiplier(), 0.5);
        assert_eq!(spec.offset(), -12.0);
        assert_eq!(spec.apply_note(40), Some(28));
        assert_eq!(spec.apply_note(5), None);
        assert_eq!(spec.apply_pitch(0.6), 0.3);

        spec.set(Octave::Down(2));
        assert!(spec.matches(Octave::Down(2)));
        assert_eq!(spec.get(), Octave::Down(2));
        assert_eq!(spec.multiplier(), 0.25);
        assert_eq!(spec.offset(), -24.0);
        assert_eq!(spec.apply_note(40), Some(16));
        assert_eq!(spec.apply_note(5), None);
        assert_eq!(spec.apply_pitch(0.6), 0.15);
    }
}
