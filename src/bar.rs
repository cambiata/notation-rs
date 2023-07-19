use std::cell::RefCell;
use std::rc::Rc;

use crate::prelude::*;

#[derive(Debug, PartialEq)]
pub struct BarColumn {
    pub btype: BarColumnType,
}

impl BarColumn {
    pub fn new(btype: BarColumnType) -> Self {
        Self { btype }
    }
}

#[derive(Debug, PartialEq)]
pub enum BarColumnType {
    Music(Option<Vec<Rc<RefCell<Part>>>>),
}

#[cfg(test)]
mod testsbars {
    #[test]
    fn example() {}
}
