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
            "53" => FunctionColor::Fc53,
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

fn parse_chord_root(r: &str) -> ChordRoot {
    if r.contains("Cb") {
        return ChordRoot::CFlat;
    } else if r.contains("C#") {
        return ChordRoot::CSharp;
    } else if r.contains("Db") {
        return ChordRoot::DFlat;
    } else if r.contains("D#") {
        return ChordRoot::DSharp;
    } else if r.contains("Eb") {
        return ChordRoot::EFlat;
    } else if r.contains("E#") {
        return ChordRoot::ESharp;
    } else if r.contains("Fb") {
        return ChordRoot::FFlat;
    } else if r.contains("F#") {
        return ChordRoot::FSharp;
    } else if r.contains("Gb") {
        return ChordRoot::GFlat;
    } else if r.contains("G#") {
        return ChordRoot::GSharp;
    } else if r.contains("Ab") {
        return ChordRoot::AFlat;
    } else if r.contains("A#") {
        return ChordRoot::ASharp;
    } else if r.contains("Bb") {
        return ChordRoot::BFlat;
    } else if r.contains("B#") {
        return ChordRoot::BSharp;
    } else if r.contains("C") {
        return ChordRoot::CNatural;
    } else if r.contains("D") {
        return ChordRoot::DNatural;
    } else if r.contains("E") {
        return ChordRoot::ENatural;
    } else if r.contains("F") {
        return ChordRoot::FNatural;
    } else if r.contains("G") {
        return ChordRoot::GNatural;
    } else if r.contains("A") {
        return ChordRoot::ANatural;
    } else if r.contains("B") {
        return ChordRoot::BNatural;
    }
    ChordRoot::None
}

pub(crate) fn parse_chord(s: &str) -> Option<(ChordRoot, ChordFlavour, ChordColor, ChordRoot)> {
    let mut croot: ChordRoot = ChordRoot::DSharp;
    let mut cflavour: ChordFlavour = ChordFlavour::Major;
    let mut ccolor: ChordColor = ChordColor::None;
    let mut cbass: ChordRoot = ChordRoot::None;

    let mut s: String = s.to_string();

    if s.contains("sus2") {
        ccolor = ChordColor::SusTwo;
    } else if s.contains("sus4") {
        ccolor = ChordColor::SusFour;
    } else if s.contains("maj7") {
        ccolor = ChordColor::MajSeven;
        s = s.replace("maj7", "");
    } else if s.contains("#5") {
        ccolor = ChordColor::PlusFive;
    } else if s.contains("#9") {
        ccolor = ChordColor::PlusNine;
    } else if s.contains("b9") {
        ccolor = ChordColor::MinusNine;
    } else if s.contains("5") {
        ccolor = ChordColor::Five;
    } else if s.contains("6") {
        ccolor = ChordColor::Six;
    } else if s.contains("7") {
        ccolor = ChordColor::Seven;
    };

    if s.contains('m') {
        cflavour = ChordFlavour::Minor;
    }

    let x = s.split(':').collect::<Vec<&str>>();
    croot = parse_chord_root(x[0]);

    if x.len() > 1 {
        cbass = parse_chord_root(x[1]);
    }

    // dbg!(croot, cbass);

    Some((croot, cflavour, ccolor, cbass))
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

pub(crate) fn parse_color(s: &str) -> Option<NColor> {
    if s.contains("CRed") {
        return Some(NColor::Red);
    }
    if s.contains("CBlu") {
        return Some(NColor::Blue);
    }
    if s.contains("CGre") {
        return Some(NColor::Green);
    }
    if s.contains("COra") {
        return Some(NColor::Orange);
    }
    if s.contains("CTom") {
        return Some(NColor::Tomato);
    }
    if s.contains("CDod") {
        return Some(NColor::Dodgerblue);
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

pub(crate) fn parse_symbol(s: &str) -> Option<(String, SymbolType)> {
    if s.contains("2up") {
        return Some((s.to_string(), SymbolType::ChordProgress2ndUp));
    }

    if s.contains("2down") {
        return Some((s.to_string(), SymbolType::ChordProgress2ndDown));
    }
    if s.contains("3up") {
        return Some((s.to_string(), SymbolType::ChordProgress3rdUp));
    }

    if s.contains("3down") {
        return Some((s.to_string(), SymbolType::ChordProgress3rdDown));
    }
    if s.contains("5up") {
        return Some((s.to_string(), SymbolType::ChordProgress5thUp));
    }

    if s.contains("5down") {
        return Some((s.to_string(), SymbolType::ChordProgress5thDown));
    }

    return Some((s.to_string(), SymbolType::Square(2.0)));
}

pub fn rect_x(rect: &NRect, nrects: Vec<NRectExt>) -> f32 {
    for nrect in &nrects {
        let cmp: NRect = nrect.0;
    }

    0.0
}

pub fn chord_guess_width(chord_root: &ChordRoot, chord_flavour: &ChordFlavour, chord_color: &ChordColor, chord_bass: &ChordRoot) -> f32 {
    let mut width = 0.0;

    match chord_root {
        ChordRoot::None => {}
        ChordRoot::CNatural | ChordRoot::CFlat | ChordRoot::CSharp => width += 60.0,
        ChordRoot::DNatural | ChordRoot::DFlat | ChordRoot::DSharp => width += 70.0,
        ChordRoot::ENatural | ChordRoot::EFlat | ChordRoot::ESharp => width += 50.0,
        ChordRoot::FNatural | ChordRoot::FFlat | ChordRoot::FSharp => width += 50.0,
        ChordRoot::GNatural | ChordRoot::GFlat | ChordRoot::GSharp => width += 60.0,
        ChordRoot::ANatural | ChordRoot::AFlat | ChordRoot::ASharp => width += 55.0,
        ChordRoot::BNatural | ChordRoot::BFlat | ChordRoot::BSharp => width += 55.0,
    };

    match chord_root {
        ChordRoot::CFlat
        | ChordRoot::CSharp
        | ChordRoot::DFlat
        | ChordRoot::DSharp
        | ChordRoot::EFlat
        | ChordRoot::ESharp
        | ChordRoot::FFlat
        | ChordRoot::FSharp
        | ChordRoot::GFlat
        | ChordRoot::GSharp
        | ChordRoot::AFlat
        | ChordRoot::ASharp
        | ChordRoot::BFlat
        | ChordRoot::BSharp => width += 25.0,
        _ => width += 0.0,
    };

    match chord_flavour {
        ChordFlavour::Minor => {
            width += 60.0;
        }
        _ => {}
    }
    match chord_color {
        ChordColor::None => {}
        ChordColor::SusTwo => width += 25.0 + 75.0,
        ChordColor::SusFour => width += 25.0 + 75.0,
        ChordColor::MajSeven => width += 100.0,
        ChordColor::Five => width += 25.0,
        ChordColor::PlusFive => width += 25.0,
        ChordColor::Six => width += 25.0,
        ChordColor::Seven => width += 30.0,
        ChordColor::Nine => width += 25.0,
        ChordColor::MinusNine => width += 35.0,
        ChordColor::PlusNine => width += 25.0,
        ChordColor::Thirteen => width += 25.0,
    }

    match chord_bass {
        ChordRoot::None => {}
        _ => {
            width += 28.0;

            match chord_bass {
                ChordRoot::None => {}
                ChordRoot::CNatural | ChordRoot::CFlat | ChordRoot::CSharp => width += 50.0,
                ChordRoot::DNatural | ChordRoot::DFlat | ChordRoot::DSharp => width += 60.0,
                ChordRoot::ENatural | ChordRoot::EFlat | ChordRoot::ESharp => width += 50.0,
                ChordRoot::FNatural | ChordRoot::FFlat | ChordRoot::FSharp => width += 50.0,
                ChordRoot::GNatural | ChordRoot::GFlat | ChordRoot::GSharp => width += 60.0,
                ChordRoot::ANatural | ChordRoot::AFlat | ChordRoot::ASharp => width += 55.0,
                ChordRoot::BNatural | ChordRoot::BFlat | ChordRoot::BSharp => width += 55.0,
            };

            match chord_bass {
                ChordRoot::CFlat
                | ChordRoot::CSharp
                | ChordRoot::DFlat
                | ChordRoot::DSharp
                | ChordRoot::EFlat
                | ChordRoot::ESharp
                | ChordRoot::FFlat
                | ChordRoot::FSharp
                | ChordRoot::GFlat
                | ChordRoot::GSharp
                | ChordRoot::AFlat
                | ChordRoot::ASharp
                | ChordRoot::BFlat
                | ChordRoot::BSharp => width += 30.0,
                _ => width += 0.0,
            };
        }
    }

    width
}
