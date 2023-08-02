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

    pub fn from_keys(keys: Vec<Option<KeySignature>>) -> Self {
        let btype = BarType::NonContent(NonContentType::Keys(keys));
        Self { btype, rects: None }
    }

    pub fn from_times(times: Vec<Option<TimeSignature>>) -> Self {
        let btype = BarType::NonContent(NonContentType::Times(times));
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
