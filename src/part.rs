use crate::prelude::*;
use crate::voice::Voice;
use crate::{complex::Complex, core::Duration, note::*};

#[derive(Debug)]
pub struct Part {
    pub ptype: PartType,
    pub background: PartBackground,
}

impl Part {
    pub fn from_voices(voices: Vec<Voice>) -> Self {
        Self {
            ptype: PartType::Voices(voices),
            background: PartBackground::FiveLines,
        }
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
    #[test]
    fn example() {
        
        
    }
}