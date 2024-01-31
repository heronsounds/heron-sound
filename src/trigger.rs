//! Midi note trigger implementation.

use crate::{Note, Vel};

/// Midi note trigger: stores a note number and velocity.
#[derive(Debug, Default, Clone, Copy)]
pub struct NoteOn {
    pub note: Note,
    pub vel: Vel,
}
