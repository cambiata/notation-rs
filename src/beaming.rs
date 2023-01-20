use crate::core::*;
use crate::note::*;
use crate::notes::*;
use crate::tools::pair_iterator::*;

#[derive(Debug)]
pub enum BeamingItem<'a> {
    Unbeamed(&'a Note),
    Single(&'a Note),
    Group(&'a Notes),
}

#[derive(Debug)]
pub struct BeamingItems<'a>(pub Vec<BeamingItem<'a>>);

pub enum BeamingPattern {
    NoBeams,
    NValues(Vec<NValue>),
}

pub struct BeamingItemsGenerator;

impl BeamingItemsGenerator {
    pub fn generate(notes: &Notes, pattern: BeamingPattern) -> BeamingItems {
        match pattern {
            BeamingPattern::NoBeams => {
                let mut items: Vec<BeamingItem> = vec![];
                for note in notes {
                    let beamItem: BeamingItem = match note.ntype {
                        NoteType::Heads(_) => BeamingItem::Single(note),
                        NoteType::Pause => BeamingItem::Unbeamed(note),
                        NoteType::Slash => BeamingItem::Unbeamed(note),
                    };
                    items.push(beamItem);
                }
                BeamingItems(items)
            }
            
            BeamingPattern::NValues(values) => {
                println!("val:{:?}", notes.value);
                let mut valueCycle: Vec<NValue> = vec![];
                let mut valueCycleLength: u32 = 0;
                while valueCycleLength <= notes.value {
                    let idx: usize = 0;
                    let valueToPush = values[(idx % values.len()) as usize];
                    valueCycle.push(valueToPush);
                    valueCycleLength += valueToPush as u32;
                    println!("valueCycleLength:{}", valueCycleLength);
                }

                BeamingItems(vec![])
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::NValue::{Nv4, Nv4dot};
    use crate::core::*;
    use crate::quick::QCode;

    #[test]
    fn beaming1() {
        // let notes = QCode::notes("nv8 0 1 2 nv16 3 2 0 1 0 1 nv8dot 2 3");
        let notes = QCode::notes("nv8 p 0");
        let beams = BeamingItemsGenerator::generate(&notes, super::BeamingPattern::NoBeams);
        for beam in beams.0.iter() {
            println!("beam:{:?}", beam);
        }
    }
    #[test]
    fn beaming2() {
        // let notes = QCode::notes("nv8 0 1 2 nv16 3 2 0 1 0 1 nv8dot 2 3");
        let notes = QCode::notes("nv8 0 0 0 0 0 0 0 0 0");
        let beams = BeamingItemsGenerator::generate(
            &notes,
            super::BeamingPattern::NValues(vec![Nv4, Nv4dot]),
        );
        for beam in beams.0.iter() {
            println!("beam:{:?}", beam);
        }
    }
}
