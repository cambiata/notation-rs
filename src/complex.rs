use std::collections::{HashMap, HashSet};
use std::marker::PhantomData;

use crate::core::*;
use crate::note::Note;
use crate::prelude::*;
use crate::voice::{BarPause, Voice, VoiceType};

#[derive(Debug)]
pub struct Complex<'a> {
    pub position: Position,
    pub duration: Duration,
    pub ctype: ComplexType<'a>,
    // pub bitem: BeamingItem<'a>,
}

#[derive(Debug)]
pub enum ComplexType<'a> {
    OneBarpause(&'a BarPause),
    TwoBarpauses(&'a BarPause, &'a BarPause),
    //
    OneNote(&'a Note),
    TwoNotes(&'a Note, &'a Note),
    BarpauseNote(&'a BarPause, &'a Note),
    NoteBarpause(&'a Note, &'a BarPause),
}

impl<'a> ComplexType<'a> {
    pub fn debug_str(&self) -> String {
        match self {
            ComplexType::OneBarpause(bp) => format!("OneBarpause({:?})", bp),
            ComplexType::OneNote(note) => format!("OneNote"),
            ComplexType::TwoNotes(note1, note2) => format!("TwoNotes()"),
            ComplexType::BarpauseNote(bp, note) => format!("BarpauseNote({:?} note)", bp),
            ComplexType::NoteBarpause(note, bp) => format!("NoteBarpause(note/{:?})", bp),
            ComplexType::TwoBarpauses(bp1, bp2) => format!("TwoBarpauses({:?}/{:?})", bp1, bp2),
        }
    }
}

impl<'a> Complex<'a> {
    pub fn new(position: Position, duration: Duration, ctype: ComplexType<'a>) -> Self {
        Self {
            position,
            duration,
            ctype,
        }
    }
}

pub fn complexes_from_voices(voices: &Voices) -> Result<Vec<Complex>> {
    let mut complexes: Vec<Complex> = vec![];

    match voices {
        Voices::One(upper) => match upper.vtype {
            VoiceType::VBarpause(ref bp) => {
                complexes.push(Complex {
                    position: 0,
                    duration: upper.duration,
                    ctype: ComplexType::OneBarpause(bp),
                });
            }
            VoiceType::VNotes(ref notes) => {
                println!("notes");
                let mut position = 0;
                for note in notes {
                    println!("- note:{:?}", note);
                    let duration = note.duration;
                    complexes.push(Complex {
                        position,
                        duration,
                        ctype: ComplexType::OneNote(note),
                    });
                    position += duration;
                }
            }
        },
        Voices::Two(upper, lower) => {
            match [&upper.vtype, &lower.vtype] {
                [VoiceType::VBarpause(ref bp), VoiceType::VNotes(ref notes)] => {
                    let mut position = 0;
                    for (idx, note) in notes.iter().enumerate() {
                        let duration = note.duration;
                        complexes.push(Complex {
                            position,
                            duration,
                            ctype: if idx == 0 {
                                ComplexType::BarpauseNote(bp, note)
                            } else {
                                ComplexType::OneNote(note)
                            },
                        });
                        position += duration;
                    }
                }
                [VoiceType::VNotes(ref notes), VoiceType::VBarpause(ref bp)] => {
                    let mut position = 0;
                    for (idx, note) in notes.iter().enumerate() {
                        let duration = note.duration;
                        complexes.push(Complex {
                            position,
                            duration,
                            ctype: if idx == 0 {
                                ComplexType::NoteBarpause(note, bp)
                            } else {
                                ComplexType::OneNote(note)
                            },
                        });
                        position += duration;
                    }
                }
                [VoiceType::VNotes(ref notes1), VoiceType::VNotes(ref notes2)] => {
                    let max_duration = notes1.duration.max(notes2.duration);
                    let min_duration = notes1.duration.min(notes2.duration);

                    let mut map1: HashMap<usize, &Note> = HashMap::new();
                    for np in notes1.get_note_positions() {
                        map1.insert(np.1, np.0);
                    }
                    let mut map2: HashMap<usize, &Note> = HashMap::new();
                    for np in notes2.get_note_positions() {
                        map2.insert(np.1, np.0);
                    }

                    let mut positions_hash: HashSet<usize> = HashSet::new();
                    map1.keys().for_each(|f| {
                        positions_hash.insert(*f);
                    });
                    map2.keys().for_each(|f| {
                        positions_hash.insert(*f);
                    });

                    let mut positions: Vec<usize> = positions_hash.into_iter().collect();
                    positions.sort();

                    let mut durations: Vec<usize> =
                        positions.windows(2).map(|f| f[1] - f[0]).collect();
                    durations.push(max_duration - positions[positions.len() - 1]);

                    //---------------------------------------------------------------------------------

                    for (idx, position) in positions.iter().enumerate() {
                        let duration = durations[idx];

                        match [map1.get(position), map2.get(position)] {
                            [Some(note1), Some(note2)] => {
                                complexes.push(Complex {
                                    position: *position,
                                    duration,
                                    ctype: ComplexType::TwoNotes(note1, note2),
                                });
                            }
                            [Some(note1), None] => {
                                complexes.push(Complex {
                                    position: *position,
                                    duration,
                                    ctype: ComplexType::OneNote(note1),
                                });
                            }
                            [None, Some(note2)] => {
                                complexes.push(Complex {
                                    position: *position,
                                    duration,
                                    ctype: ComplexType::OneNote(note2),
                                });
                            }
                            [None, None] => {
                                return Err(ComplexError(
                                    "Complex match error - None/None".to_string(),
                                )
                                .into());
                            }
                        }
                    }
                }
                [VoiceType::VBarpause(ref bp1), VoiceType::VBarpause(ref bp2)] => {
                    println!("barpause/barpause");
                    complexes.push(Complex {
                        position: 0,
                        duration: upper.duration.max(lower.duration),
                        ctype: ComplexType::TwoBarpauses(bp1, bp2),
                    });
                }
            }
        }
    };

    Ok(complexes)
}

pub fn get_complex_directions(
    complex: &Complex,
    map: &HashMap<&Note, &BeamingItem>,
) -> Result<ComplexDirections> {
    match complex.ctype {
        ComplexType::OneBarpause(_) => Ok(ComplexDirections::One(None)),
        ComplexType::TwoBarpauses(_, _) => Ok(ComplexDirections::Two(None, None)),
        ComplexType::OneNote(note) => {
            let dir = map.get(&note).ok_or(Basic)?.internal_direction;
            Ok(ComplexDirections::One(dir))
        }
        ComplexType::TwoNotes(upper, lower) => {
            let upper_dir = map.get(&upper).ok_or(Basic)?.internal_direction;
            let lower_dir = map.get(&lower).ok_or(Basic)?.internal_direction;
            println!("upper_dir, lower_dir:{:?}, :{:?}", &upper_dir, &lower_dir);
            Ok(ComplexDirections::Two(upper_dir, lower_dir))
        }
        ComplexType::BarpauseNote(_, note) => {
            let dir = map.get(&note).ok_or(Basic)?.internal_direction;
            Ok(ComplexDirections::Two(None, dir))
        }
        ComplexType::NoteBarpause(note, _) => {
            let dir = map.get(&note).ok_or(Basic)?.internal_direction;
            Ok(ComplexDirections::Two(dir, None))
        }
    }
}

pub fn get_map_note_beamings<'a>(
    beamings: &'a VoicesBeamings,
) -> Result<HashMap<&'a Note, &'a BeamingItem<'a>>> {
    let mut map: HashMap<&Note, &BeamingItem<'a>> = HashMap::new();

    let mut note_idx = 0;
    match &beamings {
        VoicesBeamings::One(ref beamability) => match beamability {
            Some(ref beamings) => {
                for (idx, bitem) in beamings.iter().enumerate() {
                    match &bitem.btype {
                        BeamingItemType::None(ref note) => {
                            println!(
                                "- One None: bitem idx:{}:{} note_idx:{}",
                                idx, bitem.position, note_idx
                            );
                            map.insert(note, bitem);
                            note_idx += 1;
                        }
                        BeamingItemType::Group(ref notes) => {
                            for note in notes {
                                println!(
                                    "- One Group: bitem idx:{}:{} note_idx:{}",
                                    idx, bitem.position, note_idx
                                );
                                map.insert(note, bitem);
                                note_idx += 1;
                            }
                        }
                    }
                }
            }
            None => {}
        },
        VoicesBeamings::Two(upper_beamability, lower_beamability) => {
            match upper_beamability {
                Some(ref beamings) => {
                    for bitem in beamings.iter() {
                        match &bitem.btype {
                            BeamingItemType::None(ref note) => {
                                map.insert(note, bitem);
                            }
                            BeamingItemType::Group(ref notes) => {
                                for note in notes {
                                    map.insert(note, bitem);
                                }
                            }
                        }
                    }
                }
                None => {}
            };
            match lower_beamability {
                Some(ref beamings) => {
                    for bitem in beamings.iter() {
                        match &bitem.btype {
                            BeamingItemType::None(ref note) => {
                                map.insert(note, bitem);
                            }
                            BeamingItemType::Group(ref notes) => {
                                for note in notes {
                                    map.insert(note, bitem);
                                }
                            }
                        }
                    }
                }
                None => {}
            };
        }
    }
    Ok(map)
}

#[derive(Debug)]
pub enum ComplexNotesOverlap {
    None,
    UnderRight(f32),
    LowerRight(f32),
}

pub const OVERLAP_NORMAL_HEAD: f32 = 1.0;
pub const OVERLAP_WIDE_HEAD: f32 = 1.5;
pub const OVERLAP_SPACE: f32 = 0.2;
pub const OVERLAP_DIAGONAL_SPACE: f32 = -0.5;

pub fn get_complex_notes_overlap_type<'a>(complex: &'a Complex) -> ComplexNotesOverlap {
    match complex.ctype {
        crate::complex::ComplexType::OneBarpause(_) => ComplexNotesOverlap::None,
        crate::complex::ComplexType::TwoBarpauses(_, _) => ComplexNotesOverlap::None,
        crate::complex::ComplexType::OneNote(_) => ComplexNotesOverlap::None,
        crate::complex::ComplexType::BarpauseNote(_, _) => ComplexNotesOverlap::None,
        crate::complex::ComplexType::NoteBarpause(_, _) => ComplexNotesOverlap::None,
        crate::complex::ComplexType::TwoNotes(upper, lower) => {
            let overlap = match [&upper.ntype, &lower.ntype] {
                [NoteType::Heads(upper_heads), NoteType::Heads(lower_heads)] => {
                    let level_diff = lower_heads.get_level_top() - upper_heads.get_level_bottom();

                    let upper_head_width = match duration_get_headtype(upper.duration) {
                        crate::head::HeadType::NormalHead => OVERLAP_NORMAL_HEAD,
                        crate::head::HeadType::WideHead => OVERLAP_WIDE_HEAD,
                    };

                    if level_diff < 0 {
                        ComplexNotesOverlap::UnderRight(upper_head_width + OVERLAP_SPACE)
                    } else if level_diff == 0 {
                        ComplexNotesOverlap::UnderRight(upper_head_width)
                    } else if level_diff == 1 {
                        ComplexNotesOverlap::UnderRight(upper_head_width + OVERLAP_DIAGONAL_SPACE)
                    } else {
                        ComplexNotesOverlap::None
                    }
                }
                _ => ComplexNotesOverlap::None,
            };
            overlap
        }
    }
}

#[derive(Debug)]
pub enum ComplexDirections {
    None,
    One(Option<DirUD>),
    Two(Option<DirUD>, Option<DirUD>),
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test1() {
        let voices = QCode::voices("Nv4 #0 / Nv8 b1 0 0").unwrap();
        let complexes = complexes_from_voices(&voices).unwrap();
        let first_complex = &complexes[0];
        dbg!(first_complex);
    }

    #[test]
    fn complex() {
        let voices = QCode::voices("Nv4 0 0 / Nv8 0 0 0 0 0").unwrap();
        let complexes = complexes_from_voices(&voices).unwrap();
        for complex in complexes {
            println!(
                "complex:{:?} {:?} {:?}",
                complex.position,
                complex.duration,
                complex.ctype.debug_str()
            );
        }
    }
    #[test]
    fn complex2() {
        let voices = QCode::voices(" Nv4 0 0 0 / bp").unwrap();
        let complexes = complexes_from_voices(&voices).unwrap();
        for complex in complexes {
            println!(
                "complex:{:?} {:?} {:?}",
                complex.position,
                complex.duration,
                complex.ctype.debug_str()
            );
        }
    }
    #[test]
    fn complex3() {
        let voices = QCode::voices(" bp nv4/ bp nv8 nv8 nv8  ").unwrap();
        let complexes = complexes_from_voices(&voices).unwrap();
        for complex in complexes {
            println!(
                "complex:{:?} {:?} {:?}",
                complex.position,
                complex.duration,
                complex.ctype.debug_str()
            );
        }
    }
}
