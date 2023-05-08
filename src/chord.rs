use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct ChordItem {
    // pub root: Note,
    // pub quality: ChordQuality,
    // pub extensions: Vec<ChordExtension>,
    // pub alterations: Vec<ChordAlteration>,
    // pub bass: Option<Note>,
}
