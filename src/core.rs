use crate::{head::HeadType, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum Accidental {
    DblSharp,
    Sharp,
    Neutral,
    Flat,
    DblFlat,
}

// #[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
// pub enum Duration {
//     Nv1dot = 144,
//     Nv1 = 96,
//     Nv2dot = 72,
//     Nv2 = 48,
//     Nv4dot = 36,
//     Nv2tri = 32,
//     Nv4 = 24,
//     Nv8dot = 18,
//     Nv4tri = 16,
//     Nv8 = 12,
//     Nv16dot = 9,
//     Nv8tri = 8,
//     Nv16 = 6,
//     Nv32 = 3,
// }

// impl Duration {
//     pub fn from_str(s: &str) -> Option<Duration> {
//         match s.to_lowercase().as_str() {
//             "nv1dot" => Some(Self::Nv1dot),
//             "nv1." => Some(Self::Nv1dot),
//             "nv1" => Some(Self::Nv1),
//             "nv2dot" => Some(Self::Nv2dot),
//             "nv2." => Some(Self::Nv2dot),
//             "nv2" => Some(Self::Nv2),
//             "nv2tri" => Some(Self::Nv2tri),
//             "nv4dot" => Some(Self::Nv4dot),
//             "nv4." => Some(Self::Nv4dot),
//             "nv4" => Some(Self::Nv4),
//             "nv8dot" => Some(Self::Nv8dot),
//             "nv8." => Some(Self::Nv8dot),
//             "nv4tri" => Some(Self::Nv4tri),
//             "nv8" => Some(Self::Nv8),
//             "nv16dot" => Some(Self::Nv16dot),
//             "nv16." => Some(Self::Nv16dot),
//             "nv8tri" => Some(Self::Nv8tri),
//             "nv16" => Some(Self::Nv16),
//             "nv32" => Some(Self::Nv32),
//             _ => {
//                 println!("Unimplemented note value:{}", s);
//                 None
//             }
//         }
//     }

//     pub fn is_beamable(self: Duration) -> bool {
//         match self {
//             Self::Nv8 | Self::Nv8dot | Self::Nv8tri | Self::Nv16 | Self::Nv16dot | Self::Nv32 => {
//                 true
//             }
//             _ => false,
//         }
//     }
// }

// impl From<usize> for Duration {
//     fn from(val: usize) -> Self {
//         match val {
//             144 => Self::Nv1dot,
//             96 => Self::Nv1,
//             72 => Self::Nv2dot,
//             48 => Self::Nv2,
//             36 => Self::Nv4dot,
//             32 => Self::Nv2tri,
//             24 => Self::Nv4,
//             18 => Self::Nv8dot,
//             16 => Self::Nv4tri,
//             12 => Self::Nv8,
//             9 => Self::Nv16dot,
//             6 => Self::Nv16,
//             8 => Self::Nv8tri,
//             3 => Self::Nv32,
//             _ => {
//                 panic!("Unimplemented note value:{}", val);
//             }
//         }
//     }
// }

// impl From<Duration> for usize {
//     fn from(val: Duration) -> Self {
//         match val {
//             Duration::Nv1dot => 144,
//             Duration::Nv1 => 96,
//             Duration::Nv2dot => 72,
//             Duration::Nv2 => 48,
//             Duration::Nv4dot => 36,
//             Duration::Nv2tri => 32,
//             Duration::Nv4 => 24,
//             Duration::Nv8dot => 18,
//             Duration::Nv4tri => 16,
//             Duration::Nv8 => 12,
//             Duration::Nv16dot => 9,
//             Duration::Nv8tri => 8,
//             Duration::Nv16 => 6,
//             Duration::Nv32 => 3,
//         }
//     }
// }

// pub trait NValueItem {
//     fn val(&self) -> u32;
// }

// struct NValueIterator<'a> {
//     pos: usize,
//     idx: usize,
//     items: Vec<&'a dyn NValueItem>,
// }

// impl<'a> NValueIterator<'a> {
//     fn new(items: Vec<&'a dyn NValueItem>) -> Self {
//         Self {
//             pos: 0,
//             idx: 0,
//             items,
//         }
//     }
// }

// impl<'a> Iterator for NValueIterator<'a> {
//     type Item = &'a dyn NValueItem;
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.idx < self.items.len() - 1 {
//             let item = self.items[self.idx];
//             self.idx += 1;
//             return Some(item);
//         }
//         None
//     }
// }
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum DirUAD {
    Up,
    Auto,
    Down,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum DirUD {
    Up,
    Down,
}

pub const NV1DOT: usize = 144;
pub const NV1: usize = 96;
pub const NV2DOT: usize = 72;
pub const NV2: usize = 48;
pub const NV4DOT: usize = 36;
pub const NV2TRI: usize = 32;
pub const NV4: usize = 24;
pub const NV8DOT: usize = 18;
pub const NV4TRI: usize = 16;
pub const NV8: usize = 12;
pub const NV16DOT: usize = 9;
pub const NV8TRI: usize = 8;
pub const NV16: usize = 6;
pub const NV16TRI: usize = 4;
pub const NV32: usize = 3;

pub type Duration = usize;

pub type Position = usize;

pub fn duration_from_str(s: &str) -> Result<Duration> {
    match s.to_lowercase().as_str() {
        "1dot" => Ok(NV1DOT),
        "1." => Ok(NV1DOT),
        "1" => Ok(NV1),
        "2dot" => Ok(NV2DOT),
        "2." => Ok(NV2DOT),
        "2" => Ok(NV2),
        "2tri" => Ok(NV2TRI),
        "4dot" => Ok(NV4DOT),
        "4." => Ok(NV4DOT),
        "4" => Ok(NV4),
        "8dot" => Ok(NV8DOT),
        "8." => Ok(NV8DOT),
        "4tri" => Ok(NV4TRI),
        "8" => Ok(NV8),
        "16dot" => Ok(NV16DOT),
        "16." => Ok(NV16DOT),
        "8tri" => Ok(NV8TRI),
        "16" => Ok(NV16),
        "16tri" => Ok(NV16TRI),
        "32" => Ok(NV32),
        _ => Err(DurationError(format!(
            "Can not convert string '{}' into usize Duration",
            s
        ))
        .into()),
    }
}

pub fn duration_from(v: usize) -> Result<Duration> {
    match v {
        NV1DOT | NV1 | NV2DOT | NV2 | NV4DOT | NV2TRI | NV4 | NV8DOT | NV4TRI | NV8 | NV16DOT
        | NV8TRI | NV16 | NV16TRI | NV32 => Ok(v),
        _ => Err(DurationError(format!("Can not convert value {} to usize Duration", v)).into()),
    }
}

pub fn duration_is_beamable(dur: usize) -> bool {
    match dur {
        NV8 | NV8DOT | NV8TRI | NV16 | NV16DOT | NV32 => true,
        _ => false,
    }
}

pub fn duration_get_headtype(duration: Duration) -> HeadType {
    match duration {
        NV1DOT | NV1 => HeadType::WideHead,
        _ => HeadType::NormalHead,
    }
}

pub struct NRect(f32, f32, f32, f32);

#[cfg(test)]
mod tests {

    use crate::prelude::*;

    #[test]
    fn example() {
        let _notes = QCode::notes("nv4 0 nv8 1 nv2 2");
    }

    #[test]
    fn nvalues() {
        assert_eq!(NV4, 24);
    }

    #[test]
    fn nvalues2() {
        let _v = duration_from(333);
    }
}
