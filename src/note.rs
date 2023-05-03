use crate::core::*;
use serde::{Deserialize, Serialize};


use crate::heads::Heads;

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Note {
    pub value: NValue,
    pub ntype: NoteType,
    pub attr: NoteAttributes,
}

impl Note {
    pub fn new(value: NValue, ntype: NoteType, attr: NoteAttributes) -> Note {
        Note { value, ntype, attr }
    }

    pub fn is_beamable(self: &Note) -> bool {
        match self.ntype {
            NoteType::Pause | NoteType::Slash => false,
            NoteType::Heads(_) => self.value.is_beamable(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum NoteType {
    Heads(Heads),
    Pause,
    Slash,
}

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct NoteAttributes {
    pub color: Option<u16>,
}
