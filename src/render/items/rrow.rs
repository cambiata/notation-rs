use crate::prelude::NRect;
use crate::{prelude::*, types::some_cloneables::SomeCloneablePairs};
use std::cell::{Ref, RefMut};

#[derive(Debug, PartialEq)]
pub struct RRow {
    pub items: Vec<Option<Rc<RefCell<RItem>>>>,
    pub distance_y: f32,
    pub y: f32,
    pub nrects: Vec<Rc<RefCell<NRectExt>>>,
}

impl RRow {
    pub fn new(items: Vec<Option<Rc<RefCell<RItem>>>>, distance_y: f32) -> Self {
        Self {
            items,
            distance_y,
            y: 0.0,
            nrects: vec![],
        }
    }
}
