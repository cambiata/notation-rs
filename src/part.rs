use std::collections::HashMap;

use crate::prelude::*;

#[derive(Debug)]
pub struct Part {
    pub ptype: PartType,
    pub background: PartBackground,
}

impl Part {
    pub fn from_voices(voices: Vec<Voice>) -> Result<Self> {
        Ok(Self {
            ptype: PartType::Voices(voices),
            background: PartBackground::FiveLines,
        })
    }
}

#[derive(Debug)]
pub enum PartType {
    Voices(Vec<Voice>),
}
#[derive(Debug)]
pub enum PartBackground {
    FiveLines,
}

#[cfg(test)]
mod tests {
    use crate::{complex::*, note, prelude::*};
    use std::collections::HashMap;
    #[test]
    fn example() -> Result<()> {
        let voices = QCode::voices("nv4 1 0 / nv4 0").unwrap();
        let upper_note0 = voices.get_note(0, 0)?;
        let upper_note1 = voices.get_note(0, 1)?;
        let lower_note = voices.get_note(1, 0)?;

        let voices_beamings = beamings_from_voices(
            &voices,
            &BeamingPattern::NValues(vec![NV4]),
            &DirUAD::Auto,
            &DirUAD::Auto,
        )?;
        let note_beamings_map: HashMap<&Note, &BeamingItem<'_>> =
            get_map_note_beamings(&voices_beamings)?;
        let complexes = complexes_from_voices(&voices, &note_beamings_map)?;

        for complex in &complexes {
            let overlap = get_complex_notes_overlap_type(&complex);
            let directions = get_complex_directions(&complex, &note_beamings_map)?;
            println!(
                "complex:{:?} {:?} {:?} {:?} {:?}",
                complex.position,
                complex.duration,
                complex.ctype.debug_str(),
                overlap,
                directions
            );
        }
        Ok(())
    }

    // fn test_map(map: HashMap<&Note, &BeamingItem>) {}
}
