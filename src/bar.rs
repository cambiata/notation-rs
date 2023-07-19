use std::cell::RefCell;
use std::rc::Rc;

use crate::prelude::*;

#[derive(Debug, PartialEq)]
pub struct BarColumn {
    pub btype: BarColumnType,
}

#[derive(Debug, PartialEq)]
pub enum BarColumnType {
    Music(Option<Vec<Rc<RefCell<Part>>>>),
}
