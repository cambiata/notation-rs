use serde::{Deserialize, Serialize};
use serde_json;

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum Accidental {
    DblSharp,
    Sharp,
    Neutral,
    Flat,
    DblFlat,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum NValue {
    Nv1dot = 144,
    Nv1 = 96,
    Nv2dot = 72,
    Nv2 = 48,
    Nv4dot = 36,
    Nv2tri = 32,
    Nv4 = 24,
    Nv8dot = 18,
    Nv4tri = 16,
    Nv8 = 12,
    Nv16dot = 9,
    Nv8tri = 8,
    Nv16 = 6,
    Nv32 = 3,
}

impl NValue {
    pub fn from_str(s: &str) -> NValue {
        match s.to_lowercase().as_str() {
            "nv1dot" => Self::Nv1dot,
            "nv1." => Self::Nv1dot,
            "nv1" => Self::Nv1,
            "nv2dot" => Self::Nv2dot,
            "nv2." => Self::Nv2dot,
            "nv2" => Self::Nv2,
            "nv2tri" => Self::Nv2tri,
            "nv4dot" => Self::Nv4dot,
            "nv4." => Self::Nv4dot,
            "nv4" => Self::Nv4,
            "nv8dot" => Self::Nv8dot,
            "nv8." => Self::Nv8dot,
            "nv4tri" => Self::Nv4tri,
            "nv8" => Self::Nv8,
            "nv16dot" => Self::Nv16dot,
            "nv16." => Self::Nv16dot,
            "nv8tri" => Self::Nv8tri,
            "nv16" => Self::Nv16,
            "nv32" => Self::Nv32,
            _ => {
                println!("Unimplemented note value:{}", s);
                Self::Nv4
            }
        }
    }
}

pub trait NValueItem {
    fn val(&self) -> u32;
}

struct NValueIterator<'a> {
    pos: usize,
    idx: usize,
    items: Vec<&'a dyn NValueItem>,
}

impl<'a> NValueIterator<'a> {
    fn new(items: Vec<&'a dyn NValueItem>) -> Self {
        Self {
            pos: 0,
            idx: 0,
            items,
        }
    }
}

impl<'a> Iterator for NValueIterator<'a> {
    type Item = &'a dyn NValueItem;
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.items.len() - 1 {
            let item = self.items[self.idx];
            self.idx += 1;
            return Some(item);
        }
        None
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub enum DirUAD {
    Up,
    Auto,
    Down,
}

#[cfg(test)]
mod tests {
    use crate::core::*;
    use crate::quick::QCode;

    #[test]
    fn example() {
        let notes = QCode::notes("nv4 0 nv8 1 nv2 2");
    }
}
