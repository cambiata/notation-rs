pub mod playback;
pub mod resolve;

use crate::prelude::*;
use itertools::Itertools;
use std::cell::{Ref, RefMut};
use std::collections::BTreeMap;
use std::hash::Hash;
use std::rc::Rc;

// use std::{cell::RefCell, collections::BTreeMap};

#[derive(Debug, PartialEq)]
// pub struct Bars(pub Vec<Rc<RefCell<Bar>>>);
pub struct Bars {
    pub items: Vec<Rc<RefCell<Bar>>>,
    pub matrix: Option<RMatrix>,
    pub id1_map: BTreeMap<usize, Rc<RefCell<Note>>>, // map note.id to Rc<RefCell<Note>>
    pub allotment_fn: SpacingFn,
}

impl Bars {
    pub fn new(items: Vec<Rc<RefCell<Bar>>>) -> Self {
        // calculate positions
        calculate_bar_positions(&items);
        Self {
            items,
            matrix: None,
            id1_map: BTreeMap::new(),
            allotment_fn: ALLOTMENT_EQUAL_FN,
        }
    }

    pub fn create_matrix(&mut self, bartemplate: Option<BarTemplate>) -> Result<()> {
        let bartemplate = match bartemplate {
            Some(bartemplate) => bartemplate,
            None => {
                return Err(Generic("Bartemplate is missing!".to_string()).into());
            }
        };

        let mut matrix_cols: Vec<Rc<RefCell<RCol>>> = vec![];

        for (baridx, bar) in self.items.iter().enumerate() {
            let bar = bar.borrow();

            match &bar.btype {
                BarType::Standard(parts) => {
                    for part in parts {
                        let mut part = part.borrow_mut();
                        part.setup_complexes()?;
                    }

                    let mut positions = vec![];
                    let mut parts_positions: Vec<BTreeMap<usize, usize>> = vec![];

                    let mut duration = 0;
                    for (partidx, part) in parts.iter().enumerate() {
                        let mut complex_positions: BTreeMap<usize, usize> = BTreeMap::new();

                        let mut part = part.borrow_mut();
                        for (complexidx, complex) in
                            part.complexes.as_ref().unwrap().iter().enumerate()
                        {
                            let mut complex = complex.borrow_mut();
                            positions.push(complex.position);
                            complex_positions.insert(complex.position, complexidx);
                        }
                        parts_positions.push(complex_positions);
                        duration = duration.max(part.duration);
                    }

                    positions.sort();
                    positions.dedup();

                    let mut positions2 = positions.clone();
                    positions2.push(duration);
                    let durations = positions2
                        .windows(2)
                        .map(|w| w[1] - w[0])
                        .collect::<Vec<_>>();

                    for (posidx, position) in positions.iter().enumerate() {
                        let mut colitems = vec![];
                        let mut colduration: Option<Duration> = None;

                        for (partidx, part) in parts.iter().enumerate() {
                            let complex_positions = &parts_positions[partidx];
                            let complexidx = complex_positions.get(&position);
                            let mut item: Option<Rc<RefCell<RItem>>> = None;

                            if let Some(complexidx) = complexidx {
                                let part = part.borrow();
                                let mut complex =
                                    part.complexes.as_ref().expect("This complex should exist!")
                                        [*complexidx]
                                        .borrow_mut();
                                let item_rects: Vec<NRect> =
                                    complex.rects.iter().map(|nrect| nrect.borrow().0).collect();
                                let item_nrects = complex
                                    .rects
                                    .iter()
                                    .map(|nrect| nrect.clone())
                                    .collect::<Vec<_>>();

                                let ritem = Rc::new(RefCell::new(RItem::new_from_nrects(
                                    RItemType::Content,
                                    item_nrects,
                                    complex.duration,
                                )));
                                complex.matrix_item = Some(ritem.clone());
                                item = Some(ritem);

                                colduration = Some(durations[posidx]);
                            }

                            colitems.push(item);
                        }

                        let rcol: RCol =
                            RCol::new(colitems, colduration, Some(bar.position + position));
                        matrix_cols.push(Rc::new(RefCell::new(rcol)));
                    }
                }

                BarType::MultiRest(_) => todo!(),
                BarType::NonContent(nctype) => match nctype {
                    NonContentType::VerticalLine => {
                        let mut colitems = vec![];
                        for parttemplate in bartemplate.0.iter() {
                            let (space_above, space_below) = match parttemplate {
                                PartTemplate::Music => {
                                    (VERTICAL_SPACE_ABOVE_MUSIC, VERTICAL_SPACE_BELOW_NOMUSIC)
                                }
                                PartTemplate::Nonmusic => {
                                    (VERTICAL_SPACE_ABOVE_NONMUSIC, VERTICAL_SPACE_BELOW_NONMUSIC)
                                }
                            };

                            let item = Some(Rc::new(RefCell::new(RItem::new_from_nrects(
                                RItemType::NonContent,
                                vec![Rc::new(RefCell::new(NRectExt::new(
                                    NRect::new(0., -space_above, 10., space_above + space_below),
                                    NRectType::WIP("VerticalLine".to_string()),
                                )))],
                                0,
                            ))));
                            colitems.push(item);
                        }
                        let rcol: RCol = RCol::new(colitems, None, Some(bar.position));
                        matrix_cols.push(Rc::new(RefCell::new(rcol)));
                    }
                    NonContentType::Barline(btype) => {
                        let mut colitems = vec![];
                        for parttemplate in bartemplate.0.iter() {
                            colitems.push(match parttemplate {
                                PartTemplate::Music => {
                                    let rects = match btype {
                                        BarlineType::Single => {
                                            vec![Rc::new(RefCell::new(NRectExt::new(
                                                NRect::new(0., -50., BARLINE_WIDTH_SINGLE, 100.),
                                                NRectType::Barline(btype.clone()),
                                            )))]
                                        }
                                        BarlineType::Double => {
                                            vec![Rc::new(RefCell::new(NRectExt::new(
                                                NRect::new(0., -50., BARLINE_DOUBLE_SPACE, 100.),
                                                NRectType::Barline(btype.clone()),
                                            )))]
                                        }
                                        BarlineType::Final => todo!(),
                                        BarlineType::RepeatTo => todo!(),
                                        BarlineType::RepeatFrom => todo!(),
                                        BarlineType::RepeatToAndFrom => todo!(),
                                        BarlineType::FraseTick => {
                                            vec![Rc::new(RefCell::new(NRectExt::new(
                                                NRect::new(0., -50., BARLINE_DOUBLE_SPACE, 100.),
                                                NRectType::Barline(btype.clone()),
                                            )))]
                                        }
                                    };

                                    Some(Rc::new(RefCell::new(RItem::new_from_nrects(
                                        RItemType::Barline,
                                        rects,
                                        0,
                                    ))))
                                }
                                PartTemplate::Nonmusic => None,
                            });
                        }
                        let rcol: RCol = RCol::new(colitems, None, Some(bar.position));
                        matrix_cols.push(Rc::new(RefCell::new(rcol)));
                    }
                    NonContentType::Spacer(width, height) => {
                        let mut colitems = vec![];
                        for parttemplate in bartemplate.0.iter() {
                            colitems.push(match parttemplate {
                                PartTemplate::Music => {
                                    Some(Rc::new(RefCell::new(RItem::new_from_nrects(
                                        RItemType::Space,
                                        vec![Rc::new(RefCell::new(NRectExt::new(
                                            NRect::new(0., -height / 2.0, *width, *height),
                                            NRectType::WIP("spacer".to_string()),
                                        )))],
                                        0,
                                    ))))
                                }
                                PartTemplate::Nonmusic => None,
                            });
                        }
                        let rcol: RCol = RCol::new(colitems, None, Some(bar.position));
                        matrix_cols.push(Rc::new(RefCell::new(rcol)));
                    }
                },
                BarType::BarAttribute(attribute) => match attribute {
                    BarAttributeType::Clefs(clefs) => {
                        let mut colitems = vec![];
                        for (clefidx, clefsig) in clefs.iter().enumerate() {
                            let mut item: Option<Rc<RefCell<RItem>>> = None;
                            let mut item_rects: Vec<NRect> = vec![];
                            if let Some(clefsig) = clefsig {
                                match clefsig {
                                    Some(clef) => {
                                        let (y, h) = match clef {
                                            Clef::G => (-90.0, 186.0),
                                            Clef::F => (-50.0, 84.0),
                                            Clef::C => (-50.0, 100.0),
                                        };

                                        item = Some(Rc::new(RefCell::new(RItem::new_from_nrects(
                                            RItemType::Clef,
                                            vec![Rc::new(RefCell::new(NRectExt::new(
                                                NRect::new(0., y, 74., h),
                                                NRectType::Clef(clef.clone()),
                                            )))],
                                            0,
                                        ))))
                                    }
                                    None => {
                                        //item_rects.push(NRect::new(0., -5.0, 10., 10.));
                                        item = Some(Rc::new(RefCell::new(RItem::new_from_nrects(
                                            RItemType::Other,
                                            vec![Rc::new(RefCell::new(NRectExt::new(
                                                NRect::new(0., -5.0, 10., 10.),
                                                NRectType::WIP("no clef".to_string()),
                                            )))],
                                            0,
                                        ))))
                                    }
                                }
                            } else {
                            }
                            colitems.push(item);
                        }
                        let rcol: RCol = RCol::new(colitems, None, Some(bar.position));
                        matrix_cols.push(Rc::new(RefCell::new(rcol)));
                    }

                    BarAttributeType::Keys(items) => {
                        let mut ritems = vec![];
                        for (idx, sig) in items.iter().enumerate() {
                            let mut item: Option<Rc<RefCell<RItem>>> = None;
                            // let mut item_rects: Vec<NRect> = vec![];
                            if let Some(sig) = sig {
                                let mut nrects = Vec::new();
                                match sig {
                                    Some(key) => {
                                        //
                                        let mut x = 0.0;
                                        match key {
                                            Key::Sharps(ref n, ref key_clef) => {
                                                let clef_y: f32 = match key_clef {
                                                    Clef::G => 0.0,
                                                    Clef::F => SPACE,
                                                    Clef::C => SPACE_HALF,
                                                };
                                                nrects.push(Rc::new(RefCell::new(NRectExt::new(
                                                    NRect::new(
                                                        x,
                                                        -SPACE * 3.5 + clef_y,
                                                        *n as f32 * ACCIDENTAL_WIDTH_SHARP,
                                                        6.0 * SPACE,
                                                    ),
                                                    NRectType::KeySignature(key.clone(), None),
                                                ))));
                                            }

                                            Key::Flats(n, ref key_clef) => {
                                                let clef_y: f32 = match key_clef {
                                                    Clef::G => 0.0,
                                                    Clef::F => SPACE,
                                                    Clef::C => SPACE_HALF,
                                                };

                                                nrects.push(Rc::new(RefCell::new(NRectExt::new(
                                                    NRect::new(
                                                        x,
                                                        -SPACE * 3.5 + clef_y,
                                                        *n as f32 * ACCIDENTAL_WIDTH_FLAT,
                                                        6.0 * SPACE,
                                                    ),
                                                    NRectType::KeySignature(key.clone(), None),
                                                ))));
                                            }
                                            Key::Open => {}
                                            Key::Naturals(n, ref key_clef) => {
                                                todo!("Key::Naturals not defined yet!")
                                            }
                                        }
                                    }
                                    None => {
                                        let nrect = Rc::new(RefCell::new(NRectExt::new(
                                            NRect::new(0., -5.0, 10., 10.),
                                            NRectType::WIP("no key".to_string()),
                                        )));
                                        nrects.push(nrect);
                                    }
                                }

                                item = Some(Rc::new(RefCell::new(RItem::new_from_nrects(
                                    RItemType::Key,
                                    nrects,
                                    0,
                                ))));
                            } else {
                            }
                            ritems.push(item);
                        }
                        let rcol: RCol = RCol::new(ritems, None, Some(bar.position));
                        matrix_cols.push(Rc::new(RefCell::new(rcol)));
                    }

                    BarAttributeType::Times(items) => {
                        let mut ritems = vec![];
                        for (idx, sig) in items.iter().enumerate() {
                            let mut item: Option<Rc<RefCell<RItem>>> = None;
                            if let Some(sig) = sig {
                                let mut nrects = Vec::new();
                                match sig {
                                    Some(time) => {
                                        //
                                        nrects.push(Rc::new(RefCell::new(NRectExt::new(
                                            NRect::new(
                                                0.0,
                                                -SPACE * 3.0,
                                                TIME_SIGNATURE_WIDTH,
                                                6.0 * SPACE,
                                            ),
                                            NRectType::TimeSignature(time.clone()),
                                        ))));
                                    }
                                    None => {
                                        let nrect = Rc::new(RefCell::new(NRectExt::new(
                                            NRect::new(0., -5.0, 10., 10.),
                                            NRectType::WIP("no key".to_string()),
                                        )));
                                        nrects.push(nrect);
                                    }
                                }

                                item = Some(Rc::new(RefCell::new(RItem::new_from_nrects(
                                    RItemType::Time,
                                    nrects,
                                    0,
                                ))));
                            } else {
                            }
                            ritems.push(item);
                        }
                        let rcol: RCol = RCol::new(ritems, None, Some(bar.position));
                        matrix_cols.push(Rc::new(RefCell::new(rcol)));
                    }
                },

                BarType::CountIn(_) => {
                    //
                }
            }
        }

        // for col in &matrix_cols {
        //     let col = col.borrow();
        //     dbg!(col.position);
        // }

        // self.map_id1_to_note();
        // self.resolve_ties();
        // self.resolve_slurs();

        // self.resolve_stuff();
        self.matrix = Some(RMatrix::new(matrix_cols, Some(bartemplate)));
        // Ok(matrix)
        Ok(())
    }

    pub fn matrix_add_beamgroups(&self) {
        for (baridx, bar) in self.items.iter().enumerate() {
            let bar = bar.borrow();
            match bar.btype {
                BarType::Standard(ref parts) => {
                    for part in parts {
                        let part = part.borrow();
                        let complexes = part
                            .complexes
                            .as_ref()
                            .expect("Part should have complexes!");

                        let mut note_current_beamgroup_id: usize = 0;
                        let mut note_current_beamgroup_id1x: usize = 0;
                        let mut note2_current_beamgroup_id: usize = 0;
                        let mut note2_current_beamgroup_id1x: usize = 0;

                        for (complexidx, complex) in complexes.iter().enumerate() {
                            let complex = complex.borrow();

                            if let Some(item) = &complex.matrix_item {
                                let mut item = item.borrow_mut();

                                let note = match &complex.ctype {
                                    ComplexType::Single(ref note, _)
                                    | ComplexType::Upper(ref note, _) => Some(note),
                                    ComplexType::Two(ref note, _, _) => Some(note),
                                    _ => None,
                                };

                                let note2 = match &complex.ctype {
                                    ComplexType::Two(_, ref note2, _) => Some(note2),
                                    ComplexType::Lower(ref note2, _) => Some(note2),
                                    _ => None,
                                };

                                let adjust_x = match &complex.ctype {
                                    ComplexType::Two(_, _, adjust_x) => adjust_x,
                                    _ => &None,
                                };

                                // NOTE 1 ------------------------------------------------------------------------------

                                if let Some(note) = note {
                                    let note = note.borrow();
                                    if note.is_heads() {
                                        let beamgroup_ref = note
                                            .beamgroup
                                            .as_ref()
                                            .expect("Single note should have beamgroup!")
                                            .clone();
                                        let beamgroup = beamgroup_ref.borrow();
                                        if beamgroup.id != note_current_beamgroup_id {
                                            note_current_beamgroup_id = beamgroup.id;
                                            note_current_beamgroup_id1x = 0;

                                            let data = RItemBeamData {
                                                id: beamgroup.id,
                                                id1: note.id,
                                                note_position: note.position,
                                                direction: beamgroup.direction.unwrap(),
                                                tip_level: beamgroup.start_level,
                                                duration: note.duration,
                                                top_level: note.top_level(),
                                                bottom_level: note.bottom_level(),
                                                has_stem: note.has_stem(),
                                                adjustment_x: *adjust_x,
                                                head_width: duration_get_headwidth(&note.duration),
                                                note_durations: None,
                                                lower_voice: false,
                                            };

                                            if beamgroup.notes.len() == 1 {
                                                item.notedata.beamdata1 = RItemBeam::Single(data);
                                            } else {
                                                item.notedata.beamdata1 = RItemBeam::Start(data);
                                            }
                                        } else {
                                            note_current_beamgroup_id1x += 1;

                                            let mut data = RItemBeamData {
                                                id: beamgroup.id,
                                                id1: note.id,
                                                note_position: note.position,
                                                direction: beamgroup.direction.unwrap(),
                                                tip_level: beamgroup.end_level,
                                                duration: note.duration,
                                                top_level: note.top_level(),
                                                bottom_level: note.bottom_level(),
                                                has_stem: note.has_stem(),
                                                adjustment_x: *adjust_x,
                                                head_width: duration_get_headwidth(&note.duration),
                                                note_durations: None,
                                                lower_voice: false,
                                            };

                                            if note_current_beamgroup_id1x
                                                < beamgroup.notes.len() - 1
                                            {
                                                item.notedata.beamdata1 = RItemBeam::Middle(data);
                                            } else {
                                                data.note_durations =
                                                    Some(beamgroup.note_durations.clone());
                                                item.notedata.beamdata1 = RItemBeam::End(data);
                                            }
                                        }
                                    }
                                }

                                // NOTE 2 ------------------------------------------------------------------------------

                                if let Some(note2) = note2 {
                                    let note2 = note2.borrow();
                                    if note2.is_heads() {
                                        let beamgroup_ref = note2
                                            .beamgroup
                                            .as_ref()
                                            .expect("Lower note should have beamgroup!")
                                            .clone();
                                        let beamgroup = beamgroup_ref.borrow();
                                        if beamgroup.id != note2_current_beamgroup_id {
                                            note2_current_beamgroup_id = beamgroup.id;
                                            note2_current_beamgroup_id1x = 0;

                                            let data = RItemBeamData {
                                                id: beamgroup.id,
                                                id1: note2.id,
                                                note_position: note2.position,
                                                direction: beamgroup.direction.unwrap(),
                                                tip_level: beamgroup.start_level,
                                                duration: note2.duration,
                                                top_level: note2.top_level(),
                                                bottom_level: note2.bottom_level(),
                                                has_stem: note2.has_stem(),
                                                adjustment_x: *adjust_x,
                                                head_width: duration_get_headwidth(&note2.duration),
                                                note_durations: None,
                                                lower_voice: true,
                                            };

                                            if beamgroup.notes.len() == 1 {
                                                item.notedata.beamdata2 = RItemBeam::Single(data);
                                            } else {
                                                item.notedata.beamdata2 = RItemBeam::Start(data);
                                            }
                                        } else {
                                            let mut data = RItemBeamData {
                                                id: beamgroup.id,
                                                id1: note2.id,
                                                note_position: note2.position,
                                                direction: beamgroup.direction.unwrap(),
                                                tip_level: beamgroup.end_level,
                                                duration: note2.duration,
                                                top_level: note2.top_level(),
                                                bottom_level: note2.bottom_level(),
                                                has_stem: note2.has_stem(),
                                                adjustment_x: *adjust_x,
                                                head_width: duration_get_headwidth(&note2.duration),
                                                note_durations: None,
                                                lower_voice: true,
                                            };

                                            note2_current_beamgroup_id1x += 1;
                                            if note2_current_beamgroup_id1x
                                                < beamgroup.notes.len() - 1
                                            {
                                                item.notedata.beamdata2 = RItemBeam::Middle(data);
                                            } else {
                                                data.note_durations =
                                                    Some(beamgroup.note_durations.clone());
                                                item.notedata.beamdata2 = RItemBeam::End(data);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    pub fn matrix_add_lines(&self) {
        for (baridx, bar) in self.items.iter().enumerate() {
            let bar = bar.borrow();
            match bar.btype {
                BarType::Standard(ref parts) => {
                    for part in parts {
                        let part = part.borrow();
                        let complexes: &Vec<Rc<RefCell<Complex>>> = part
                            .complexes
                            .as_ref()
                            .expect("Part should have complexes!");
                        for (complexidx, complex) in complexes.iter().enumerate() {
                            let complex = complex.borrow();
                            if let Some(item) = &complex.matrix_item {
                                let mut item: RefMut<RItem> = item.borrow_mut();
                                println!("complexidx:{}", complexidx);
                                match &complex.ctype {
                                    ComplexType::Single(note, _)
                                    | ComplexType::Upper(note, _)
                                    | ComplexType::Lower(note, _) => {
                                        let note = note.borrow();
                                        if note.is_heads() {
                                            let (head_width, adjust_x) = note.adjust_x.unwrap();
                                            for line in &note.lines_to {
                                                dbg!(line);
                                                item.lines.push((line.0, line.1, line.2.clone()));
                                            }
                                        }
                                    }
                                    ComplexType::Two(note, note2, adjust) => {
                                        let note = note.borrow();
                                        if note.is_heads() {
                                            let (head_width, adjust_x) = note.adjust_x.unwrap();
                                            for line in &note.lines_to {
                                                item.lines.push((line.0, line.1, line.2.clone()));
                                            }
                                        }

                                        let note2 = note2.borrow();
                                        if note2.is_heads() {
                                            let (head_width, adjust_x) = note2.adjust_x.unwrap();
                                            for line in &note2.lines_to {
                                                item.lines.push((line.0, line.1, line.2.clone()));
                                            }
                                        }
                                    }
                                    ComplexType::OneBarpause(_) => {}
                                    ComplexType::TwoBarpauses(_, _) => {}
                                }
                            }
                        }
                    }
                }
                _ => {
                    //
                }
            }
        }
    }

    pub fn matrix_add_ties(&self) {
        for (baridx, bar) in self.items.iter().enumerate() {
            let bar = bar.borrow();
            match bar.btype {
                BarType::Standard(ref parts) => {
                    for part in parts {
                        let part = part.borrow();
                        let complexes: &Vec<Rc<RefCell<Complex>>> = part
                            .complexes
                            .as_ref()
                            .expect("Part should have complexes!");

                        for complex in complexes {
                            let complex = complex.borrow();

                            if let Some(item) = &complex.matrix_item {
                                let mut item: RefMut<RItem> = item.borrow_mut();

                                match &complex.ctype {
                                    ComplexType::OneBarpause(_) => {}
                                    ComplexType::TwoBarpauses(_, _) => {}
                                    ComplexType::Single(note, _)
                                    | ComplexType::Upper(note, _)
                                    | ComplexType::Lower(note, _) => {
                                        let note = note.borrow();
                                        if note.is_heads() {
                                            let note_direction = note.direction.unwrap();
                                            let (head_width, adjust_x) = note.adjust_x.unwrap();
                                            let ties_count = &note.ties.len();
                                            let mut tieidx = 0;

                                            for tie in &note.ties {
                                                let tie_direction = match &complex.ctype {
                                                    ComplexType::Single(_, _) => {
                                                        if note.ties.len() == 1 {
                                                            note_direction.flip()
                                                        } else if tieidx < &note.ties.len() / 2 {
                                                            DirUD::Up
                                                        } else {
                                                            DirUD::Down
                                                        }
                                                    }
                                                    ComplexType::Upper(_, _) => DirUD::Up,
                                                    ComplexType::Lower(_, _) => DirUD::Down,
                                                    ComplexType::Two(_, _, _) => todo!(), // shouldn't matter!
                                                    ComplexType::OneBarpause(_) => todo!(),
                                                    ComplexType::TwoBarpauses(_, _) => todo!(),
                                                };

                                                let tie_placement = match &complex.ctype {
                                                    ComplexType::Single(_, _) => {
                                                        if note.ties.len() == 1 {
                                                            match note_direction {
                                                                DirUD::Up => TiePlacement::Bottom,
                                                                DirUD::Down => TiePlacement::Top,
                                                            }
                                                        } else if tieidx == 0 {
                                                            TiePlacement::Top
                                                        } else if tieidx == ties_count - 1 {
                                                            TiePlacement::Bottom
                                                        } else {
                                                            TiePlacement::Mid
                                                        }
                                                    }
                                                    ComplexType::Upper(_, _) => {
                                                        if tieidx == 0 {
                                                            TiePlacement::Top
                                                        } else {
                                                            TiePlacement::Mid
                                                        }
                                                    }
                                                    ComplexType::Lower(_, _) => {
                                                        if tieidx == ties_count - 1 {
                                                            TiePlacement::Bottom
                                                        } else {
                                                            TiePlacement::Mid
                                                        }
                                                    }

                                                    ComplexType::Two(_, _, _) => todo!(),
                                                    ComplexType::OneBarpause(_) => todo!(),
                                                    ComplexType::TwoBarpauses(_, _) => todo!(),
                                                };

                                                let rect: NRect = NRect::new(
                                                    adjust_x + head_width,
                                                    0.0 + tie.level as f32 * SPACE_HALF
                                                        - TIE_SPACE_HALF,
                                                    TIE_FROM_WIDTH,
                                                    TIE_SPACE,
                                                );
                                                let nrects = item.nrects.as_mut();
                                                if nrects.is_none() {
                                                    item.nrects = Some(vec![]);
                                                }
                                                item.nrects.as_mut().unwrap().push(Rc::new(
                                                    RefCell::new(NRectExt(
                                                        rect,
                                                        NRectType::TieFrom(
                                                            note.id,
                                                            tie.level,
                                                            tie.ttype.clone(),
                                                            note.duration,
                                                            note_direction,
                                                            tie_direction,
                                                            tie_placement,
                                                        ),
                                                    )),
                                                ));
                                                tieidx += 1;
                                            }

                                            for tie_to in &note.ties_to {
                                                let rect: NRect = NRect::new(
                                                    -TIE_TO_WIDTH,
                                                    0.0 + tie_to.level as f32 * SPACE_HALF
                                                        - TIE_SPACE_HALF,
                                                    TIE_TO_WIDTH,
                                                    TIE_SPACE,
                                                );
                                                let nrects = item.nrects.as_mut();
                                                if nrects.is_none() {
                                                    item.nrects = Some(vec![]);
                                                }
                                                item.nrects.as_mut().unwrap().push(Rc::new(
                                                    RefCell::new(NRectExt(
                                                        rect,
                                                        NRectType::TieTo(tie_to.ttype.clone()),
                                                    )),
                                                ));
                                            }
                                        }
                                        //
                                    }

                                    ComplexType::Two(note, note2, adjust) => {
                                        // upper
                                        let note = note.borrow();
                                        if note.is_heads() {
                                            let note_direction = note.direction.unwrap();
                                            let (head_width, adjust_x) = note.adjust_x.unwrap();

                                            let ties_count = &note.ties.len();
                                            let mut tieidx = 0;
                                            for tie in &note.ties {
                                                let rect: NRect = NRect::new(
                                                    adjust_x + head_width,
                                                    0.0 + tie.level as f32 * SPACE_HALF
                                                        - TIE_SPACE_HALF,
                                                    TIE_FROM_WIDTH,
                                                    TIE_SPACE,
                                                );
                                                let nrects = item.nrects.as_mut();
                                                if nrects.is_none() {
                                                    item.nrects = Some(vec![]);
                                                }
                                                let tie_direction = DirUD::Up;
                                                let tie_placement = if tieidx == 0 {
                                                    TiePlacement::Top
                                                } else {
                                                    TiePlacement::Mid
                                                };
                                                item.nrects.as_mut().unwrap().push(Rc::new(
                                                    RefCell::new(NRectExt(
                                                        rect,
                                                        NRectType::TieFrom(
                                                            note.id,
                                                            tie.level,
                                                            tie.ttype.clone(),
                                                            note.duration,
                                                            note_direction,
                                                            tie_direction,
                                                            tie_placement,
                                                        ),
                                                    )),
                                                ));
                                            }

                                            for tie_to in &note.ties_to {
                                                let rect: NRect = NRect::new(
                                                    adjust_x - TIE_TO_WIDTH,
                                                    0.0 + tie_to.level as f32 * SPACE_HALF
                                                        - TIE_SPACE_HALF,
                                                    TIE_TO_WIDTH,
                                                    TIE_SPACE,
                                                );
                                                let nrects = item.nrects.as_mut();
                                                if nrects.is_none() {
                                                    item.nrects = Some(vec![]);
                                                }
                                                item.nrects.as_mut().unwrap().push(Rc::new(
                                                    RefCell::new(NRectExt(
                                                        rect,
                                                        NRectType::TieTo(tie_to.ttype.clone()),
                                                    )),
                                                ));
                                            }
                                        };

                                        // lower
                                        let note2 = note2.borrow();
                                        if note2.is_heads() {
                                            let note_direction = note2.direction.unwrap();
                                            let (head_width, adjust_x) = note2.adjust_x.unwrap();

                                            let ties_count = &note2.ties.len();
                                            let mut tieidx = 0;
                                            for tie in &note2.ties {
                                                let rect: NRect = NRect::new(
                                                    adjust_x + head_width,
                                                    0.0 + tie.level as f32 * SPACE_HALF
                                                        - TIE_SPACE_HALF,
                                                    TIE_FROM_WIDTH,
                                                    TIE_SPACE,
                                                );
                                                let nrects = item.nrects.as_mut();
                                                if nrects.is_none() {
                                                    item.nrects = Some(vec![]);
                                                }

                                                let tie_direction = DirUD::Down;
                                                let tie_placement = if tieidx == ties_count - 1 {
                                                    TiePlacement::Bottom
                                                } else {
                                                    TiePlacement::Mid
                                                };

                                                item.nrects.as_mut().unwrap().push(Rc::new(
                                                    RefCell::new(NRectExt(
                                                        rect,
                                                        NRectType::TieFrom(
                                                            note2.id,
                                                            tie.level,
                                                            tie.ttype.clone(),
                                                            note.duration,
                                                            note_direction,
                                                            tie_direction,
                                                            tie_placement,
                                                        ),
                                                    )),
                                                ));
                                            }

                                            for tie_to in &note2.ties_to {
                                                let rect: NRect = NRect::new(
                                                    adjust_x + -TIE_TO_WIDTH,
                                                    0.0 + tie_to.level as f32 * SPACE_HALF
                                                        - TIE_SPACE_HALF,
                                                    TIE_TO_WIDTH,
                                                    TIE_SPACE,
                                                );
                                                let nrects = item.nrects.as_mut();
                                                if nrects.is_none() {
                                                    item.nrects = Some(vec![]);
                                                }
                                                item.nrects.as_mut().unwrap().push(Rc::new(
                                                    RefCell::new(NRectExt(
                                                        rect,
                                                        NRectType::TieTo(tie_to.ttype.clone()),
                                                    )),
                                                ));
                                            }
                                        };
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    pub fn parts_count(&self) -> Option<usize> {
        let mut part_count = 0;

        for (baridx, bar) in self.items.iter().enumerate() {
            let bar = bar.borrow();
            match bar.btype {
                BarType::Standard(ref parts) => {
                    for (partidx, part) in parts.iter().enumerate() {
                        part_count += 1;
                    }
                }
                _ => {}
            }
        }
        if part_count == 0 {
            println!("No parts were found! parts_count == 0");
            return None;
        }
        Some(part_count)
    }

    pub fn consecutive_note_chunks(&self) -> Vec<(usize, usize, NotesChunk)> {
        // (partidx, voiceidx, notes)
        let parts_count = self.parts_count().unwrap_or(0);

        let mut chunk_indexes: BTreeMap<usize, usize> = BTreeMap::new();
        let mut chunk_notes: BTreeMap<usize, Vec<Rc<RefCell<Note>>>> = BTreeMap::new();

        for partidx in 0..parts_count {
            let part_voice_id = (partidx + 1) * 1_000_000;
            chunk_indexes.insert(part_voice_id, 0);
            let part_voice_id = (partidx + 1) * 1_000_000 + 1_000;
            chunk_indexes.insert(part_voice_id, 0);
        }

        for (baridx, bar) in self.items.iter().enumerate() {
            let bar = bar.borrow();
            match bar.btype {
                BarType::Standard(ref parts) => {
                    for (partidx, part) in parts.iter().enumerate() {
                        let mut part = part.borrow_mut();
                        match part.ptype {
                            PartType::Music(ref pmtype) => {
                                match pmtype {
                                    PartMusicType::Voices(ref voices) => match voices {
                                        Voices::One(ref voice) => {
                                            let voice = voice.borrow();
                                            let part_voice_id = (partidx + 1) * 1_000_000;
                                            if !chunk_indexes.contains_key(&part_voice_id) {
                                                chunk_indexes.insert(part_voice_id, 0);
                                            }

                                            match voice.vtype {
                                                VoiceType::Notes(ref notes) => {
                                                    for (noteidx, note) in
                                                        notes.items.iter().enumerate()
                                                    {
                                                        let part_voice_current_id = part_voice_id
                                                            + *chunk_indexes
                                                                .get(&part_voice_id)
                                                                .unwrap();

                                                        let note_ = note.borrow();
                                                        if !note_.is_pause() {
                                                            // println!("Note {part_voice_id}/{part_voice_current_id}: {partidx}:{}", 0);

                                                            if !chunk_notes.contains_key(
                                                                &part_voice_current_id,
                                                            ) {
                                                                chunk_notes.insert(
                                                                    part_voice_current_id,
                                                                    vec![note.clone()],
                                                                );
                                                            } else {
                                                                chunk_notes
                                                                    .get_mut(&part_voice_current_id)
                                                                    .unwrap()
                                                                    .push(note.clone());
                                                            }
                                                        } else {
                                                            // println!("Note {part_voice_id} not heads! - increase chunk_indexe");
                                                            chunk_indexes.insert(
                                                                part_voice_id,
                                                                chunk_indexes
                                                                    .get(&part_voice_id)
                                                                    .unwrap()
                                                                    + 1,
                                                            );
                                                        }
                                                    }
                                                }
                                                _ => {
                                                    // println!("{partidx}:{} Not VoiceType::Notes", 0);
                                                    chunk_indexes.insert(
                                                        part_voice_id,
                                                        chunk_indexes.get(&part_voice_id).unwrap()
                                                            + 1,
                                                    );
                                                    let part_voice2_id =
                                                        (partidx + 1) * 1_000_000 + 1_000;
                                                    chunk_indexes.insert(
                                                        part_voice2_id,
                                                        chunk_indexes.get(&part_voice2_id).unwrap()
                                                            + 1,
                                                    );
                                                }
                                            }
                                        }
                                        Voices::Two(ref upper, ref lower) => {
                                            let upper = upper.borrow();
                                            let part_voice_id = (partidx + 1) * 1_000_000;
                                            if !chunk_indexes.contains_key(&part_voice_id) {
                                                chunk_indexes.insert(part_voice_id, 0);
                                            }

                                            match upper.vtype {
                                                VoiceType::Notes(ref notes) => {
                                                    for (noteidx, note) in
                                                        notes.items.iter().enumerate()
                                                    {
                                                        let note_ = note.borrow_mut();
                                                        let part_voice_current_id = part_voice_id
                                                            + *chunk_indexes
                                                                .get(&part_voice_id)
                                                                .unwrap();
                                                        if !note_.is_pause() {
                                                            // println!("Note {part_voice_id}/{part_voice_current_id}: {partidx}:{}", 0);
                                                            if !chunk_notes.contains_key(
                                                                &part_voice_current_id,
                                                            ) {
                                                                chunk_notes.insert(
                                                                    part_voice_current_id,
                                                                    vec![note.clone()],
                                                                );
                                                            } else {
                                                                chunk_notes
                                                                    .get_mut(&part_voice_current_id)
                                                                    .unwrap()
                                                                    .push(note.clone());
                                                            }
                                                        } else {
                                                            // println!("Note not heads!");
                                                            chunk_indexes.insert(
                                                                part_voice_id,
                                                                chunk_indexes
                                                                    .get(&part_voice_id)
                                                                    .unwrap()
                                                                    + 1,
                                                            );
                                                        }
                                                    }
                                                }
                                                _ => {
                                                    // println!("{partidx}:{} Not VoiceType::Notes", 0);
                                                    chunk_indexes.insert(
                                                        part_voice_id,
                                                        chunk_indexes.get(&part_voice_id).unwrap()
                                                            + 1,
                                                    );
                                                }
                                            }

                                            let lower = lower.borrow();
                                            let part_voice_id = (partidx + 1) * 1_000_000 + 1_000;
                                            if !chunk_indexes.contains_key(&part_voice_id) {
                                                chunk_indexes.insert(part_voice_id, 0);
                                            }

                                            match lower.vtype {
                                                VoiceType::Notes(ref notes) => {
                                                    for (noteidx, note) in
                                                        notes.items.iter().enumerate()
                                                    {
                                                        let note_ = note.borrow_mut();
                                                        let part_voice_current_id = part_voice_id
                                                            + *chunk_indexes
                                                                .get(&part_voice_id)
                                                                .unwrap();
                                                        if !note_.is_pause() {
                                                            // println!("Note {part_voice_id}/{part_voice_current_id}: {partidx}:{}", 1);
                                                            if !chunk_notes.contains_key(
                                                                &part_voice_current_id,
                                                            ) {
                                                                chunk_notes.insert(
                                                                    part_voice_current_id,
                                                                    vec![note.clone()],
                                                                );
                                                            } else {
                                                                chunk_notes
                                                                    .get_mut(&part_voice_current_id)
                                                                    .unwrap()
                                                                    .push(note.clone());
                                                            }
                                                        } else {
                                                            // println!("Note {part_voice_id} not heads!");
                                                            chunk_indexes.insert(
                                                                part_voice_id,
                                                                chunk_indexes
                                                                    .get(&part_voice_id)
                                                                    .unwrap()
                                                                    + 1,
                                                            );
                                                        }
                                                    }
                                                }
                                                _ => {
                                                    // println!("{partidx}:{} Not VoiceType::Notes", 1);
                                                    chunk_indexes.insert(
                                                        part_voice_id,
                                                        chunk_indexes.get(&part_voice_id).unwrap()
                                                            + 1,
                                                    );
                                                }
                                            }
                                            //
                                        }
                                    },

                                    _ => {
                                        // println!("{partidx} Not PartMusicType::Voices");
                                    }
                                }
                            }
                            _ => {
                                // println!("{partidx} Not PartType::Music");
                            }
                        }
                    }
                    //
                }
                BarType::MultiRest(_) => {
                    for partidx in 0..parts_count {
                        let part_voice_id = (partidx + 1) * 1_000_000;
                        chunk_indexes.insert(
                            part_voice_id,
                            chunk_indexes.get(&part_voice_id).unwrap() + 1,
                        );
                        let part_voice_id = (partidx + 1) * 1_000_000 + 1_000;
                        chunk_indexes.insert(
                            part_voice_id,
                            chunk_indexes.get(&part_voice_id).unwrap() + 1,
                        );
                    }
                }

                BarType::NonContent(_) => {
                    // println!("BarType::NonContent +  this should NOT cause new chunks!");
                }
                BarType::BarAttribute(_) => {}

                BarType::CountIn(_) => {}
            }
        }

        let mut result: Vec<(usize, usize, NotesChunk)> = Vec::new();
        for (key, value) in chunk_notes {
            let partidx = key / 1000000;
            let voiceidx = (key - 1000000) / 1000;
            // dbg!(key, partidx, voiceidx);
            result.push((partidx, voiceidx, value));
        }
        result
    }
}

fn calculate_bar_positions(items: &Vec<Rc<RefCell<Bar>>>) {
    let mut position = 0;
    for item in items {
        let mut item = item.borrow_mut();
        let duration = item.duration;
        item.position = position;
        position += duration;
    }
}
