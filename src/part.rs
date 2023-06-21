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
    use crate::prelude::*;

    #[test]
    fn example() {
        let voices = QCode::voices("Nv8 0 0 0 / Nv8 0 0 0 0").unwrap();
        let beamings =
            BeamingItemsGenerator::from_voices(&voices, BeamingPattern::NValues(vec![NV4]))
                .unwrap();
        let complexes = Complexes::from_voices(&voices).unwrap();
        // assert_eq!(voices.len(), 2);
        // assert_eq!(beamings.len(), 2);
        // assert_eq!(complexes.len(), 4);

        dbg!(beamings);
    }
}
