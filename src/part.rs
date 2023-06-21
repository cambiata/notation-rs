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
    use crate::{part::set_beamings_directions, prelude::*};
    #[test]
    fn example() {
        let voices = QCode::voices("Nv8 -2 -2 3 3 ").unwrap();
        let beamings = beamings_from_voices(&voices, BeamingPattern::NValues(vec![NV4])).unwrap();
        dbg!(&beamings);
        let complexes = complexes_from_voices(&voices).unwrap();
        // assert_eq!(voices.len(), 2);
        // assert_eq!(beamings.len(), 2);
        assert_eq!(complexes.len(), 4);

        set_beamings_directions(beamings, &complexes, DirUAD::Auto).unwrap();
    }
}

pub fn set_beamings_directions<'a>(
    beamings: VoicesBeamings<'a>,
    complexes: &Vec<Complex<'a>>,
    overlap_direction_policy: DirUAD,
) -> Result<()> {
    match beamings {
        VoicesBeamings::Two(upper_beaming, lower_beaming) => {
            println!("set directions for upper voice");
            set_beamings_directions_for_voice(upper_beaming, DirUAD::Up)?;
            println!("Set directions for lower voice");
            set_beamings_directions_for_voice(lower_beaming, DirUAD::Down)?;
        }
        VoicesBeamings::One(beaming) => {
            println!("Set directions for single voice");
            set_beamings_directions_for_voice(beaming, DirUAD::Auto)?;
        }
    };
    Ok(())
}

pub fn set_beamings_directions_for_voice(
    voice_beaming: VoiceBeamability,
    set_to_direction: DirUAD,
) -> Result<()> {
    match voice_beaming {
        VoiceBeamability::Beamable(mut beamings) => {
            for bitem in beamings.iter_mut() {
                match set_to_direction {
                    DirUAD::Up => bitem.set_direction(Some(DirUD::Up)),
                    DirUAD::Down => bitem.set_direction(Some(DirUD::Down)),
                    DirUAD::Auto => {}
                }
            }
        }
        VoiceBeamability::Unbeamable => {}
    };

    Ok(())
}
