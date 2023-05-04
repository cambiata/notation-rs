use crate::core::*;
use crate::notes::*;

#[derive(Debug)]
pub struct Voice {
    pub duration: Duration,
    pub vtype: VoiceType,
    pub attr: VoiceAttributes,
}

impl Voice {
    pub fn new(vtype: VoiceType, attr: VoiceAttributes) -> Self {
        let duration = match vtype {
            VoiceType::VBarpause(ref bp) => {
                let BarPause(duration) = bp;
                *duration
            }
            VoiceType::VNotes(ref notes) => notes.duration,
        };

        Self {
            duration,
            vtype,
            attr,
            // beaming_items: vec![],
        }
    }

    pub fn get_duration(&self) -> Duration {
        match self.vtype {
            VoiceType::VBarpause(ref bp) => {
                let BarPause(val) = bp;
                *val
            }
            VoiceType::VNotes(ref notes) => notes.duration,
        }
    }
}

#[derive(Debug)]
pub struct BarPause(pub usize);

#[derive(Debug)]
pub enum VoiceType {
    VBarpause(BarPause), // val
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
        let voice = Voice::new(VBarpause(BarPause(NV1)), VoiceAttributes {});
        println!("voice:{:?}", voice);
    }
}
