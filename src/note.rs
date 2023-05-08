use crate::{chord::ChordItem, core::*, dynamic::DynamicItem, syllable::Syllable};
use serde::{Deserialize, Serialize};

use crate::heads::Heads;

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Note {
    pub duration: Duration,
    pub ntype: NoteType,
    pub attr: NoteAttributes,
}

impl Note {
    pub fn new(duration: usize, ntype: NoteType, attr: NoteAttributes) -> Note {
        Note {
            duration,
            ntype,
            attr,
        }
    }

    pub fn is_beamable(self: &Note) -> bool {
        match self.ntype {
            // NoteType::Pause | NoteType::Slash | NoteType::Lyric(_), NoteType::Dynamic(_), NoteType::Chord(_) => false,
            NoteType::Heads(_) => Dur::is_beamable(self.duration),
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum NoteType {
    Heads(Heads),
    Pause,
    Slash,
    Lyric(Syllable),
    Dynamic(DynamicItem),
    Chord(ChordItem),
}

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct NoteAttributes {
    pub color: Option<u16>,
}
