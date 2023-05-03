
use crate::core::*;


use crate::notes::*;

#[derive(Debug)]
pub struct Voice {
    pub val: usize,
    pub vtype: VoiceType,
    pub attr: VoiceAttributes,
}

impl Voice {
    pub fn new(vtype: VoiceType, attr: VoiceAttributes) -> Self {
        let val = match vtype {
            VoiceType::VBarpause(val) => val,
            VoiceType::VNotes(ref notes) => notes.value,
        };

        Self {
            val,
            vtype,
            attr,
            // beaming_items: vec![],
        }
    }

    // pub fn create_beams(&mut self, pattern: BeamingPattern) {
    //     if let VoiceType::VNotes(notes) = self.vtype {
    //         self.beaming_items = BeamingItemsGenerator::generate(notes, pattern);
    //     }
    // }
}

#[derive(Debug)]
pub enum VoiceType {
    VBarpause(usize), // val
    VNotes(Notes),
}

#[derive(Debug)]
pub struct VoiceAttributes {}

#[cfg(test)]
mod tests {
    use super::VoiceType::{VBarpause, VNotes};
    use super::*;
    use crate::quick::QCode;
    

    #[test]
    fn voice() {
        let notes = QCode::notes("nv8 0 1 2 nv16 3 2 0 1 0 1 nv8dot 2 3");
        let voice = Voice::new(VNotes(notes), VoiceAttributes {});
        println!("voice:{:?}", voice);
    }

    #[test]
    fn voice2() {
        let voice = Voice::new(VBarpause(NValue::Nv1.into()), VoiceAttributes {});
        println!("voice:{:?}", voice);
    }
}
