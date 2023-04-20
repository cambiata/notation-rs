use crate::core::*;
use crate::note;
use crate::note::*;
use crate::notes::*;
// use crate::tools::pair_iterator::*;

#[derive(Debug)]
pub enum BeamingItem<'a> {
    Unbeamed(&'a Note),
    Single(&'a Note),
    Group(Vec<&'a Note>),
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
                let mut value_cycle: Vec<(usize,usize)> = vec![];
                let mut vpos_start: usize = 0;
                let mut vpos_end: usize = 0;
                let mut idx: usize = 0;
                while vpos_end <= notes.value as usize  {
                    vpos_start = vpos_end;
                    let value = values[idx  % values.len()] as usize;
                    // let value_to_push = values[(idx % values.len()) as usize];
                    vpos_end = vpos_start + value as usize;
                    value_cycle.push((vpos_start, vpos_end));
                    idx += 1;
                }

                println!("value_cycle:{:?}", v alue_cycle);
                let note_positions = NotesPositions::new(notes);
                let mut cycle_idx = 0;
                let mut cycle_start = value_cycle[cycle_idx].0;
                let mut cycle_end = value_cycle[cycle_idx].1;
                // let mut note_groups:Vec<Vec<&Note>> = vec![];
                let mut note_group:Vec<&Note> = vec![];

                for note_pos in note_positions {
                    let (note_idx, note_start, note_end, note) = note_pos;
                    
                    if !note.is_beamable() {
                        note_group.push(note);
                        println!(":{} {} {} {} {} {} {} {}", note_idx, note_start, note_end, note.is_beamable(), cycle_idx, cycle_start, cycle_end, note_group.len());
                        note_group = vec![];
                        continue;
                    }

                    if note_end <= cycle_end {
                        note_group.push(note);
                        println!(":{} {} {} {} {} {} {} {}", note_idx, note_start, note_end, note.is_beamable(), cycle_idx, cycle_start, cycle_end, note_group.len());
                        continue;
                    }
                    note_group = vec![];
                    cycle_idx += 1;
                    cycle_start = value_cycle[cycle_idx].0;
                    cycle_end = value_cycle[cycle_idx].1;
                    note_group.push(note);

                    println!(":{} {} {} {} {} {} {} {}", note_idx, note_start, note_end, note.is_beamable(), cycle_idx, cycle_start, cycle_end, note_group.len());
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
        let notes = QCode::notes("nv8 0 1 2 nv16 3 2 0 1 0 1 nv8dot 2 3");
        // let notes = QCode::notes("nv8 p 0");
        let beams = BeamingItemsGenerator::generate(&notes, super::BeamingPattern::NoBeams);
        for beam in beams.0.iter() {
            println!("beam:{:?}", beam);
        }
    }
    #[test]
    fn beaming2() {
        // let notes = QCode::notes("nv8 0 1 2 nv16 3 2 0 1 0 1 nv8dot 2 3");
        let notes = QCode::notes("nv8 0 0 0 0 0 0");
        let beams = BeamingItemsGenerator::generate(
            &notes,
            super::BeamingPattern::NValues(vec![Nv4, Nv4dot]),
        );
        for beam in beams.0.iter() {
            println!("beam:{:?}", beam);
        }
    }

    #[test]
    fn beaming3() {
        // let notes = QCode::notes("nv8 0 1 2 nv16 3 2 0 1 0 1 nv8dot 2 3");
        let notes = QCode::notes("nv8 0 nv16 p 0");
        // let notes = QCode::notes("nv4 0 0 0 0 0 ");
        let beams = BeamingItemsGenerator::generate(
            &notes,
            super::BeamingPattern::NValues(vec![Nv4]),
        );
        for beam in beams.0.iter() {
            println!("beam:{:?}", beam);
            println!("");    
        }
    }
}
