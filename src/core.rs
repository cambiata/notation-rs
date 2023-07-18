use crate::prelude::*;

pub const SPACE: f32 = 25.0;
pub const SPACE_HALF: f32 = SPACE / 2.0;
pub const SPACE_QUARTER: f32 = SPACE / 4.0;
pub const HEAD_WIDTH_BLACK: f32 = 1.3 * SPACE;
pub const HEAD_WIDTH_WHITE: f32 = 1.35 * SPACE;
pub const HEAD_WIDTH_WIDE: f32 = 1.65 * SPACE;
pub const DOT_WIDTH: f32 = 0.8 * SPACE;
pub const STEM_WIDTH: f32 = 2.5;
pub const FONT_SCALE_LYRICS: f32 = 0.08;

pub const RELATIVE_SPCACING_FACTOR: f32 = 30.0;
pub const LINEAR_SPACING_FACTOR: f32 = 10.0;
pub const SAME_SPACING_FACTOR: f32 = 10.0;

//------------------------------------------------------------
pub const LINE: f32 = 2.7;
//------------------------------------------------------------
pub const NOTELINES_WIDTH: f32 = 1.0 * LINE;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Accidental {
    DblSharp,
    Sharp,
    Natural,
    Flat,
    DblFlat,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Tie {
    Standard,
    LetRing,
}

//============================================================

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum DirUD {
    Up,
    Down,
}

#[derive(Debug)]
pub enum DirUAD {
    Up,
    Auto,
    Down,
}

//============================================================

pub type Duration = usize;
pub type Position = usize;

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

// 144
// 96
// 72
// 48
// 36
// 32
// 24
// 18
// 16
// 12
// 9
// 8
// 6
// 4
// 3

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

pub fn duration_is_beamable(duration: &Duration) -> bool {
    match *duration {
        NV8 | NV8DOT | NV8TRI | NV16 | NV16DOT | NV32 => true,
        _ => false,
    }
}

pub fn duration_has_stem(duration: &Duration) -> bool {
    match *duration {
        NV1DOT | NV1 => false,
        _ => true,
    }
}

pub fn duration_get_headtype(duration: &Duration) -> &HeadType {
    match *duration {
        NV1DOT | NV1 => &HeadType::WideHead,
        _ => &HeadType::NormalHead,
    }
}

pub fn duration_get_headshape(duration: &Duration) -> &HeadShape {
    match *duration {
        NV1DOT | NV1 => &HeadShape::WholeHead,
        NV2 | NV2DOT | NV2TRI => &HeadShape::WhiteHead,
        _ => &HeadShape::BlackHead,
    }
}

pub fn duration_get_headwidth(duration: &Duration) -> f32 {
    match *duration {
        NV1DOT | NV1 => HEAD_WIDTH_WIDE,
        NV2 | NV2DOT | NV2TRI => HEAD_WIDTH_WHITE,
        _ => HEAD_WIDTH_BLACK,
    }
}

pub fn duration_get_dots(duration: &Duration) -> u8 {
    match *duration {
        NV1DOT | NV2DOT | NV4DOT | NV8DOT | NV16DOT => 1,
        _ => 0,
    }
}

//============================================================

#[derive(Clone, Copy, Debug)]
pub struct NRect(pub f32, pub f32, pub f32, pub f32);

impl NRect {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self(x, y, w, h)
    }

    // pub fn overlap_x(&self, right: &Self) -> f32 {
    //     if self.1 + self.3 <= right.1 {
    //         return 0.0;
    //     }
    //     if self.1 >= right.1 + right.3 {
    //         return 0.0;
    //     }
    //     return (self.0 + self.2) - right.0;
    // }

    pub fn overlap_multi_nrectexts_x(&self, nrects: &Vec<NRectExt>) -> f32 {
        let mut result = 0.0;
        for nrect in nrects {
            let overlap = self.overlap_x(&nrect.0);
            let overlap_value = overlap.unwrap();
            if overlap.is_some() && overlap_value > result {
                result = overlap.unwrap();
            }
        }
        result
    }

    pub fn overlap_x(&self, right: &Self) -> Option<f32> {
        if self.1 + self.3 <= right.1 {
            return None;
        }
        if self.1 >= right.1 + right.3 {
            return None;
        }
        return Some(self.0 + self.2 - right.0);
    }

    pub fn move_rect(&self, x: f32, y: f32) -> Self {
        Self(self.0 + x, self.1 + y, self.2, self.3)
    }
}
pub struct NRects(pub Vec<NRect>);

impl NRects {
    pub fn new(nrects: Vec<NRect>) -> Self {
        Self(nrects)
    }

    pub fn move_nrects(&self, x: f32, y: f32) -> Self {
        let mut result = Vec::new();
        for nrect in self.0.iter() {
            result.push(nrect.move_rect(x, y));
        }
        Self(result)
    }

    pub fn overlap_x(&self, rights: &Self) -> Option<f32> {
        let mut result: Option<f32> = None;
        for left in self.0.iter() {
            for right in rights.0.iter() {
                let overlap = left.overlap_x(&right);
                dbg!(overlap);
                match [overlap.is_some(), result.is_some()] {
                    [true, true] => {
                        if overlap.unwrap() > result.unwrap() {
                            result = overlap;
                        }
                    }
                    [true, false] => {
                        result = overlap;
                    }
                    _ => {}
                }
            }
        }
        dbg!(result);
        result
    }
}

#[derive(Debug)]
pub struct NRectExt(pub NRect, pub NRectType);

impl NRectExt {
    pub fn new(nrect: NRect, nrect_type: NRectType) -> Self {
        Self(nrect, nrect_type)
    }
}

//============================================================

pub trait Spacing {
    fn duration(duration: &Duration) -> f32;
}

pub struct SpacingLinear {}

impl Spacing for SpacingLinear {
    fn duration(duration: &Duration) -> f32 {
        *duration as f32 * LINEAR_SPACING_FACTOR
    }
}

pub struct SpacingRelative {}

impl Spacing for SpacingRelative {
    fn duration(duration: &Duration) -> f32 {
        let v = match duration {
            0 => 0.0,
            144 => 8.0, //NV1DOT
            96 => 7.0,  // NV1 =>
            72 => 6.0,  // NV2DOT =>
            48 => 5.0,  // NV2 =>
            36 => 4.0,  // NV4DOT =>
            32 => 2.75, // NV2TRI =>
            24 => 3.5,  // NV4 =>
            18 => 3.0,  // NV8DOT =>
            16 => 2.75, // NV4TRI =>
            12 => 2.5,  // NV8 =>
            9 => 2.35,  // NV16DOT =>
            8 => 2.15,  // NV8TRI =>
            6 => 2.0,   // NV16 =>
            4 => 1.75,  // NV16TRI =>
            3 => 1.5,   // NV32 =>
            _ => {
                todo!("Unknown spacing duration: {}", duration);
            }
        };
        v * RELATIVE_SPCACING_FACTOR
    }
}

pub struct SpacingEqual {}

impl Spacing for SpacingEqual {
    fn duration(duration: &Duration) -> f32 {
        4.0 * SAME_SPACING_FACTOR
    }
}

#[derive(Debug)]
pub enum NRectType {
    Head(HeadType, HeadShape),
    Dotted(u8),
    Pause(PauseShape),
    Clef,
    Accidental(Accidental),
    Tie(Tie),
    LyricChar(char),
    WIP(String),
    DevStem,
    DUMMY,
    Dev(bool, String),
    // DevRectRed,
    // DevRectBlue,
}

#[derive(Debug)]
pub enum PauseShape {
    Whole,
    Half,
    Quarter,
    Eighth,
    Sixteenth,
    ThirtySecond,
}

#[cfg(test)]
mod tests2 {
    use super::*;
    use crate::prelude::*;
    #[test]
    fn overlap() {
        let left = NRect::new(0.0, 0.0, 10.0, 10.0);
        let right = NRect::new(5.0, 0.0, 10.0, 10.0);
        let overlap_x = left.overlap_x(&right);
        dbg!(overlap_x);
    }

    #[test]
    fn overlap2() {
        let lefts = NRects(vec![
            NRect::new(0.0, 0.0, 10.0, 10.0),
            NRect::new(0.0, 10.0, 10.0, 10.0),
        ]);

        let rights = NRects(vec![
            NRect::new(20.0, 0.0, 10.0, 10.0),
            NRect::new(5.0, 10.0, 10.0, 10.0),
        ]);

        let overlap_x = lefts.overlap_x(&rights);
        // dbg!(overlap_x);
    }
}

fn r10() -> NRect {
    NRect(0.0, 0.0, 10.0, 10.0)
}
