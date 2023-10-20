use std::collections::HashMap;

use crate::prelude::*;

#[derive(Debug, PartialEq)]
pub struct Voice {
    pub duration: Duration,
    pub vtype: VoiceType,
    // pub attr: VoiceAttributes,
    pub beamgroups: Option<Beamgroups>,
}

impl Voice {
    pub fn new(vtype: VoiceType) -> Self {
        let duration: Duration = match &vtype {
            VoiceType::Barpause(v) => v.unwrap_or(0),
            VoiceType::Notes(notes) => notes.duration,
        };
        Self { duration, vtype, beamgroups: None }
    }

    pub fn create_beamgroups(&mut self, pattern: &BeamingPattern) {
        match &self.vtype {
            VoiceType::Notes(notes) => {
                let beamgroups = get_beamgroups(&notes, pattern).unwrap();

                for (beamgroup_idx, beamgroup) in beamgroups.iter().enumerate() {
                    for note in beamgroup.borrow().notes.iter() {
                        note.borrow_mut().beamgroup = Some(beamgroup.clone());
                    }
                }
                self.beamgroups = Some(beamgroups);
            }
            VoiceType::Barpause(_) => {}
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum VoiceType {
    Barpause(Option<Duration>), // val
    Notes(Notes),
}

#[derive(Debug, PartialEq)]
pub enum Voices {
    One(Rc<RefCell<Voice>>),
    Two(Rc<RefCell<Voice>>, Rc<RefCell<Voice>>),
}

#[derive(Debug)]
pub struct VoiceAttributes {}

#[cfg(test)]
mod tests2 {
    use crate::prelude::*;

    #[test]
    fn example() {
        let notes = QCode::notes("nv8 0 1 2 nv16 3 2 0 1 0 1 nv8dot 2 3").unwrap();
        let voice = Voice::new(VoiceType::Notes(notes)); // VoiceAttributes
    }
    #[test]
    fn voice2() {
        let voice = Voice::new(VoiceType::Barpause(Some(NV1))); // VoiceAttributes
        println!("voice:{:?}", voice);
    }

    #[test]
    fn voice3() {
        let mut voice = QCode::voice("nv8 0 0 0").unwrap();
        voice.create_beamgroups(&BeamingPattern::NValues(vec![NV4]));
    }
}
