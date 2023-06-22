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
    pub fn from_heads(duration: usize, heads: Heads) -> Note {
        Note::new(
            duration,
            NoteType::Heads(heads),
            NoteAttributes { color: None },
        )
    }

    pub fn new(duration: usize, ntype: NoteType, attr: NoteAttributes) -> Note {
        Note {
            duration,
            ntype,
            attr,
        }
    }

    pub fn is_beamable(self: &Note) -> bool {
        match self.ntype {
            // normal note
            NoteType::Heads(_) => duration_is_beamable(self.duration),
            _ => false,
        }
    }

    // pub fn get_heads_balance(self: &Note) -> i8 {
    //     match self.ntype {
    //         NoteType::Heads(ref heads) => {
    //             println!(
    //                 "heads.get_level_bottom(), heads.get_level_top():{}, :{}",
    //                 heads.get_level_bottom(),
    //                 heads.get_level_top()
    //             );
    //             heads.get_level_bottom() - heads.get_level_top()
    //         }
    //         _ => 0,
    //     }
    // }

    pub fn get_heads_top(self: &Note) -> i8 {
        match self.ntype {
            NoteType::Heads(ref heads) => heads.get_level_top(),
            _ => 0,
        }
    }

    pub fn get_heads_bottom(self: &Note) -> i8 {
        match self.ntype {
            NoteType::Heads(ref heads) => heads.get_level_bottom(),
            _ => 0,
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
    Spacer,
}

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct NoteAttributes {
    pub color: Option<u16>,
}
