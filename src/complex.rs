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
pub struct NoteExt2<'a> {
    note: &'a Note,
    dir: Option<DirUD>,
    placements: Option<HeadsPlacement<'a>>,
}

#[derive(Debug)]
pub struct NoteExt<'a>(
    pub &'a Note,
    pub Option<DirUD>,
    pub Option<HeadsPlacement<'a>>,
);

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
            ComplexType::OneNote(note) => format!("OneNote: {} {:?}", note.0.duration, note.1),
            ComplexType::TwoNotes(note1, note2) => format!(
                "TwoNotes: {} {:?} / {} {:?}",
                note1.0.duration, note1.1, note2.0.duration, note2.1,
            ),
            ComplexType::BarpauseNote(bp, note) => {
                format!("BarpauseNote: {:?} {:?}", note.0.duration, note.1)
            }
            ComplexType::NoteBarpause(note, bp) => {
                format!("NoteBarpause: {:?} {:?})", note.0.duration, note.1)
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

    pub fn get_rectangles(&self) -> Option<Vec<NRectExt>> {
        let mut rects: Vec<NRectExt> = Vec::new();

        // Heads rects
        let heads_rects = match &self.ctype {
            ComplexType::OneBarpause(_) => rects,
            ComplexType::TwoBarpauses(_, _) => rects,

            ComplexType::OneNote(ref note) => {
                rects = add_note_rects(rects, note, 0.0, 0.);
                rects
            }
            ComplexType::TwoNotes(upper, lower) => {
                let (notes_overlap, dots_type) = self.get_notes_overlap_type();
                let mut upper_overlap = 0.0;
                let mut lower_overlap = 0.0;
                match notes_overlap {
                    ComplexNotesOverlap::None => {}
                    ComplexNotesOverlap::UpperRight(overlap) => {
                        upper_overlap = overlap;
                    }
                    ComplexNotesOverlap::LowerRight(overlap) => {
                        lower_overlap = overlap;
                    }
                }

                // --------------------------------------------------
                rects = add_note_rects(rects, upper, upper_overlap, 2. * -SPACE_HALF);
                rects = add_note_rects(rects, lower, lower_overlap, 2. * SPACE_HALF);
                rects
                // None
            }

            ComplexType::BarpauseNote(_, note) => {
                rects = add_note_rects(rects, note, 0.0, 0.);
                rects
            }

            ComplexType::NoteBarpause(note, _) => {
                rects = add_note_rects(rects, note, 0.0, 0.);
                rects
            }
        };

        Some(heads_rects)
    }

    // const OVERLAP_NORMAL_HEAD: f32 = 1.0;
    // const OVERLAP_WIDE_HEAD: f32 = 1.5;
    // const OVERLAP_SPACE: f32 = 0.1;
    // const OVERLAP_DIAGONAL_SPACE: f32 = -0.5;

    pub fn get_notes_overlap_type(&self) -> (ComplexNotesOverlap, Option<DotsInfo>) {
        match &self.ctype {
            crate::complex::ComplexType::OneBarpause(_) => (ComplexNotesOverlap::None, None),
            crate::complex::ComplexType::TwoBarpauses(_, _) => (ComplexNotesOverlap::None, None),
            crate::complex::ComplexType::OneNote(note) => {
                let dots_info = note.0.get_dots_info(); // TODO: use this info
                (ComplexNotesOverlap::None, None)
            }
            crate::complex::ComplexType::BarpauseNote(_, _) => (ComplexNotesOverlap::None, None),
            crate::complex::ComplexType::NoteBarpause(_, _) => (ComplexNotesOverlap::None, None),

            crate::complex::ComplexType::TwoNotes(upper, lower) => {
                let overlap = match [&upper.0.ntype, &lower.0.ntype] {
                    [NoteType::Heads(upper_heads), NoteType::Heads(lower_heads)] => {
                        let level_diff =
                            lower_heads.get_level_top() - upper_heads.get_level_bottom();

                        let upper_head_width = match duration_get_headshape(&upper.0.duration) {
                            HeadShape::BlackHead => HEAD_WIDTH_BLACK,
                            HeadShape::WhiteHead => HEAD_WIDTH_WHITE,
                            HeadShape::WholeHead => HEAD_WIDTH_WIDE,
                        };

                        let upper_dots_nr = duration_get_dots(&upper.0.duration);
                        let upper_dots_width = upper_dots_nr as f32 * DOT_WIDTH;

                        let lower_head_width = match duration_get_headshape(&lower.0.duration) {
                            HeadShape::BlackHead => HEAD_WIDTH_BLACK,
                            HeadShape::WhiteHead => HEAD_WIDTH_WHITE,
                            HeadShape::WholeHead => HEAD_WIDTH_WIDE,
                        };

                        let lower_dots_nr = duration_get_dots(&lower.0.duration);
                        let lower_dots_width = lower_dots_nr as f32 * DOT_WIDTH;

                        let dots_info = Some(DotsInfo::DotsOnNotes(
                            upper.0.get_dots_info(),
                            lower.0.get_dots_info(),
                        ));

                        if level_diff < 0 {
                            // upper is lower than lower
                            (
                                ComplexNotesOverlap::UpperRight(
                                    lower_head_width + lower_dots_width,
                                ),
                                dots_info,
                            )
                        } else if level_diff == 0 {
                            // same level
                            let same_duration = upper.0.duration == lower.0.duration;
                            if same_duration {
                                (ComplexNotesOverlap::None, dots_info)
                            } else {
                                (
                                    ComplexNotesOverlap::UpperRight(
                                        lower_head_width + lower_dots_width,
                                    ),
                                    dots_info,
                                )
                            }
                        } else if level_diff == 1 {
                            // lower is one lower than upper
                            (
                                ComplexNotesOverlap::LowerRight(
                                    upper_head_width + lower_dots_width,
                                ),
                                dots_info,
                            )
                        } else {
                            // level_diff > 1
                            (ComplexNotesOverlap::None, dots_info)
                        }
                    }
                    _ => (ComplexNotesOverlap::None, None),
                };
                overlap
            }
        }
    }
}

fn add_dots_rects<'a>(
    mut rects: Vec<NRectExt<'a>>,
    note: &NoteExt<'a>,
    note_overlap: f32,
) -> Vec<NRectExt<'a>> {
    rects
}

fn add_note_rects<'a>(
    mut rects: Vec<NRectExt<'a>>,
    note: &NoteExt<'a>,
    note_overlap: f32,
    avoid_y_collision: f32,
) -> Vec<NRectExt<'a>> {
    match note.0.ntype {
        NoteType::Heads(_) => {
            let note_head_type = duration_get_headtype(&note.0.duration);
            let note_shape = duration_get_headshape(&note.0.duration);
            let duration = note.0.duration;
            let dots_width = duration_get_dots(&duration) as f32 * DOT_WIDTH;
            let note_width: f32 = duration_get_headwidth(&note.0.duration) + dots_width;

            if let Some(placements) = note.0.get_heads_placements(&note.1.unwrap()) {
                for placement in placements {
                    let (level, place, head) = placement;

                    let rect: NRect = NRect::new(
                        (place.as_f32() * note_width) + (note_overlap),
                        level as f32 * SPACE_HALF - SPACE_HALF,
                        note_width,
                        SPACE,
                    );

                    rects.push(NRectExt(rect, NRectType::Head(note_head_type, note_shape)));
                }
            }
        }
        NoteType::Pause => {
            match note.0.duration {
                NV1 | NV1DOT => {
                    let rect = NRect::new(0., -SPACE + avoid_y_collision, SPACE, SPACE_HALF);
                    rects.push(NRectExt(rect, NRectType::Pause(&PauseShape::Whole)));
                }
                NV2 | NV2DOT | NV2TRI => {
                    let rect = NRect::new(0., -SPACE_HALF + avoid_y_collision, SPACE, SPACE_HALF);
                    rects.push(NRectExt(rect, NRectType::Pause(&PauseShape::Half)));
                }
                NV4 | NV4DOT | NV4TRI => {
                    let rect = NRect::new(0., -1.4 * SPACE + avoid_y_collision, SPACE, 2.8 * SPACE);
                    rects.push(NRectExt(rect, NRectType::Pause(&PauseShape::Quarter)));
                }
                NV8 | NV8DOT | NV8TRI => {
                    let rect = NRect::new(0., -SPACE + avoid_y_collision, SPACE, 2. * SPACE);
                    rects.push(NRectExt(rect, NRectType::Pause(&PauseShape::Eighth)));
                }
                NV16 | NV16DOT | NV16TRI => {
                    let rect = NRect::new(0., -SPACE + avoid_y_collision, SPACE * 1.3, 3. * SPACE);
                    rects.push(NRectExt(rect, NRectType::Pause(&PauseShape::Sixteenth)));
                }

                _ => {
                    let rect = NRect::new(0., -SPACE_HALF + avoid_y_collision, SPACE, SPACE);
                    rects.push(NRectExt(rect, NRectType::WIP("pause undefined")));
                }
            };
        }
        NoteType::Slash => {
            //
        }
        NoteType::Lyric(_) => {
            //
        }
        NoteType::Dynamic(_) => {
            //
        }
        NoteType::Chord(_) => {
            //
        }
        NoteType::Spacer => {
            //
        }
    }

    rects
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
                        ctype: ComplexType::OneNote(NoteExt(note, dir, placements)),
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
                                ComplexType::BarpauseNote(bp, NoteExt(note, dir, placements))
                            } else {
                                ComplexType::OneNote(NoteExt(note, dir, placements))
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
                                ComplexType::NoteBarpause(NoteExt(note, dir, placements), bp)
                            } else {
                                ComplexType::OneNote(NoteExt(note, dir, placements))
                            },
                        });
                        position += duration;
                    }
                }
                [VoiceType::VNotes(ref notes1), VoiceType::VNotes(ref notes2)] => {
                    let max_duration = notes1.duration.max(notes2.duration);
                    let min_duration = notes1.duration.min(notes2.duration);

                    let mut map1: HashMap<usize, &Note> = HashMap::new();
                    for np in notes1.get_notes_info() {
                        map1.insert(np.1, np.0);
                    }
                    let mut map2: HashMap<usize, &Note> = HashMap::new();
                    for np in notes2.get_notes_info() {
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
                                        NoteExt(note1, dir1, placements1),
                                        NoteExt(note2, dir2, placements2),
                                    ),
                                });
                            }
                            [Some(note), None] => {
                                let dir = map.get(note).unwrap().internal_direction;
                                let placements = note.get_heads_placements(&dir.unwrap());
                                complexes.push(Complex {
                                    position: *position,
                                    duration,
                                    ctype: ComplexType::OneNote(NoteExt(note, dir, placements)),
                                });
                            }
                            [None, Some(note)] => {
                                let dir = map.get(note).unwrap().internal_direction;
                                let placements = note.get_heads_placements(&dir.unwrap());
                                complexes.push(Complex {
                                    position: *position,
                                    duration,
                                    ctype: ComplexType::OneNote(NoteExt(note, dir, placements)),
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

impl ComplexNotesOverlap {
    pub fn as_f32(&self) -> f32 {
        match self {
            ComplexNotesOverlap::None => 0.0,
            ComplexNotesOverlap::UpperRight(f) => *f,
            ComplexNotesOverlap::LowerRight(f) => *f,
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
    use std::collections::HashMap;

    use crate::prelude::*;

    #[test]
    fn test1() {
        let voices = QCode::voices("Nv4 #0 % Nv8 b1 0 0").unwrap();
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
        let voices = QCode::voices("Nv4 0 0 % Nv8 0 0 0 0 0").unwrap();
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
        let voices = QCode::voices(" Nv4 0 0 0 % bp").unwrap();
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
        let voices = QCode::voices(" bp nv4 % bp nv8 nv8 nv8  ").unwrap();
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
