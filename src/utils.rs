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

pub fn parse_function(s: &str) -> Option<(FunctionType, FunctionColor, FunctionBass, bool, bool)> {
    let segments = s.split(':').collect::<Vec<&str>>();
    dbg!(&segments);
    let mut ftype: FunctionType = FunctionType::Spacer;
    let mut fcolor: FunctionColor = FunctionColor::FcNone;
    let mut fbass: FunctionBass = FunctionBass::FbNone;
    let mut start_par: bool = false;
    let mut end_par: bool = false;
    let mut segments_len = &segments.len();

    if *segments_len >= 1 {
        ftype = match segments[0] {
            "T" => FunctionType::T,
            "Tp" => FunctionType::Tp,
            "D" => FunctionType::D,
            "Dp" => FunctionType::Dp,
            "DD" => FunctionType::DD,
            "D/" => FunctionType::DNonComplete,
            "S" => FunctionType::S,
            "Sp" => FunctionType::Sp,
            _ => FunctionType::Spacer,
        };
    }

    if *segments_len >= 2 {
        fcolor = match segments[1] {
            "2" => FunctionColor::Fc2,
            "3" => FunctionColor::Fc3,
            "4" => FunctionColor::Fc4,
            "5" => FunctionColor::Fc5,
            "6" => FunctionColor::Fc6,
            "65" => FunctionColor::Fc65,
            "64" => FunctionColor::Fc64,
            "7" => FunctionColor::Fc7,
            "9" => FunctionColor::Fc9,
            "9b" => FunctionColor::Fc9flat,
            _ => FunctionColor::FcNone,
        };
    }

    if *segments_len >= 3 {
        fbass = match segments[2] {
            "3" => FunctionBass::Fb3,
            "5" => FunctionBass::Fb5,
            "7" => FunctionBass::Fb7,
            _ => FunctionBass::FbNone,
        };
    };

    if s.contains("(") {
        start_par = true;
    }

    if s.contains(")") {
        end_par = true;
    }

    Some((ftype, fcolor, fbass, start_par, end_par))
}

pub fn parse_tie(s: &str) -> Option<TieFromType> {
    if s.contains("_") {
        return Some(TieFromType::Standard);
    }
    if s[1..].contains("~") {
        return Some(TieFromType::LetRing);
    }
    None
}

pub(crate) fn parse_tie_to(s: &str) -> Option<TieToType> {
    if s.starts_with("~") {
        return Some(TieToType::LetRing);
    }
    None
}

pub(crate) fn parse_line(s: &str) -> Option<HeadLine> {
    if s.contains("LH") {
        return Some(HeadLine(0, 0, HeadLineType::Halfstep));
    }
    if s.contains("LW") {
        return Some(HeadLine(0, 0, HeadLineType::Wholestep));
    }
    if s.contains("LR") {
        return Some(HeadLine(0, 0, HeadLineType::LineColor(NColor::Tomato)));
    }
    if s.contains("LB") {
        return Some(HeadLine(0, 0, HeadLineType::LineColor(NColor::Dodgerblue)));
    }
    if s.contains("LG") {
        return Some(HeadLine(0, 0, HeadLineType::LineColor(NColor::Lime)));
    }
    if s.contains("L") {
        return Some(HeadLine(0, 0, HeadLineType::Line));
    }

    None
}

pub(crate) fn parse_articulation(s: &str) -> (String, NoteArticulation) {
    if s.contains("-.") {
        return (s.replace("=.", ""), NoteArticulation::TenutoStaccato);
    }
    if s.contains(">>") {
        return (s.replace(">>", ""), NoteArticulation::MarcatoStaccato);
    }
    //------------------------------
    if s.contains(".") {
        return (s.replace(".", ""), NoteArticulation::Staccato);
    }
    if s.contains("-") {
        return (s.replace("=", ""), NoteArticulation::Tenuto);
    }
    if s.contains(">") {
        return (s.replace(">", ""), NoteArticulation::Marcato);
    }
    (s.to_string(), NoteArticulation::None)
}

pub fn rect_x(rect: &NRect, nrects: Vec<NRectExt>) -> f32 {
    for nrect in &nrects {
        let cmp: NRect = nrect.0;
    }

    0.0
}
