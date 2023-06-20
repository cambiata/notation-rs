use crate::core::DirUAD;
use crate::note::*;
use crate::notes::*;
use crate::prelude::*;
use crate::voice::Voice;
use crate::voice::VoiceType;

pub struct BeamingItem<'a> {
    pub btype: BeamingItemType<'a>,
    pub direction: DirUAD,
}

impl<'a> BeamingItem<'a> {
    pub fn new(btype: BeamingItemType<'a>) -> Self {
        Self {
            btype,
            direction: DirUAD::Auto,
        }
    }

    pub fn set_direction(&mut self, direction: DirUAD) {
        self.direction = direction;
    }
}

pub type BeamingItems<'a> = Vec<BeamingItem<'a>>;

#[derive(Debug)]
pub enum BeamingItemType<'a> {
    None(&'a Note),
    Group(Vec<&'a Note>),
}

#[derive(Debug, Clone)]
pub enum BeamingPattern {
    NoBeams,
    NValues(Vec<usize>),
}

pub struct BeamingItemsGenerator;

impl BeamingItemsGenerator {
    pub fn from_voice(voice: &Voice, pattern: BeamingPattern) -> Option<BeamingItems> {
        match voice.vtype {
            VoiceType::VBarpause(_) => None,
            VoiceType::VNotes(ref notes) => BeamingItemsGenerator::from_notes(notes, pattern).ok(),
        }
    }

    pub fn from_notes(notes: &Notes, pattern: BeamingPattern) -> Result<BeamingItems> {
        if notes.items.len() == 0 {
            return Err(Generic(format!("notes is empty")).into());
        }

        match pattern {
            BeamingPattern::NoBeams => {
                let mut items: Vec<BeamingItem> = vec![];
                for note in notes {
                    let btype: BeamingItemType = match note.ntype {
                        NoteType::Heads(_) => BeamingItemType::None(note),
                        NoteType::Pause => BeamingItemType::None(note),
                        NoteType::Slash => BeamingItemType::None(note),
                        NoteType::Lyric(_) => BeamingItemType::None(note),
                        NoteType::Chord(_) => BeamingItemType::None(note),
                        NoteType::Dynamic(_) => BeamingItemType::None(note),
                        NoteType::Spacer => BeamingItemType::None(note),
                    };
                    items.push(BeamingItem::new(btype));
                }
                Ok(items)
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
                                    // println!("create group wiBeamingItemTypeth single item");
                                    let btype = BeamingItemType::None(note_group[0]);
                                    beaming_items.push(BeamingItem::new(btype));
                                }
                                _ => {
                                    // println!("create group with multiple items");
                                    let btype = BeamingItemType::Group(note_group);
                                    beaming_items.push(BeamingItem::new(btype));
                                }
                            }
                            note_group = vec![];

                            beaming_items.push(BeamingItem::new(BeamingItemType::None(note)));
                            note_group = vec![];
                        }
                    } else {
                        // println!("note does not fit in cycle {}", note_group.len());
                        // println!("avsluta ev grupp");
                        match note_group.len() {
                            0 => {}
                            1 => {
                                let btype = BeamingItemType::None(note);
                                beaming_items.push(BeamingItem::new(btype));
                            }
                            _ => {
                                let btype = BeamingItemType::Group(note_group);
                                beaming_items.push(BeamingItem::new(btype));
                            }
                        }
                        note_group = vec![];
                        //------------------------------------------------
                        if note.is_beamable() {
                            // println!("note is beamable");
                            note_group.push(note);
                        } else {
                            // println!("note is not beamable {}", note_group.len());
                            let btype = BeamingItemType::None(note);
                            beaming_items.push(BeamingItem::new(btype));
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
                        let btype = BeamingItemType::None(note_group[0]);
                        beaming_items.push(BeamingItem::new(btype));
                    }
                    _ => {
                        let btype = BeamingItemType::Group(note_group);
                        beaming_items.push(BeamingItem::new(btype));
                    }
                }

                Ok(beaming_items)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{NV4, NV4DOT};
    use crate::quick::QCode;
    fn print_beam(beam: &BeamingItem) {
        match &beam.btype {
            BeamingItemType::None(note) => println!("single:{}", note.duration),
            BeamingItemType::Group(notes) => {
                println!("group:");
                for note in notes.iter() {
                    println!(" - note:{}", note.duration);
                }
            }
        }
    }

    #[test]
    fn beaming_2_3() {
        let notes = QCode::notes("nv8 0 0 0 0 0 0 0 0 0 0 0").unwrap();
        let beams = BeamingItemsGenerator::from_notes(
            &notes,
            super::BeamingPattern::NValues(vec![NV4, NV4DOT]),
        )
        .unwrap();

        for beam in beams.iter() {
            print_beam(beam);
        }
    }

    #[test]
    fn beaming_3() {
        let notes = QCode::notes("nv8 0 nv16 0 0 0 0 nv8 0 nv16 0 0 0 0 0 nv8 0 nv16 0 nv8tri 0 0 0 nv16tri 0 0 0 0 0 0 ").unwrap();
        let beams =
            BeamingItemsGenerator::from_notes(&notes, super::BeamingPattern::NValues(vec![NV4]))
                .unwrap();
        println!();
        for beam in beams.iter() {
            print_beam(beam);
        }
    }

    #[test]
    fn beaming_1() {
        let notes = QCode::notes("nv8 0 1 2 nv16 3 2 0 1 0 1 nv8dot 2 3").unwrap();
        let beams =
            BeamingItemsGenerator::from_notes(&notes, super::BeamingPattern::NoBeams).unwrap();
        println!();
        for beam in beams.iter() {
            print_beam(beam);
        }
    }
}
