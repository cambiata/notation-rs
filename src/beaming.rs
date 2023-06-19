use crate::note::*;
use crate::notes::*;

#[derive(Debug)]
pub enum BeamingItem<'a> {
    Single(&'a Note),
    Group(Vec<&'a Note>),
}

#[derive(Debug)]
pub struct BeamingItems<'a>(pub Vec<BeamingItem<'a>>);

pub enum BeamingPattern {
    NoBeams,
    NValues(Vec<usize>),
}

pub struct BeamingItemsGenerator;

impl BeamingItemsGenerator {
    pub fn generate(notes: &Notes, pattern: BeamingPattern) -> Vec<BeamingItem> {
        match pattern {
            BeamingPattern::NoBeams => {
                let mut items: Vec<BeamingItem> = vec![];
                for note in notes {
                    let beam_item: BeamingItem = match note.ntype {
                        NoteType::Heads(_) => BeamingItem::Single(note),
                        NoteType::Pause => BeamingItem::Single(note),
                        NoteType::Slash => BeamingItem::Single(note),
                        NoteType::Lyric(_) => BeamingItem::Single(note),
                        NoteType::Chord(_) => BeamingItem::Single(note),
                        NoteType::Dynamic(_) => BeamingItem::Single(note),
                    };
                    items.push(beam_item);
                }
                items
            }

            BeamingPattern::NValues(values) => {
                println!("val:{:?}", notes.duration);
                let mut value_cycle: Vec<(usize, usize)> = vec![];
                let mut vpos_start: usize = 0;
                let mut vpos_end: usize = 0;
                let mut idx: usize = 0;

                // create value cycle of sufficient length
                while vpos_end <= notes.duration {
                    vpos_start = vpos_end;
                    let value = values[idx % values.len()];
                    // let value_to_push = values[(idx % values.len()) as usize];
                    vpos_end = vpos_start + value;
                    value_cycle.push((vpos_start, vpos_end));
                    idx += 1;
                }

                let mut cycle_idx = 0;
                let mut cycle_start = value_cycle[cycle_idx].0;
                let mut cycle_end = value_cycle[cycle_idx].1;
                // let mut note_groups: Vec<&Vec<&Note>> = vec![];
                let mut beaming_items: Vec<BeamingItem> = vec![];
                let mut note_group: Vec<&Note> = vec![];

                for note_pos in notes.get_note_positions() {
                    let (note, note_start, note_end) = note_pos;

                    if note_end <= cycle_end {
                        // println!("note fits in cycle");
                        if note.is_beamable() {
                            // println!("note is beamable");
                            note_group.push(note);
                        } else {
                            // println!("note is not beamable");
                            // println!("avsluta ev grupp");
                            match note_group.len() {
                                0 => {}
                                1 => {
                                    // println!("create group with single item");
                                    let item = BeamingItem::Single(note_group[0]);
                                    beaming_items.push(item);
                                }
                                _ => {
                                    // println!("create group with multiple items");
                                    let item = BeamingItem::Group(note_group);
                                    beaming_items.push(item);
                                }
                            }
                            note_group = vec![];

                            beaming_items.push(BeamingItem::Single(note));
                            note_group = vec![];
                        }
                    } else {
                        // println!("note does not fit in cycle {}", note_group.len());
                        // println!("avsluta ev grupp");
                        match note_group.len() {
                            0 => {}
                            1 => {
                                println!("create group with single item");
                                let item = BeamingItem::Single(note);
                                beaming_items.push(item);
                            }
                            _ => {
                                println!("create group with multiple items");
                                let item = BeamingItem::Group(note_group);
                                beaming_items.push(item);
                            }
                        }
                        note_group = vec![];
                        //------------------------------------------------
                        if note.is_beamable() {
                            // println!("note is beamable");
                            note_group.push(note);
                        } else {
                            // println!("note is not beamable {}", note_group.len());
                            beaming_items.push(BeamingItem::Single(note));
                            note_group = vec![];
                        }

                        while cycle_start < note_start && cycle_idx < value_cycle.len() - 1 {
                            cycle_idx += 1;
                            cycle_start = value_cycle[cycle_idx].0;
                            cycle_end = value_cycle[cycle_idx].1;
                        }
                    }
                }

                match note_group.len() {
                    0 => {}
                    1 => {
                        let item = BeamingItem::Single(note_group[0]);
                        beaming_items.push(item);
                    }
                    _ => {
                        let item = BeamingItem::Group(note_group);
                        beaming_items.push(item);
                    }
                }

                beaming_items
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{NV4, NV4DOT};
    use crate::quick::QCode;

    #[test]
    fn beaming_2() {
        // let notes = QCode::notes("nv8 0 1 2 nv16 3 2 0 1 0 1 nv8dot 2 3");
        let notes = QCode::notes("nv8 0 0 0 0 0 p").unwrap();
        let beams = BeamingItemsGenerator::generate(
            &notes,
            super::BeamingPattern::NValues(vec![NV4, NV4DOT]),
        );
        println!();
        for beam in beams.iter() {
            println!("beam:{:?}", beam);
            println!();
        }
    }

    #[test]
    fn beaming_3() {
        // let notes = QCode::notes("nv8 0 1 2 nv16 3 2 0 1 0 1 nv8dot 2 3");
        let notes = QCode::notes("nv8 0 0").unwrap();
        // let notes = QCode::notes("nv4 0 0 0 0 0 ");
        let beams =
            BeamingItemsGenerator::generate(&notes, super::BeamingPattern::NValues(vec![NV4]));
        println!();
        for beam in beams.iter() {
            println!("beam:{:?}", beam);
            println!();
        }
    }

    #[test]
    fn beaming_1() {
        let notes = QCode::notes("nv8 0 1 2 nv16 3 2 0 1 0 1 nv8dot 2 3").unwrap();
        // let notes = QCode::notes("nv8 p 0");
        let beams = BeamingItemsGenerator::generate(&notes, super::BeamingPattern::NoBeams);
        println!();
        for beam in beams.iter() {
            println!("beam:{:?}", beam);
            //
        }
    }
}
