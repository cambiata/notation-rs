use crate::prelude::NRect;
use crate::prelude::*;
use std::cell::{Ref, RefMut};

#[derive(Debug, PartialEq)]
pub struct RCol {
    pub duration: Duration,
    pub items: Vec<Option<Rc<RefCell<RItem>>>>,
    pub distance_x: f32,
    pub x: f32,

    pub spacing_duration: f32,
    pub spacing_overlap: f32,
    pub overlap_overshoot: f32,
    pub alloted_duration: f32,

    pub position: Option<Position>,
}

impl RCol {
    pub fn new(items: Vec<Option<Rc<RefCell<RItem>>>>, duration: Option<Duration>, position: Option<Position>) -> Self {
        Self {
            items,
            duration: duration.unwrap_or(0),
            distance_x: 0.0,
            x: 0.0,

            spacing_duration: 0.0,
            spacing_overlap: 0.0,
            overlap_overshoot: 0.0,
            alloted_duration: 0.0,

            position,
        }
    }
}
