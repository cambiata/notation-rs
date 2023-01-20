use crate::core::*;
use serde::{Deserialize, Serialize};
use serde_json;

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
}

// impl NValueItem for Note {
//     fn val(&self) -> u32 {
//         self.val as u32
//     }
// }

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
