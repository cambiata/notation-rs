use itertools::Itertools;
use std::cell::{Ref, RefMut};
use std::collections::BTreeMap;
use std::hash::Hash;
use std::rc::Rc;
use std::{cell::RefCell, collections::HashMap};

use crate::{complex, part, prelude::*};

#[derive(Debug, PartialEq)]
pub enum TiePlacement {
    Top,
    Mid,
    Bottom,
}

#[derive(Debug, PartialEq)]
pub struct Bar {
    pub btype: BarType,
    pub rects: Option<Vec<Rc<RefCell<Vec<NRectExt>>>>>,
    pub duration: Duration,
    pub position: Position,
}

impl Bar {
    pub fn new(btype: BarType) -> Self {
        let duration = btype.duration();
        Self {
            btype,
            rects: None,
            duration,
            position: 0,
        }
    }

    pub fn from_parts(parts: Parts) -> Self {
        let btype = BarType::Standard(parts);
        let duration = btype.duration();
        Self {
            btype,
            rects: None,
            duration,
            position: 0,
        }
    }

    pub fn from_clefs(clefs: Vec<Option<ClefSignature>>) -> Self {
        let btype = BarType::BarAttribute(BarAttributeType::Clefs(clefs));
        let duration = btype.duration();
        Self {
            btype,
            rects: None,
            duration,
            position: 0,
        }
    }

    pub fn from_keys(keys: Vec<Option<KeySignature>>) -> Self {
        let btype = BarType::BarAttribute(BarAttributeType::Keys(keys));
        let duration = btype.duration();
        Self {
            btype,
            rects: None,
            duration,
            position: 0,
        }
    }

    pub fn from_times(times: Vec<Option<TimeSignature>>) -> Self {
        let btype = BarType::BarAttribute(BarAttributeType::Times(times));
        let duration = btype.duration();
        Self {
            btype,
            rects: None,
            duration,
            position: 0,
        }
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
            BarType::CountIn(_) => 0,
            BarType::BarAttribute(_) => 0,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum BarType {
    Standard(Parts),
    BarAttribute(BarAttributeType),
    MultiRest(usize),
    NonContent(NonContentType),
    CountIn(Notes),
}

impl BarType {
    pub fn duration(&self) -> Duration {
        match self {
            BarType::Standard(parts) => {
                let mut duration = 0;
                for part in parts {
                    let part = part.borrow();
                    duration = duration.max(part.duration);
                }
                duration
            }
            BarType::MultiRest(_) => todo!(),
            BarType::NonContent(_) => 0,
            BarType::BarAttribute(_) => 0,
            BarType::CountIn(notes) => notes.duration,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum NonContentType {
    Barline(BarlineType),
    VerticalLine,
    Spacer(f32, f32),
}

#[derive(Debug, PartialEq)]
pub enum BarAttributeType {
    Clefs(Vec<Option<ClefSignature>>),
    Times(Vec<Option<TimeSignature>>),
    Keys(Vec<Option<KeySignature>>),
}

#[cfg(test)]
mod testsbars {
    use crate::prelude::*;
    use graphics::{glyphs::ebgaramond::*, prelude::*};
    use render_notation::render::output::*;

    #[test]
    fn example() {
        // let bar_data = QCode::bars("|clef G F | 0 0 / 0 0 0 ").unwrap();
        let bar_data = QCode::bars(" 0 ").unwrap();
        // QCode::bars("|clefs G F - | 0 % 1 / 0 /lyr $lyr:aaa | 0 / 0 /lyr $lyr:bbb").unwrap();
        let (bartemplate, mut bars) = bar_data;
        bars.create_matrix(Some(bartemplate)).unwrap();
    }

    #[test]
    fn resolve() {
        let bar_data = QCode::bars(" 0 1 2 | bp | 3 4 ").unwrap();
        let (bartemplate, mut bars) = bar_data;
        bars.resolve_ties();
    }
}
