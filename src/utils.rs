use std::num::ParseIntError;

use crate::{core::Accidental, prelude::*};

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
    None
}

#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    use super::parse_string_to_int;
    #[test]
    fn test() {
        let i = parse_string_to_int("abc 3 21 xyz").unwrap();
        assert_eq!(i, 321);
    }
}
