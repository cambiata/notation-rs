use std::{cmp::min, collections::HashMap, iter::Map};

use crate::prelude::*;

#[derive(Debug)]
pub struct BeamingItem<'a> {
    pub position: Position,
    pub end_position: Position,
    pub btype: BeamingItemType<'a>,
    // pub heads_balance: i8,
    pub internal_direction: Option<DirUD>,
}

impl<'a> BeamingItem<'a> {
    pub fn new(btype: BeamingItemType<'a>) -> Self {
        let heads_balance = crate::beaming::get_heads_balance(&btype);

        // let internal_direction = match direction {
        //     DirUAD::Auto => {
        //         if heads_balance > 0 {
        //             Some(DirUD::Up)
        //         } else {
        //             Some(DirUD::Down)
        //         }
        //     }
        //     DirUAD::Up => Some(DirUD::Up),
        //     DirUAD::Down => Some(DirUD::Down),
        // };

        Self {
            btype,
            internal_direction: None, // set default direction based upon heads_balance
            position: 0,
            end_position: 0,
            // heads_balance,
        }
    }

    // pub fn set_direction(&mut self, direction: Option<DirUD>) {
    //     self.internal_direction = direction;
    // }
}

pub fn get_heads_balance(btype: &BeamingItemType) -> i8 {
    let balance: i8;
    match btype {
        BeamingItemType::None(note) => {
            balance = note.get_heads_bottom() + note.get_heads_top();
            // println!("NONE balance:{}", &balance);
        }
        BeamingItemType::Group(notes) => {
            let heads_top = notes.iter().map(|note| note.get_heads_top()).min().unwrap();
            let heads_bottom = notes
                .iter()
                .map(|note| note.get_heads_bottom())
                .max()
                .unwrap();

            balance = heads_bottom + heads_top;
            // println!("GROUP balance:{} {} {}", balance, heads_top, heads_bottom);
        }
    }
    balance
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

#[derive(Debug)]
pub enum VoiceBeamability<'a> {
    Unbeamable, //(VoiceType<'a>),
    Beamable(BeamingItems<'a>),
}

#[derive(Debug)]
pub enum VoicesBeamings<'a> {
    One(VoiceBeamability<'a>),
    Two(VoiceBeamability<'a>, VoiceBeamability<'a>),
}

type BeamPerNoteMap<'a> = HashMap<&'a Note, &'a BeamingItem<'a>>;

pub fn beamings_from_voices(
    voices: &Voices,
    pattern: BeamingPattern,
    main_direction: DirUAD,
    overlap_direction: DirUAD,
) -> Result<VoicesBeamings> {
    match voices {
        Voices::One(voice) => {
            let voice_beaming =
                beamings_from_voice(voice, pattern, main_direction, 0, main_direction)?;
            Ok(VoicesBeamings::One(voice_beaming))
        }
        Voices::Two(upper, lower) => {
            // println!("upper.duration:{}", upper.duration);
            // println!("lower.duration:{}", lower.duration);
            let shortest_duration = min(upper.duration, lower.duration);

            let upper_beaming = beamings_from_voice(
                upper,
                pattern.clone(),
                DirUAD::Up,
                shortest_duration,
                overlap_direction,
            )?;
            let lower_beaming = beamings_from_voice(
                lower,
                pattern,
                DirUAD::Down,
                shortest_duration,
                overlap_direction,
            )?;
            Ok(VoicesBeamings::Two(upper_beaming, lower_beaming))
        }
    }
}

pub fn beamings_from_voice(
    voice: &Voice,
    pattern: BeamingPattern,
    dir_before_breakpoint: DirUAD,
    breakpoint: usize,
    dir_after_breakpoint: DirUAD,
) -> Result<VoiceBeamability> {
    match voice.vtype {
        VoiceType::VBarpause(_) => Ok(VoiceBeamability::Unbeamable),
        VoiceType::VNotes(ref notes) => {
            let beamings = beamings_from_notes(
                notes,
                pattern,
                dir_before_breakpoint,
                breakpoint,
                dir_after_breakpoint,
            )?;
            Ok(VoiceBeamability::Beamable(beamings))
        }
    }
}

pub fn beamings_from_notes(
    notes: &Notes,
    pattern: BeamingPattern,
    dir_before_breakpoint: DirUAD,
    breakpoint: usize,
    dir_after_breakpoint: DirUAD,
) -> Result<BeamingItems> {
    if notes.items.is_empty() {
        return Err(Generic(format!("notes is empty")).into());
    }

    match pattern {
        BeamingPattern::NoBeams => {
            let mut beaming_items: Vec<BeamingItem> = vec![];
            // let mut items: Vec<BeamingItem> = vec![];
            for (note, pos, endpos) in notes.get_note_positions() {
                let btype: BeamingItemType = match note.ntype {
                    NoteType::Heads(_) => BeamingItemType::None(note),
                    NoteType::Pause => BeamingItemType::None(note),
                    NoteType::Slash => BeamingItemType::None(note),
                    NoteType::Lyric(_) => BeamingItemType::None(note),
                    NoteType::Chord(_) => BeamingItemType::None(note),
                    NoteType::Dynamic(_) => BeamingItemType::None(note),
                    NoteType::Spacer => BeamingItemType::None(note),
                };
                let dir = if endpos < breakpoint {
                    dir_before_breakpoint
                } else {
                    dir_after_breakpoint
                };
                let mut beaming_item = BeamingItem::new(btype);
                beaming_item.position = pos;
                beaming_item.end_position = endpos;
                beaming_items.push(beaming_item);
            }
            Ok(beaming_items)
        }

        BeamingPattern::NValues(values) => {
            let mut beaming_items: Vec<BeamingItem> = vec![];

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
            // let mut beaming_items: Vec<BeamingItem> = vec![];
            let mut note_group: Vec<&Note> = vec![];
            let mut note_group_start: usize = 0;

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
                        note_group_start = note_start;

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
                    note_group_start = note_start;
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
                    let group_duration: usize = note_group.iter().map(|note| note.duration).sum();
                    let btype = BeamingItemType::Group(note_group);
                    beaming_items.push(BeamingItem::new(btype));
                }
            }

            // set beaming_item positions
            let mut _pos = 0;
            for beaming_item in beaming_items.iter_mut() {
                match &mut beaming_item.btype {
                    BeamingItemType::None(note) => {
                        beaming_item.position = _pos;
                        beaming_item.end_position = _pos + note.duration;
                        _pos = beaming_item.end_position;
                    }
                    BeamingItemType::Group(notes) => {
                        beaming_item.position = _pos;
                        beaming_item.end_position = _pos;
                        for note in notes.iter() {
                            beaming_item.end_position += note.duration;
                        }
                        _pos = beaming_item.end_position;
                    }
                }
            }

            // set beam directions
            for beaming_item in beaming_items.iter_mut() {
                let dir_ud: DirUD = if beaming_item.position >= breakpoint {
                    // beaming_item.set_direction(Some(DirUD::Up));

                    match dir_after_breakpoint {
                        DirUAD::Up => DirUD::Up,
                        DirUAD::Down => DirUD::Down,
                        DirUAD::Auto => {
                            if crate::beaming::get_heads_balance(&beaming_item.btype) > 0 {
                                DirUD::Up
                            } else {
                                DirUD::Down
                            }
                        }
                    }
                } else {
                    match dir_before_breakpoint {
                        DirUAD::Up => DirUD::Up,
                        DirUAD::Down => DirUD::Down,
                        DirUAD::Auto => {
                            if crate::beaming::get_heads_balance(&beaming_item.btype) > 0 {
                                DirUD::Up
                            } else {
                                DirUD::Down
                            }
                        }
                    }
                };

                beaming_item.internal_direction = Some(dir_ud);
            }

            Ok(beaming_items)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{NV4, NV4DOT};
    use crate::quick::QCode;

    fn print_voice_beamability(vb: &VoiceBeamability) {
        match vb {
            VoiceBeamability::Unbeamable => println!("unbeamable"),
            VoiceBeamability::Beamable(items) => {
                println!("beamable");
                for item in items.iter() {
                    print_beam(item);
                }
            }
        }
    }

    fn print_beam(beam: &BeamingItem) {
        match &beam.btype {
            BeamingItemType::None(note) => {
                println!(
                    "single:{} {} {:?}",
                    beam.position, beam.end_position, beam.internal_direction
                );
            }
            BeamingItemType::Group(notes) => {
                println!(
                    "group: {} {} {:?}",
                    beam.position, beam.end_position, beam.internal_direction
                );
                for note in notes.iter() {
                    println!(" - note:{}", note.duration);
                }
            }
        }
    }

    #[test]
    fn beaming_mixed() {
        let notes = QCode::notes("nv4 0 nv8 0 0 0 Nv4 0 nv8 0").unwrap();
        let beams = beamings_from_notes(
            &notes,
            super::BeamingPattern::NValues(vec![NV4, NV4DOT]),
            DirUAD::Auto,
            0,
            DirUAD::Auto,
        )
        .unwrap();

        for beam in beams.iter() {
            print_beam(beam);
        }
    }
    #[test]
    fn beaming_2_3() {
        let notes = QCode::notes("nv8 0 0 0 0 0 0 0 0 0 0 0").unwrap();
        let beams = beamings_from_notes(
            &notes,
            super::BeamingPattern::NValues(vec![NV4, NV4DOT]),
            DirUAD::Auto,
            0,
            DirUAD::Auto,
        )
        .unwrap();

        for beam in beams.iter() {
            print_beam(beam);
        }
    }

    #[test]
    fn beaming_3() {
        let notes = QCode::notes("nv8 0 nv16 0 0 0 0 nv8 0 nv16 0 0 0 0 0 nv8 0 nv16 0 nv8tri 0 0 0 nv16tri 0 0 0 0 0 0 ").unwrap();
        let beams = beamings_from_notes(
            &notes,
            super::BeamingPattern::NValues(vec![NV4]),
            DirUAD::Auto,
            0,
            DirUAD::Auto,
        )
        .unwrap();
        println!();
        for beam in beams.iter() {
            print_beam(beam);
        }
    }

    #[test]
    fn beaming_1() {
        let notes = QCode::notes("nv8 0 1 2 nv16 3 2 0 1 0 1 nv8dot 2 3").unwrap();
        let beams = beamings_from_notes(
            &notes,
            super::BeamingPattern::NoBeams,
            DirUAD::Auto,
            0,
            DirUAD::Auto,
        )
        .unwrap();
        println!();
        for beam in beams.iter() {
            print_beam(beam);
        }
    }

    #[test]
    fn balance1() {
        let notes = QCode::notes("nv8 0 0 0 0 0 0 0 0").unwrap();
        let beams = beamings_from_notes(
            &notes,
            super::BeamingPattern::NValues(vec![NV4]),
            DirUAD::Up,
            72,
            DirUAD::Auto,
        )
        .unwrap();
        for beam in beams.iter() {
            print_beam(beam);
        }
    }

    #[test]
    fn balance2() {
        let voices = QCode::voices("nv8 0 0 0 0 1 / NV8 0 0 0 0").unwrap();
        let beams = beamings_from_voices(
            &voices,
            BeamingPattern::NValues(vec![NV4]),
            DirUAD::Auto,
            DirUAD::Down,
        )
        .unwrap();
        match beams {
            VoicesBeamings::Two(upper, lower) => {
                println!("======================================");
                println!("upper");

                print_voice_beamability(&upper);
                println!("======================================");
                println!("lower");
                print_voice_beamability(&lower);
            }
            VoicesBeamings::One(voicebeams) => {
                println!("single");
                print_voice_beamability(&voicebeams);
            }
        }
    }
}

//=========================================================================================================
