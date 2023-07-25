use std::cell::Ref;
use std::hash::Hash;
use std::rc::Rc;
use std::{cell::RefCell, collections::HashMap};

use crate::{complex, prelude::*};

#[derive(Debug, PartialEq)]
pub struct Bars(pub Vec<Rc<RefCell<Bar>>>);

impl Bars {
    pub fn to_matrix(&self, bartemplate: &BarTemplate) -> Result<RMatrix> {
        // pub fn to_matrix(&self) -> Result<()> {
        let mut matrix_cols: Vec<Rc<RefCell<RCol>>> = vec![];
        for (baridx, bar) in self.0.iter().enumerate() {
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

                    // let complex_count = bar.complex_count();

                    // for columnidx in 0..complex_count {
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

        let matrix = RMatrix::new(matrix_cols);

        Ok(matrix)
        // Ok(())
    }

    pub fn add_beamgroups_to_matrix_items(&self) {
        for (baridx, bar) in self.0.iter().enumerate() {
            dbg!(baridx);
            let bar = bar.borrow();
            match bar.btype {
                BarType::Standard(ref parts) => {
                    for part in parts {
                        let part = part.borrow();
                        let complexes = part.complexes.as_ref().expect("Part should have complexes!");

                        // let mut upper_beamgroup: Option<Rc<RefCell<Beamgroup>>> = None;
                        let mut note_current_beamgroup_id: usize = 0;
                        let mut note_current_beamgroup_note_idx: usize = 0;
                        let mut note2_current_beamgroup_id: usize = 0;
                        let mut note2_current_beamgroup_note_idx: usize = 0;

                        for complex in complexes {
                            let complex = complex.borrow();
                            if let Some(item) = &complex.matrix_item {
                                let mut item = item.borrow_mut();

                                match &complex.ctype {
                                    ComplexType::Lower(ref note, _) | ComplexType::Upper(ref note, _) | ComplexType::Single(ref note, _) => {
                                        let note = &note.borrow();
                                        let beamgroup_ref = note.beamgroup.as_ref().expect("Single note should have beamgroup!").clone();
                                        let beamgroup = beamgroup_ref.borrow();
                                        println!("beamgroup.id:{}", beamgroup.id);
                                        if beamgroup.id != note_current_beamgroup_id {
                                            note_current_beamgroup_id = beamgroup.id;
                                            note_current_beamgroup_note_idx = 0;
                                            if (beamgroup.notes.len() == 1) {
                                                item.upper_beam = RItemBeam::Single(beamgroup.id);
                                            } else {
                                                item.upper_beam = RItemBeam::Start(beamgroup.id, beamgroup.direction.unwrap());
                                            }
                                        } else {
                                            note_current_beamgroup_note_idx += 1;
                                            if note_current_beamgroup_note_idx < beamgroup.notes.len() - 1 {
                                                item.upper_beam = RItemBeam::Middle(beamgroup.id);
                                            } else {
                                                item.upper_beam = RItemBeam::End(beamgroup.id);
                                            }
                                        }
                                    }

                                    ComplexType::Two(ref note, ref note2, _) => {
                                        let note = &note.borrow();
                                        let beamgroup_ref = note.beamgroup.as_ref().expect("Upper note should have beamgroup!").clone();
                                        let beamgroup = beamgroup_ref.borrow();
                                        println!("beamgroup.id:{}", beamgroup.id);
                                        if beamgroup.id != note_current_beamgroup_id {
                                            note_current_beamgroup_id = beamgroup.id;
                                            note_current_beamgroup_note_idx = 0;
                                            if (beamgroup.notes.len() == 1) {
                                                item.upper_beam = RItemBeam::Single(beamgroup.id);
                                            } else {
                                                item.upper_beam = RItemBeam::Start(beamgroup.id, beamgroup.direction.unwrap());
                                            }
                                        } else {
                                            note_current_beamgroup_note_idx += 1;
                                            if note_current_beamgroup_note_idx < beamgroup.notes.len() - 1 {
                                                item.upper_beam = RItemBeam::Middle(beamgroup.id);
                                            } else {
                                                item.upper_beam = RItemBeam::End(beamgroup.id);
                                            }
                                        }

                                        let note2 = &note2.borrow();
                                        let beamgroup_ref = note2.beamgroup.as_ref().expect("Lower note should have beamgroup!").clone();
                                        let beamgroup = beamgroup_ref.borrow();
                                        println!("beamgroup.id:{}", beamgroup.id);
                                        if beamgroup.id != note_current_beamgroup_id {
                                            note_current_beamgroup_id = beamgroup.id;
                                            note_current_beamgroup_note_idx = 0;
                                            if (beamgroup.notes.len() == 1) {
                                                item.lower_beam = RItemBeam::Single(beamgroup.id);
                                            } else {
                                                item.lower_beam = RItemBeam::Start(beamgroup.id, beamgroup.direction.unwrap());
                                            }
                                        } else {
                                            note_current_beamgroup_note_idx += 1;
                                            if note_current_beamgroup_note_idx < beamgroup.notes.len() - 1 {
                                                item.lower_beam = RItemBeam::Middle(beamgroup.id);
                                            } else {
                                                item.lower_beam = RItemBeam::End(beamgroup.id);
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
        bars.to_matrix(&bartemplate).unwrap();
    }
}
