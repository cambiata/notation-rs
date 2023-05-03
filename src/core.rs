use serde::{Deserialize, Serialize};


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

    pub fn from_str_option(s: &str) -> Option<NValue> {
        match s.to_lowercase().as_str() {
            "nv1dot" => Some(Self::Nv1dot),
            "nv1." => Some(Self::Nv1dot),
            "nv1" => Some(Self::Nv1),
            "nv2dot" => Some(Self::Nv2dot),
            "nv2." => Some(Self::Nv2dot),
            "nv2" => Some(Self::Nv2),
            "nv2tri" => Some(Self::Nv2tri),
            "nv4dot" => Some(Self::Nv4dot),
            "nv4." => Some(Self::Nv4dot),
            "nv4" => Some(Self::Nv4),
            "nv8dot" => Some(Self::Nv8dot),
            "nv8." => Some(Self::Nv8dot),
            "nv4tri" => Some(Self::Nv4tri),
            "nv8" => Some(Self::Nv8),
            "nv16dot" => Some(Self::Nv16dot),
            "nv16." => Some(Self::Nv16dot),
            "nv8tri" => Some(Self::Nv8tri),
            "nv16" => Some(Self::Nv16),
            "nv32" => Some(Self::Nv32),
            _ => {
                println!("Unimplemented note value:{}", s);
                None
            }
        }
    }

    pub fn is_beamable(self: NValue) -> bool {
        match self {
            Self::Nv8 | Self::Nv8dot | Self::Nv8tri | Self::Nv16 | Self::Nv16dot | Self::Nv32 => {
                true
            }
            _ => false,
        }
    }
}

impl From<usize> for NValue {
    fn from(val: usize) -> Self {
        match val {
            144 => Self::Nv1dot,
            96 => Self::Nv1,
            72 => Self::Nv2dot,
            48 => Self::Nv2,
            36 => Self::Nv4dot,
            32 => Self::Nv2tri,
            24 => Self::Nv4,
            18 => Self::Nv8dot,
            16 => Self::Nv4tri,
            12 => Self::Nv8,
            9 => Self::Nv16dot,
            6 => Self::Nv16,
            8 => Self::Nv8tri,
            3 => Self::Nv32,
            _ => {
                panic!("Unimplemented note value:{}", val);
            }
        }
    }
}

impl From<NValue> for usize {
    fn from(val: NValue) -> Self {
        match val {
            NValue::Nv1dot => 144,
            NValue::Nv1 => 96,
            NValue::Nv2dot => 72,
            NValue::Nv2 => 48,
            NValue::Nv4dot => 36,
            NValue::Nv2tri => 32,
            NValue::Nv4 => 24,
            NValue::Nv8dot => 18,
            NValue::Nv4tri => 16,
            NValue::Nv8 => 12,
            NValue::Nv16dot => 9,
            NValue::Nv8tri => 8,
            NValue::Nv16 => 6,
            NValue::Nv32 => 3,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum DirUAD {
    Up,
    Auto,
    Down,
}

#[cfg(test)]
mod tests {
    use crate::core::NValue;
    
    use crate::quick::QCode;

    #[test]
    fn example() {
        let _notes = QCode::notes("nv4 0 nv8 1 nv2 2");
    }

    #[test]
    fn nvalues() {
        assert_eq!(NValue::Nv4 as usize, 24);
    }

    #[test]
    fn nvalues2() {
        let _v = NValue::from(333);
    }
}
