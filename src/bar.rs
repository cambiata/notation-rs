use itertools::Itertools;
use std::cell::{Ref, RefMut};
use std::collections::BTreeMap;
use std::hash::Hash;
use std::rc::Rc;
use std::{cell::RefCell, collections::HashMap};

use crate::{complex, part, prelude::*};

#[derive(Debug, PartialEq)]
// pub struct Bars(pub Vec<Rc<RefCell<Bar>>>);
pub struct Bars {
    pub items: Vec<Rc<RefCell<Bar>>>,
    pub matrix: Option<RMatrix>,
}

impl Bars {
    pub fn new(items: Vec<Rc<RefCell<Bar>>>) -> Self {
        Self { items, matrix: None }
    }

    pub fn create_matrix(&self, bartemplate: Option<BarTemplate>) -> Result<RMatrix> {
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
                    let mut parts_positions: Vec<HashMap<usize, usize>> = vec![];

                    let mut duration = 0;
                    for (partidx, part) in parts.iter().enumerate() {
                        let mut complex_positions: HashMap<usize, usize> = HashMap::new();

                        let mut part = part.borrow_mut();
                        for (complexidx, complex) in part.complexes.as_ref().unwrap().iter().enumerate() {
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
                    let durations = positions2.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();

                    for (posidx, position) in positions.iter().enumerate() {
                        let mut colitems = vec![];
                        let mut colduration: Option<Duration> = None;

                        for (partidx, part) in parts.iter().enumerate() {
                            let complex_positions = &parts_positions[partidx];
                            let complexidx = complex_positions.get(&position);
                            let mut item: Option<Rc<RefCell<RItem>>> = None;

                            if let Some(complexidx) = complexidx {
                                let part = part.borrow();
                                let mut complex = part.complexes.as_ref().expect("This complex should exist!")[*complexidx].borrow_mut();
                                let item_rects: Vec<NRect> = complex.rects.iter().map(|nrect| nrect.borrow().0).collect();
                                let item_nrects = complex.rects.iter().map(|nrect| nrect.clone()).collect::<Vec<_>>();
                                let ritem = Rc::new(RefCell::new(RItem::new_from_nrects(item_nrects, complex.duration)));
                                complex.matrix_item = Some(ritem.clone());
                                item = Some(ritem);

                                colduration = Some(durations[posidx]);
                            }

                            colitems.push(item);
                        }
                        let rcol: RCol = RCol::new(colitems, colduration);
                        matrix_cols.push(Rc::new(RefCell::new(rcol)));
                    }
                }

                BarType::MultiRest(_) => todo!(),
                BarType::NonContent(nctype) => match nctype {
                    NonContentType::VerticalLine => {
                        let mut colitems = vec![];
                        for parttemplate in bartemplate.0.iter() {
                            let item = Some(Rc::new(RefCell::new(RItem::new_from_nrects(
                                vec![Rc::new(RefCell::new(NRectExt::new(NRect::new(0., -5.0, 10., 10.), NRectType::WIP("VerticalLine".to_string()))))],
                                0,
                            ))));
                            colitems.push(item);
                        }
                        let rcol: RCol = RCol::new(colitems, None);
                        matrix_cols.push(Rc::new(RefCell::new(rcol)));
                    }
                    NonContentType::Barline => {
                        let mut colitems = vec![];
                        for parttemplate in bartemplate.0.iter() {
                            colitems.push(match parttemplate {
                                PartTemplate::Music => Some(Rc::new(RefCell::new(RItem::new_from_nrects(
                                    vec![Rc::new(RefCell::new(NRectExt::new(NRect::new(0., -30.0, 5., 60.), NRectType::WIP("barline".to_string()))))],
                                    0,
                                )))),
                                PartTemplate::Nonmusic => None,
                            });
                        }
                        let rcol: RCol = RCol::new(colitems, None);
                        matrix_cols.push(Rc::new(RefCell::new(rcol)));
                    }

                    NonContentType::Clefs(clefs) => {
                        let mut colitems = vec![];
                        for (clefidx, clefsig) in clefs.iter().enumerate() {
                            let mut item: Option<Rc<RefCell<RItem>>> = None;
                            let mut item_rects: Vec<NRect> = vec![];
                            if let Some(clefsig) = clefsig {
                                match clefsig {
                                    Some(clef) => {
                                        let (y, h) = match clef {
                                            Clef::G => (-116.0, 186.0),
                                            Clef::F => (-50.0, 84.0),
                                            Clef::C => (-50.0, 100.0),
                                        };

                                        item = Some(Rc::new(RefCell::new(RItem::new_from_nrects(
                                            vec![Rc::new(RefCell::new(NRectExt::new(NRect::new(0., y, 74., h), NRectType::Clef(clef.clone()))))],
                                            0,
                                        ))))
                                    }
                                    None => {
                                        //item_rects.push(NRect::new(0., -5.0, 10., 10.));
                                        item = Some(Rc::new(RefCell::new(RItem::new_from_nrects(
                                            vec![Rc::new(RefCell::new(NRectExt::new(NRect::new(0., -5.0, 10., 10.), NRectType::WIP("no clef".to_string()))))],
                                            0,
                                        ))))
                                    }
                                }
                            } else {
                            }
                            colitems.push(item);
                        }
                        let rcol: RCol = RCol::new(colitems, None);
                        matrix_cols.push(Rc::new(RefCell::new(rcol)));
                    }
                },
            }
        }

        let matrix = RMatrix::new(matrix_cols, Some(bartemplate));

        // for (rowidx, row) in matrix.rows.iter().enumerate() {
        //     //
        //     let row = row.borrow();
        //     let template = bartemplate.0.get(rowidx).unwrap();
        //     match template {
        //         PartTemplate::Music => {
        //             dbg!(row.y);
        //         }
        //         PartTemplate::Nonmusic => {}
        //     }
        // }

        Ok(matrix)
        // Ok(())
    }

    pub fn matrix_add_beamgroups(&self) {
        for (baridx, bar) in self.items.iter().enumerate() {
            let bar = bar.borrow();
            match bar.btype {
                BarType::Standard(ref parts) => {
                    for part in parts {
                        let part = part.borrow();
                        let complexes = part.complexes.as_ref().expect("Part should have complexes!");

                        let mut note_current_beamgroup_id: usize = 0;
                        let mut note_current_beamgroup_note_idx: usize = 0;
                        let mut note2_current_beamgroup_id: usize = 0;
                        let mut note2_current_beamgroup_note_idx: usize = 0;

                        for complex in complexes {
                            let complex = complex.borrow();

                            if let Some(item) = &complex.matrix_item {
                                let mut item = item.borrow_mut();

                                let note = match &complex.ctype {
                                    ComplexType::Single(ref note, _) | ComplexType::Upper(ref note, _) => Some(note),
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

                                if let Some(note) = note {
                                    let note = note.borrow();
                                    if !note.is_heads() {
                                        continue;
                                    }

                                    let beamgroup_ref = note.beamgroup.as_ref().expect("Single note should have beamgroup!").clone();
                                    let beamgroup = beamgroup_ref.borrow();
                                    if beamgroup.id != note_current_beamgroup_id {
                                        note_current_beamgroup_id = beamgroup.id;
                                        note_current_beamgroup_note_idx = 0;

                                        let data = RItemBeamData {
                                            id: beamgroup.id,
                                            note_id: note.id,
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
                                            item.note_beam = RItemBeam::Single(data);
                                        } else {
                                            item.note_beam = RItemBeam::Start(data);
                                        }
                                    } else {
                                        note_current_beamgroup_note_idx += 1;

                                        let mut data = RItemBeamData {
                                            id: beamgroup.id,
                                            note_id: note.id,
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

                                        if note_current_beamgroup_note_idx < beamgroup.notes.len() - 1 {
                                            item.note_beam = RItemBeam::Middle(data);
                                        } else {
                                            data.note_durations = Some(beamgroup.note_durations.clone());
                                            item.note_beam = RItemBeam::End(data);
                                        }
                                    }
                                }

                                if let Some(note2) = note2 {
                                    let note2 = note2.borrow();
                                    if !note2.is_heads() {
                                        continue;
                                    }

                                    let beamgroup_ref = note2.beamgroup.as_ref().expect("Lower note should have beamgroup!").clone();
                                    let beamgroup = beamgroup_ref.borrow();
                                    if beamgroup.id != note2_current_beamgroup_id {
                                        note2_current_beamgroup_id = beamgroup.id;
                                        note2_current_beamgroup_note_idx = 0;

                                        let data = RItemBeamData {
                                            id: beamgroup.id,
                                            note_id: note2.id,
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
                                            item.note2_beam = RItemBeam::Single(data);
                                        } else {
                                            item.note2_beam = RItemBeam::Start(data);
                                        }
                                    } else {
                                        let mut data = RItemBeamData {
                                            id: beamgroup.id,
                                            note_id: note2.id,
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

                                        note2_current_beamgroup_note_idx += 1;
                                        if note2_current_beamgroup_note_idx < beamgroup.notes.len() - 1 {
                                            item.note2_beam = RItemBeam::Middle(data);
                                        } else {
                                            data.note_durations = Some(beamgroup.note_durations.clone());
                                            item.note2_beam = RItemBeam::End(data);
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

    pub fn matrix_add_ties(&self) {
        for (baridx, bar) in self.items.iter().enumerate() {
            let bar = bar.borrow();
            match bar.btype {
                BarType::Standard(ref parts) => {
                    for part in parts {
                        let part = part.borrow();
                        let complexes: &Vec<Rc<RefCell<Complex>>> = part.complexes.as_ref().expect("Part should have complexes!");
                        for (left, right) in complexes.into_iter().tuples::<(&Rc<RefCell<Complex>>, &Rc<RefCell<Complex>>)>() {
                            let left_complex: Ref<Complex> = left.borrow();
                            let right_complex: Ref<Complex> = right.borrow();

                            // .into_iter().tuples()
                            // let complex = complex.borrow();
                            // if let Some(item) = &complex.matrix_item {
                            //     let mut item: RefMut<RItem> = item.borrow_mut();

                            //     let note = match &complex.ctype {
                            //         ComplexType::Single(ref note, _) | ComplexType::Upper(ref note, _) => Some(note),
                            //         ComplexType::Two(ref note, _, _) => Some(note),
                            //         _ => None,
                            //     };

                            //     let note2 = match &complex.ctype {
                            //         ComplexType::Two(_, ref note2, _) => Some(note2),
                            //         ComplexType::Lower(ref note2, _) => Some(note2),
                            //         _ => None,
                            //     };

                            //     let adjust_x = match &complex.ctype {
                            //         ComplexType::Two(_, _, adjust_x) => adjust_x,
                            //         _ => &None,
                            //     };

                            //     if let Some(note) = note {
                            //         // note 1
                            //         let note = note.borrow();
                            //         if !note.is_heads() {
                            //             continue;
                            //         }
                            //     }

                            //     if let Some(note2) = note2 {
                            //         // note 1
                            //         let note2 = note2.borrow();
                            //         if !note2.is_heads() {
                            //             continue;
                            //         }
                            //     }
                            // }
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

    pub fn resolve_ties(&self) {
        let items = self.consecutive_note_chunks();
        for item in items {
            dbg!(item.0, item.1, item.2.len());
        }
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
            dbg!(&baridx);
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
                                                    for (noteidx, note) in notes.items.iter().enumerate() {
                                                        let part_voice_current_id = part_voice_id + *chunk_indexes.get(&part_voice_id).unwrap();

                                                        let note_ = note.borrow();
                                                        if !note_.is_pause() {
                                                            println!("Note {part_voice_id}/{part_voice_current_id}: {partidx}:{}", 0);

                                                            if !chunk_notes.contains_key(&part_voice_current_id) {
                                                                chunk_notes.insert(part_voice_current_id, vec![note.clone()]);
                                                            } else {
                                                                chunk_notes.get_mut(&part_voice_current_id).unwrap().push(note.clone());
                                                            }
                                                        } else {
                                                            println!("Note {part_voice_id} not heads! - increase chunk_indexe");
                                                            chunk_indexes.insert(part_voice_id, chunk_indexes.get(&part_voice_id).unwrap() + 1);
                                                        }
                                                    }
                                                }
                                                _ => {
                                                    println!("{partidx}:{} Not VoiceType::Notes", 0);
                                                    chunk_indexes.insert(part_voice_id, chunk_indexes.get(&part_voice_id).unwrap() + 1);
                                                    let part_voice2_id = (partidx + 1) * 1_000_000 + 1_000;
                                                    chunk_indexes.insert(part_voice2_id, chunk_indexes.get(&part_voice2_id).unwrap() + 1);
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
                                                    for (noteidx, note) in notes.items.iter().enumerate() {
                                                        let note_ = note.borrow_mut();
                                                        let part_voice_current_id = part_voice_id + *chunk_indexes.get(&part_voice_id).unwrap();
                                                        if !note_.is_pause() {
                                                            println!("Note {part_voice_id}/{part_voice_current_id}: {partidx}:{}", 0);
                                                            if !chunk_notes.contains_key(&part_voice_current_id) {
                                                                chunk_notes.insert(part_voice_current_id, vec![note.clone()]);
                                                            } else {
                                                                chunk_notes.get_mut(&part_voice_current_id).unwrap().push(note.clone());
                                                            }
                                                        } else {
                                                            println!("Note not heads!");
                                                            chunk_indexes.insert(part_voice_id, chunk_indexes.get(&part_voice_id).unwrap() + 1);
                                                        }
                                                    }
                                                }
                                                _ => {
                                                    println!("{partidx}:{} Not VoiceType::Notes", 0);
                                                    chunk_indexes.insert(part_voice_id, chunk_indexes.get(&part_voice_id).unwrap() + 1);
                                                }
                                            }

                                            let lower = lower.borrow();
                                            let part_voice_id = (partidx + 1) * 1_000_000 + 1_000;
                                            if !chunk_indexes.contains_key(&part_voice_id) {
                                                chunk_indexes.insert(part_voice_id, 0);
                                            }

                                            match lower.vtype {
                                                VoiceType::Notes(ref notes) => {
                                                    for (noteidx, note) in notes.items.iter().enumerate() {
                                                        let note_ = note.borrow_mut();
                                                        let part_voice_current_id = part_voice_id + *chunk_indexes.get(&part_voice_id).unwrap();
                                                        if !note_.is_pause() {
                                                            println!("Note {part_voice_id}/{part_voice_current_id}: {partidx}:{}", 1);
                                                            if !chunk_notes.contains_key(&part_voice_current_id) {
                                                                chunk_notes.insert(part_voice_current_id, vec![note.clone()]);
                                                            } else {
                                                                chunk_notes.get_mut(&part_voice_current_id).unwrap().push(note.clone());
                                                            }
                                                        } else {
                                                            println!("Note {part_voice_id} not heads!");
                                                            chunk_indexes.insert(part_voice_id, chunk_indexes.get(&part_voice_id).unwrap() + 1);
                                                        }
                                                    }
                                                }
                                                _ => {
                                                    println!("{partidx}:{} Not VoiceType::Notes", 1);
                                                    chunk_indexes.insert(part_voice_id, chunk_indexes.get(&part_voice_id).unwrap() + 1);
                                                }
                                            }
                                            //
                                        }
                                    },

                                    _ => {
                                        println!("{partidx} Not PartMusicType::Voices");
                                    }
                                }
                            }
                            _ => {
                                println!("{partidx} Not PartType::Music");
                            }
                        }
                    }
                    //
                }
                BarType::MultiRest(_) => {
                    for partidx in 0..parts_count {
                        let part_voice_id = (partidx + 1) * 1_000_000;
                        chunk_indexes.insert(part_voice_id, chunk_indexes.get(&part_voice_id).unwrap() + 1);
                        let part_voice_id = (partidx + 1) * 1_000_000 + 1_000;
                        chunk_indexes.insert(part_voice_id, chunk_indexes.get(&part_voice_id).unwrap() + 1);
                    }
                }

                BarType::NonContent(_) => {
                    println!("BarType::NonContent +  this should NOT cause new chunks!");
                }
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

#[derive(Debug, PartialEq)]
pub struct Bar {
    pub btype: BarType,
    pub rects: Option<Vec<Rc<RefCell<Vec<NRectExt>>>>>,
}

impl Bar {
    pub fn new(btype: BarType) -> Self {
        Self { btype, rects: None }
    }

    pub fn from_parts(parts: Parts) -> Self {
        let btype = BarType::Standard(parts);
        Self { btype, rects: None }
    }

    pub fn from_clefs(clefs: Vec<Option<ClefSignature>>) -> Self {
        let btype = BarType::NonContent(NonContentType::Clefs(clefs));
        Self { btype, rects: None }
    }

    pub fn complex_count(&self) -> usize {
        match &self.btype {
            BarType::Standard(parts) => {
                let mut count = 0;
                for part in parts {
                    let part = part.borrow();
                    if let Some(complexes) = &part.complexes {
                        let part_count = complexes.len();
                        count = part_count.max(count);
                    }
                }
                count
            }
            BarType::MultiRest(_) => 0,
            BarType::NonContent(_) => 0,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum BarType {
    Standard(Parts),
    MultiRest(usize),
    NonContent(NonContentType),
}

#[derive(Debug, PartialEq)]
pub enum NonContentType {
    Barline,
    VerticalLine,
    Clefs(Vec<Option<ClefSignature>>),
}

#[cfg(test)]
mod testsbars {
    use crate::{prelude::*, render::fonts::ebgaramond::GLYPH_HEIGHT};
    use graphics::{glyphs::ebgaramond::*, prelude::*};
    use render_notation::render::dev::*;

    #[test]
    fn example() {
        // let bar_data = QCode::bars("|clef G F | 0 0 / 0 0 0 ").unwrap();
        let bar_data = QCode::bars(" 0 ").unwrap();
        // QCode::bars("|clefs G F - | 0 % 1 / 0 /lyr $lyr:aaa | 0 / 0 /lyr $lyr:bbb").unwrap();
        let (bartemplate, bars) = bar_data;
        bars.create_matrix(Some(bartemplate)).unwrap();
    }

    #[test]
    fn resolve() {
        let bar_data = QCode::bars(" 0 1 2 | bp | 3 4 ").unwrap();
        let (bartemplate, bars) = bar_data;
        bars.resolve_ties();
    }
}
