use crate::voice::*;

pub struct Voices<'a> {
    pub items: (Voice<'a>, Voice<'a>),
    pub val: u32,
}

impl<'a> Voices<'a> {
    fn new(items: (Voice<'a>, Voice<'a>), val: u32) -> Self {
        Self { items, val }
    }
}
