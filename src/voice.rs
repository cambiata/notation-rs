use crate::prelude::*;

#[derive(Debug)]
pub struct Voice {
    pub duration: Duration,
    pub vtype: VoiceType,
    pub attr: VoiceAttributes,
}

impl Voice {
    pub fn new(vtype: VoiceType, attr: VoiceAttributes) -> Self {
        let duration: Duration = match vtype {
            VoiceType::VBarpause(ref bp) => match bp {
                BarPause(Some(val)) => *val,
                BarPause(None) => 0,
            },
            VoiceType::VNotes(ref notes) => notes.duration,
        };

        Self {
            duration,
            vtype,
            attr,
        }
    }
}

// pub type Voices = Vec<Voice>;

#[derive(Debug)]
pub struct BarPause(pub Option<usize>);

#[derive(Debug)]
pub enum VoiceType {
    VBarpause(BarPause), // val
    VNotes(Notes),
}

#[derive(Debug)]
pub struct VoiceAttributes {}

// pub type Voices = (Option<Voice>, Option<Voice>);
#[derive(Debug)]
pub enum Voices {
    Two(Voice, Voice),
    One(Voice),
}

#[cfg(test)]
mod tests {
    use super::VoiceType::{VBarpause, VNotes};
    use super::*;
    use crate::quick::QCode;

    #[test]
    fn voice() {
        let notes = QCode::notes("nv8 0 1 2 nv16 3 2 0 1 0 1 nv8dot 2 3").unwrap();
        let voice = Voice::new(VNotes(notes), VoiceAttributes {});
        println!("voice:{:?}", voice);
    }

    #[test]
    fn voice2() {
        let voice = Voice::new(VBarpause(BarPause(Some(NV1))), VoiceAttributes {});
        println!("voice:{:?}", voice);
    }
}
