use std::num::ParseIntError;

use crate::prelude::*;

pub fn parse_string_to_int(s: &str) -> Result<isize> {
    let mut s2 = "".to_string();
    let mut negative = false;
    for c in s.chars() {
        match c {
            '-' => negative = true,
            '.' | ',' => {
                return Err(Generic("Invalid character - can not parse strings containing '.' or ',' characters to isize integers".to_string()).into());
            }
            c if c.is_digit(10) => s2.push(c),
            _ => {}
        }
    }
    let n = s2.parse::<isize>()?;
    if negative {
        Ok(-n)
    } else {
        Ok(n)
    }
}

pub fn parse_accidental(s: &str) -> Option<Accidental> {
    if s.contains("bb") {
        return Some(Accidental::DblFlat);
    }
    if s.contains('x') {
        return Some(Accidental::DblSharp);
    }
    if s.contains('b') {
        return Some(Accidental::Flat);
    }
    if s.contains('#') {
        return Some(Accidental::Sharp);
    }
    if s.contains('n') {
        return Some(Accidental::Natural);
    }
    None
}

pub fn parse_tie(s: &str) -> Option<Tie> {
    if s.contains("_") {
        return Some(Tie::Standard);
    }
    None
}

pub fn rect_x(rect: &NRect, nrects: Vec<NRectExt>) -> f32 {
    for nrect in &nrects {
        let cmp: NRect = nrect.0;
    }

    0.0
}
