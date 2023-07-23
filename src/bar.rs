use std::cell::RefCell;
use std::rc::Rc;

use crate::{complex, prelude::*};

#[derive(Debug, PartialEq)]
pub struct Bars(pub Vec<Rc<RefCell<Bar>>>);

impl Bars {
    pub fn to_matrix(&self) -> Result<RMatrix> {
        let mut matrix_cols: Vec<Rc<RefCell<RCol>>> = vec![];
        for (baridx, bar) in self.0.iter().enumerate() {
            let bar = bar.borrow();
            match &bar.btype {
                BarType::Standard(parts) => {
                    for part in parts {
                        let mut part = part.borrow_mut();
                        part.setup_complexes()?;
                    }

                    let complex_count = bar.complex_count();

                    for columnidx in 0..complex_count {
                        let mut colitems = vec![];
                        let mut colduration: Option<Duration> = None;
                        for part in parts {
                            let mut part = part.borrow_mut();
                            let mut item: Option<Rc<RefCell<RItem>>> = None;
                            match &part.ptype {
                                PartType::Music(mtype) => match mtype {
                                    PartMusicType::Voices(voices) => {
                                        let complex = part
                                            .complexes
                                            .as_ref()
                                            .expect("PartMusicType::Voices should have complexes")
                                            .get(columnidx);

                                        if let Some(complex) = complex {
                                            println!("complex idx:{}", columnidx);
                                            let complex = complex.borrow();

                                            let item_rects: Vec<NRect> = complex
                                                .rects
                                                .borrow()
                                                .iter()
                                                .map(|nrect| nrect.0)
                                                .collect();

                                            item = Some(Rc::new(RefCell::new(RItem::new(
                                                item_rects,
                                                complex.duration,
                                            ))));
                                            colduration = Some(complex.duration);
                                        } else {
                                            println!("No complex {}", columnidx);
                                        }

                                        // }
                                    }
                                    PartMusicType::RepeatBar(_) => todo!(),
                                },
                                PartType::Nonmusic(nmtype) => match nmtype {
                                    PartNonmusicType::Lyrics(voices) => todo!("lyrics"),
                                    PartNonmusicType::Other => todo!(),
                                },
                            }
                            colitems.push(item);
                        }

                        println!("column:{} =====================================", columnidx);

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
