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
    use std::collections::HashMap;

    use crate::{note, prelude::*};
    #[test]
    fn example() {
        let voices = QCode::voices("Nv8 0 1 1,-2").unwrap();
        let beamings = beamings_from_voices(
            &voices,
            BeamingPattern::NValues(vec![NV4]),
            DirUAD::Auto,
            DirUAD::Auto,
        )
        .unwrap();

        let map = get_map_note_beamings(&beamings).unwrap();
        // test_voices_and_map(&voices, &map);
        // dbg!(&beamings);&
        let complexes = complexes_from_voices2(&voices, &map).unwrap();
        // complexes_from_beamings(&beamings).unwrap();

        // assert_eq!(voices.len(), 2);
        // assert_eq!(beamings.len(), 2);
        // assert_eq!(complexes.len(), 4);

        // set_beamings_directions(beamings, &complexes, DirUAD::Auto).unwrap();
    }

    // fn test_map(map: HashMap<&Note, &BeamingItem>) {}

    fn test_voices_and_map<'a>(
        voices: &'a Voices,
        map: &'a HashMap<&Note, &BeamingItem<'a>>,
    ) -> Result<Vec<Complex<'a>>> {
        Ok(vec![])
    }
}

pub fn get_map_note_beamings<'a>(
    beamings: &'a VoicesBeamings,
) -> Result<HashMap<&'a Note, &'a BeamingItem<'a>>> {
    let mut map: HashMap<&Note, &BeamingItem<'a>> = HashMap::new();

    match &beamings {
        VoicesBeamings::One(ref beamability) => match beamability {
            VoiceBeamability::Beamable(ref beamings) => {
                for bitem in beamings.iter() {
                    println!("bitem: {:?}", bitem);
                    match &bitem.btype {
                        BeamingItemType::None(ref note) => {
                            println!("note: {:?}", note);
                            map.insert(note, bitem);
                        }
                        BeamingItemType::Group(ref notes) => {
                            for note in notes {
                                println!("note: {:?}", note);
                                map.insert(note, bitem);
                            }
                        }
                    }
                }
            }
            VoiceBeamability::Unbeamable => {}
        },
        VoicesBeamings::Two(upper_beamability, lower_beamability) => {
            match upper_beamability {
                VoiceBeamability::Beamable(ref beamings) => {
                    for bitem in beamings.iter() {
                        println!("bitem: {:?}", bitem);
                        match &bitem.btype {
                            BeamingItemType::None(ref note) => {
                                println!("note: {:?}", note);
                                map.insert(note, bitem);
                            }
                            BeamingItemType::Group(ref notes) => {
                                for note in notes {
                                    println!("note: {:?}", note);
                                    map.insert(note, bitem);
                                }
                            }
                        }
                    }
                }
                VoiceBeamability::Unbeamable => {}
            };
            match lower_beamability {
                VoiceBeamability::Beamable(ref beamings) => {
                    for bitem in beamings.iter() {
                        println!("bitem: {:?}", bitem);
                        match &bitem.btype {
                            BeamingItemType::None(ref note) => {
                                println!("note: {:?}", note);
                                map.insert(note, bitem);
                            }
                            BeamingItemType::Group(ref notes) => {
                                for note in notes {
                                    println!("note: {:?}", note);
                                    map.insert(note, bitem);
                                }
                            }
                        }
                    }
                }
                VoiceBeamability::Unbeamable => {}
            };
        }
    }
    Ok(map)
}
