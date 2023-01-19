use crate::beaming::*;
use crate::core::*;
use crate::heads::*;
use crate::note::*;
use crate::notes::*;

#[derive(Debug)]
pub struct Voice<'a> {
    pub val: u32,
    pub vtype: VoiceType<'a>,
    pub attr: VoiceAttributes,
    pub beaming_items: BeamingItems<'a>,
}

impl<'a> Voice<'a> {
    pub fn new(vtype: VoiceType<'a>, attr: VoiceAttributes) -> Self {
        Self {
            val: 0,
            vtype,
            attr,
            beaming_items: BeamingItems(vec![]),
        }
    }

    pub fn create_beams(&mut self, pattern: BeamingPattern) {
        if let VoiceType::VNotes(notes) = self.vtype {
            self.beaming_items = BeamingItemsGenerator::generate(notes, pattern);
        }
    }
}

#[derive(Debug)]
pub enum VoiceType<'a> {
    VBarpause(u32), // val
    VNotes(&'a Notes),
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
        let voice = Voice::new(VNotes(&notes), VoiceAttributes {});
        println!("voice:{:?}", voice);
    }
}
