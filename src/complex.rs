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
pub struct NoteExt<'a> {
    note: &'a Note,
    dir: Option<DirUD>,
    placements: Option<HeadsPlacement>,
}

#[derive(Debug)]
pub enum ComplexType<'a> {
    OneBarpause(&'a BarPause),
    TwoBarpauses(&'a BarPause, &'a BarPause),
    //
    OneNote(NoteExt<'a>),
    TwoNotes(NoteExt<'a>, NoteExt<'a>),
    BarpauseNote(&'a BarPause, NoteExt<'a>),
    NoteBarpause(NoteExt<'a>, &'a BarPause),
}

impl<'a> ComplexType<'a> {
    pub fn debug_str(&self) -> String {
        match self {
            ComplexType::OneBarpause(bp) => format!("OneBarpause"),
            ComplexType::OneNote(note) => format!("OneNote: {} {:?}", note.note.duration, note.dir),
            ComplexType::TwoNotes(note1, note2) => format!(
                "TwoNotes: {} {:?} / {} {:?}",
                note1.note.duration, note1.dir, note2.note.duration, note2.dir,
            ),
            ComplexType::BarpauseNote(bp, note) => {
                format!("BarpauseNote: {:?} {:?}", note.note.duration, note.dir)
            }
            ComplexType::NoteBarpause(note, bp) => {
                format!("NoteBarpause: {:?} {:?})", note.note.duration, note.dir)
            }
            ComplexType::TwoBarpauses(bp1, bp2) => format!("TwoBarpauses"),
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

    pub fn get_heads_placements(&self) -> Option<HeadsPlacement> {
        match &self.ctype {
            ComplexType::OneBarpause(_) => None,
            ComplexType::TwoBarpauses(_, _) => None,
            ComplexType::OneNote(ref note) => note.placements.clone(),
            ComplexType::TwoNotes(upper, lower) => {
                let mut placements = upper.placements.clone().unwrap();
                placements.extend(lower.placements.clone().unwrap());
                Some(placements)
            }
            ComplexType::BarpauseNote(_, note) => note.placements.clone(),
            ComplexType::NoteBarpause(note, _) => note.placements.clone(),
        }
    }

    pub fn get_notes_overlap_type(&self) -> ComplexNotesOverlap {
        match &self.ctype {
            crate::complex::ComplexType::OneBarpause(_) => ComplexNotesOverlap::None,
            crate::complex::ComplexType::TwoBarpauses(_, _) => ComplexNotesOverlap::None,
            crate::complex::ComplexType::OneNote(_) => ComplexNotesOverlap::None,
            crate::complex::ComplexType::BarpauseNote(_, _) => ComplexNotesOverlap::None,
            crate::complex::ComplexType::NoteBarpause(_, _) => ComplexNotesOverlap::None,
            crate::complex::ComplexType::TwoNotes(upper, lower) => {
                let overlap = match [&upper.note.ntype, &lower.note.ntype] {
                    [NoteType::Heads(upper_heads), NoteType::Heads(lower_heads)] => {
                        let level_diff =
                            lower_heads.get_level_top() - upper_heads.get_level_bottom();
                        let upper_head_width = match duration_get_headtype(upper.note.duration) {
                            crate::head::HeadType::NormalHead => OVERLAP_NORMAL_HEAD,
                            crate::head::HeadType::WideHead => OVERLAP_WIDE_HEAD,
                        };
                        let lower_head_width = match duration_get_headtype(lower.note.duration) {
                            crate::head::HeadType::NormalHead => OVERLAP_NORMAL_HEAD,
                            crate::head::HeadType::WideHead => OVERLAP_WIDE_HEAD,
                        };

                        if level_diff < 0 {
                            ComplexNotesOverlap::UpperRight(upper_head_width + OVERLAP_SPACE)
                        } else if level_diff == 0 {
                            let same_duration = upper.note.duration == lower.note.duration;
                            if same_duration {
                                ComplexNotesOverlap::None
                            } else {
                                ComplexNotesOverlap::UpperRight(lower_head_width + OVERLAP_SPACE)
                            }
                        } else if level_diff == 1 {
                            ComplexNotesOverlap::LowerRight(upper_head_width)
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
}

pub fn complexes_from_voices<'a>(
    voices: &'a Voices,
    map: &HashMap<&Note, &BeamingItem<'a>>,
) -> Result<Vec<Complex<'a>>> {
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
                    let dir = map.get(note).unwrap().internal_direction;
                    let placements = note.get_heads_placements(&dir.unwrap());
                    complexes.push(Complex {
                        position,
                        duration,
                        ctype: ComplexType::OneNote(NoteExt {
                            note,
                            dir,
                            placements,
                        }),
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
                        let dir = map.get(note).unwrap().internal_direction;
                        let placements = note.get_heads_placements(&dir.unwrap());
                        complexes.push(Complex {
                            position,
                            duration,
                            ctype: if idx == 0 {
                                ComplexType::BarpauseNote(
                                    bp,
                                    NoteExt {
                                        note,
                                        dir,
                                        placements,
                                    },
                                )
                            } else {
                                ComplexType::OneNote(NoteExt {
                                    note,
                                    dir,
                                    placements,
                                })
                            },
                        });
                        position += duration;
                    }
                }
                [VoiceType::VNotes(ref notes), VoiceType::VBarpause(ref bp)] => {
                    let mut position = 0;
                    for (idx, note) in notes.iter().enumerate() {
                        let duration = note.duration;
                        // let dir: Option<DirUD> = map
                        //     .get(note)
                        //     .ok_or(Basic).into())?
                        //     .internal_direction;
                        let dir = map.get(note).unwrap().internal_direction;
                        let placements = note.get_heads_placements(&dir.unwrap());

                        complexes.push(Complex {
                            position,
                            duration,
                            ctype: if idx == 0 {
                                ComplexType::NoteBarpause(
                                    NoteExt {
                                        note,
                                        dir,
                                        placements,
                                    },
                                    bp,
                                )
                            } else {
                                ComplexType::OneNote(NoteExt {
                                    note,
                                    dir,
                                    placements,
                                })
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
                                let dir1 = map.get(note1).unwrap().internal_direction;
                                let dir2 = map.get(note1).unwrap().internal_direction;
                                let placements1 = note1.get_heads_placements(&dir1.unwrap());
                                let placements2 = note2.get_heads_placements(&dir2.unwrap());
                                complexes.push(Complex {
                                    position: *position,
                                    duration,
                                    ctype: ComplexType::TwoNotes(
                                        NoteExt {
                                            note: note1,
                                            dir: dir1,
                                            placements: placements1,
                                        },
                                        NoteExt {
                                            note: note2,
                                            dir: dir2,
                                            placements: placements2,
                                        },
                                    ),
                                });
                            }
                            [Some(note), None] => {
                                let dir = map.get(note).unwrap().internal_direction;
                                let placements = note.get_heads_placements(&dir.unwrap());
                                complexes.push(Complex {
                                    position: *position,
                                    duration,
                                    ctype: ComplexType::OneNote(NoteExt {
                                        note,
                                        dir,
                                        placements,
                                    }),
                                });
                            }
                            [None, Some(note)] => {
                                let dir = map.get(note).unwrap().internal_direction;
                                let placements = note.get_heads_placements(&dir.unwrap());
                                complexes.push(Complex {
                                    position: *position,
                                    duration,
                                    ctype: ComplexType::OneNote(NoteExt {
                                        note,
                                        dir,
                                        placements,
                                    }),
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

pub fn get_map_note_beamings<'a>(
    beamings: &'a VoicesBeamings<'a>,
) -> Result<HashMap<&'a Note, &'a BeamingItem<'a>>> {
    let mut map: HashMap<&Note, &BeamingItem<'a>> = HashMap::new();
    let mut note_idx = 0;

    match beamings {
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
    UpperRight(f32),
    LowerRight(f32),
}

pub const OVERLAP_NORMAL_HEAD: f32 = 1.0;
pub const OVERLAP_WIDE_HEAD: f32 = 1.5;
pub const OVERLAP_SPACE: f32 = 0.2;
pub const OVERLAP_DIAGONAL_SPACE: f32 = -0.5;

/*
pub fn get_complex_notes_overlap_type<'a>(complex: &'a Complex) -> ComplexNotesOverlap {
    match &complex.ctype {
        crate::complex::ComplexType::OneBarpause(_) => ComplexNotesOverlap::None,
        crate::complex::ComplexType::TwoBarpauses(_, _) => ComplexNotesOverlap::None,
        crate::complex::ComplexType::OneNote(_) => ComplexNotesOverlap::None,
        crate::complex::ComplexType::BarpauseNote(_, _) => ComplexNotesOverlap::None,
        crate::complex::ComplexType::NoteBarpause(_, _) => ComplexNotesOverlap::None,
        crate::complex::ComplexType::TwoNotes(upper, lower) => {
            let overlap = match [&upper.note.ntype, &lower.note.ntype] {
                [NoteType::Heads(upper_heads), NoteType::Heads(lower_heads)] => {
                    let level_diff = lower_heads.get_level_top() - upper_heads.get_level_bottom();
                    let upper_head_width = match duration_get_headtype(upper.note.duration) {
                        crate::head::HeadType::NormalHead => OVERLAP_NORMAL_HEAD,
                        crate::head::HeadType::WideHead => OVERLAP_WIDE_HEAD,
                    };
                    let lower_head_width = match duration_get_headtype(lower.note.duration) {
                        crate::head::HeadType::NormalHead => OVERLAP_NORMAL_HEAD,
                        crate::head::HeadType::WideHead => OVERLAP_WIDE_HEAD,
                    };

                    if level_diff < 0 {
                        ComplexNotesOverlap::UpperRight(upper_head_width + OVERLAP_SPACE)
                    } else if level_diff == 0 {
                        let same_duration = upper.note.duration == lower.note.duration;
                        if same_duration {
                            ComplexNotesOverlap::None
                        } else {
                            ComplexNotesOverlap::UpperRight(lower_head_width + OVERLAP_SPACE)
                        }
                    } else if level_diff == 1 {
                        ComplexNotesOverlap::LowerRight(upper_head_width)
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
*/

#[derive(Debug)]
pub enum ComplexDirections {
    None,
    One(Option<DirUD>),
    Two(Option<DirUD>, Option<DirUD>),
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::prelude::*;

    #[test]
    fn test1() {
        let voices = QCode::voices("Nv4 #0 / Nv8 b1 0 0").unwrap();
        let voices_beamings = beamings_from_voices(
            &voices,
            &BeamingPattern::NValues(vec![NV4]),
            &DirUAD::Auto,
            &DirUAD::Auto,
        )
        .unwrap();
        let map: HashMap<&Note, &BeamingItem<'_>> =
            get_map_note_beamings(&voices_beamings).unwrap();
        let complexes = complexes_from_voices(&voices, &map).unwrap();

        let first_complex = &complexes[0];
        dbg!(first_complex);
    }

    #[test]
    fn complex() {
        let voices = QCode::voices("Nv4 0 0 / Nv8 0 0 0 0 0").unwrap();
        let voices_beamings = beamings_from_voices(
            &voices,
            &BeamingPattern::NValues(vec![NV4]),
            &DirUAD::Auto,
            &DirUAD::Auto,
        )
        .unwrap();
        let map: HashMap<&Note, &BeamingItem<'_>> =
            get_map_note_beamings(&voices_beamings).unwrap();
        let complexes = complexes_from_voices(&voices, &map).unwrap();

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
        let voices_beamings = beamings_from_voices(
            &voices,
            &BeamingPattern::NValues(vec![NV4]),
            &DirUAD::Auto,
            &DirUAD::Auto,
        )
        .unwrap();
        let map: HashMap<&Note, &BeamingItem<'_>> =
            get_map_note_beamings(&voices_beamings).unwrap();
        let complexes = complexes_from_voices(&voices, &map).unwrap();

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
        let voices_beamings = beamings_from_voices(
            &voices,
            &BeamingPattern::NValues(vec![NV4]),
            &DirUAD::Auto,
            &DirUAD::Auto,
        )
        .unwrap();
        let map: HashMap<&Note, &BeamingItem<'_>> =
            get_map_note_beamings(&voices_beamings).unwrap();
        let complexes = complexes_from_voices(&voices, &map).unwrap();

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
