use std::hash::Hash;
use std::rc::Rc;
use std::{cell::RefCell, collections::HashMap};

use crate::{complex, prelude::*};

#[derive(Debug, PartialEq)]
pub struct Bars(pub Vec<Rc<RefCell<Bar>>>);

impl Bars {
    pub fn to_matrix(&self) -> Result<RMatrix> {
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

                    // let mut complexpositions = vec![];
                    let mut positions = vec![];
                    let mut parts_positions: Vec<HashMap<usize, usize>> = vec![];

                    let mut duration = 0;
                    for (partidx, part) in parts.iter().enumerate() {
                        let mut complex_positions: HashMap<usize, usize> = HashMap::new();

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

                    let complex_count = bar.complex_count();

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
                                let complex =
                                    &part.complexes.as_ref().expect("This complex should exist!")
                                        [*complexidx]
                                        .borrow();

                                let item_rects: Vec<NRect> =
                                    complex.rects.borrow().iter().map(|nrect| nrect.0).collect();

                                item = Some(Rc::new(RefCell::new(RItem::new(
                                    item_rects,
                                    complex.duration,
                                ))));

                                colduration = Some(durations[posidx]);
                            } else {
                                //
                            }

                            // // dbg!(&complex_positions);
                            // let mut part = part.borrow_mut();
                            // let mut item: Option<Rc<RefCell<RItem>>> = None;

                            colitems.push(item);
                        }

                        println!("column:{} =====================================", position);
                        dbg!(&colitems);

                        let rcol: RCol = RCol::new(colitems, colduration);
                        matrix_cols.push(Rc::new(RefCell::new(rcol)));
                    }
                }
                BarType::MultiRest(_) => todo!(),
                BarType::NonContent(nctype) => {}
            }
        }

        let matrix = RMatrix::new(matrix_cols);

        Ok(matrix)
        // Ok(())
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
                        dbg!(part_count);
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
        bars.to_matrix().unwrap();
    }
}
