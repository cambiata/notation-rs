use std::{cell::Ref, fmt::Formatter};

use crate::prelude::*;

#[derive(Debug, PartialEq)]
pub enum NoteType {
    Heads(crate::head::Heads),
    Pause,
    // Slash,
    Lyric(Syllable),
    // Dynamic(DynamicItem),
    Spacer(i8),
    Tpl(char, TplOctave, TplAccidental, i8),
    Symbol(SymbolType),
    Function(FunctionType, FunctionColor, FunctionBass, bool, bool),
    ChordSymbol(ChordRoot, ChordFlavour, ChordColor, ChordRoot),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ChordRoot {
    None,
    CFlat,
    CNatural,
    CSharp,
    DFlat,
    DNatural,
    DSharp,
    EFlat,
    ENatural,
    ESharp,
    FFlat,
    FNatural,
    FSharp,
    GFlat,
    GNatural,
    GSharp,
    AFlat,
    ANatural,
    ASharp,
    BFlat,
    BNatural,
    BSharp,
}
impl ChordRoot {
    pub fn get_char(self: ChordRoot) -> char {
        match self {
            ChordRoot::None => ' ',
            ChordRoot::CFlat | ChordRoot::CNatural | ChordRoot::CSharp => 'C',
            ChordRoot::DFlat | ChordRoot::DNatural | ChordRoot::DSharp => 'D',
            ChordRoot::EFlat | ChordRoot::ENatural | ChordRoot::ESharp => 'E',
            ChordRoot::FFlat | ChordRoot::FNatural | ChordRoot::FSharp => 'F',
            ChordRoot::GFlat | ChordRoot::GNatural | ChordRoot::GSharp => 'G',
            ChordRoot::AFlat | ChordRoot::ANatural | ChordRoot::ASharp => 'A',
            ChordRoot::BFlat | ChordRoot::BNatural | ChordRoot::BSharp => 'B',
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ChordFlavour {
    Major,
    Minor,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ChordColor {
    None,
    SusTwo,
    SusFour,
    Five,
    PlusFive,
    Six,
    Seven,
    MajSeven,
    Nine,
    MinusNine,
    PlusNine,
    Thirteen,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SymbolType {
    RightArrow,
    LeftArrow,
    Square(f32),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum NoteComplexType {
    Unset,
    Single,
    Upper,
    Lower,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum FunctionColor {
    FcNone,
    Fc2,
    Fc3,
    Fc4,
    Fc5,
    Fc53,
    Fc6,
    Fc65,
    Fc64,
    Fc7,
    Fc9,
    Fc9flat,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum FunctionBass {
    FbNone,
    Fb3,
    Fb5,
    Fb7,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum FunctionType {
    Spacer,
    T,
    Sp,
    DD,
    Dp,
    S,
    D,
    DNonComplete,
    Tp,
    Tk,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum TplOctave {
    Higher,
    Mid,
    Lower,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum TplAccidental {
    Raised,
    Neutral,
    Lowered,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum NoteArticulation {
    None,
    Staccato,
    Tenuto,
    TenutoStaccato,
    Marcato,
    MarcatoStaccato,
    MoltoMarcato,
}

#[derive(PartialEq)]
pub struct Note {
    pub id: usize,

    pub ntype: NoteType,
    pub duration: Duration,
    pub attr: NoteAttributes,

    //-------------------------------------------
    // calculated
    pub position: Position,
    pub end_position: Position,

    pub voice: Option<Rc<RefCell<Voice>>>,
    pub beamgroup: Option<Rc<RefCell<Beamgroup>>>,
    pub direction: Option<DirUD>,
    pub ties: Vec<TieData>,
    pub ties_to: Vec<TieToData>,
    pub articulation: NoteArticulation,

    pub adjust_x: Option<(f32, f32)>,
    pub complex_type: NoteComplexType,

    pub lines: Vec<HeadLine>,
    pub lines_to: Vec<HeadLineTo>,
}

impl Note {
    pub fn new(mut ntype: NoteType, duration: Duration) -> Self {
        let id = ID_COUNTER.fetch_add(1, Ordering::Relaxed);
        let mut ties: Vec<TieData> = Vec::new();
        let mut ties_to: Vec<TieToData> = Vec::new();

        let mut lines: Vec<HeadLine> = Vec::new();
        let mut lines_to: Vec<HeadLineTo> = Vec::new();

        match ntype {
            NoteType::Heads(ref mut heads) => {
                for (headidx, head) in heads.heads.iter().enumerate() {
                    let head: Ref<Head> = head.borrow();

                    if let Some(tie) = &head.tie {
                        ties.push(TieData {
                            note_id: id,
                            ttype: tie.clone(),
                            level: head.level,
                        });
                    }
                    if let Some(tie) = &head.tie_to {
                        ties_to.push(TieToData {
                            note_id: id,
                            ttype: tie.clone(),
                            level: head.level,
                        });
                    }

                    if let Some(line) = &head.line {
                        lines.push(HeadLine(headidx as u8, headidx as u8, line.2));
                    }
                    // if let Some(line_to) = &head.line_to {
                    //     lines.push(HeadLine(headidx as u8, headidx as u8, line_to.2));
                    // }
                }
            }
            _ => {}
        }

        Self {
            id,
            ntype,
            duration,
            attr: NoteAttributes { color: None },
            position: 0,
            end_position: 0,
            beamgroup: None,
            voice: None,
            direction: None,
            ties,
            ties_to,
            adjust_x: None,
            articulation: NoteArticulation::None,
            complex_type: NoteComplexType::Unset,
            lines,
            lines_to,
        }
    }

    pub fn has_stem(&self) -> bool {
        match &self.ntype {
            NoteType::Heads(heads) => duration_has_stem(&self.duration),
            _ => false,
        }
    }

    pub fn is_beamable(self: &Note) -> bool {
        match self.ntype {
            NoteType::Heads(_) => duration_is_beamable(&self.duration),
            _ => false,
        }
    }

    pub fn head_levels(&self) -> Vec<i8> {
        match &self.ntype {
            NoteType::Heads(heads) => heads.levels(),
            _ => Vec::new(),
        }
    }

    pub fn top_level(&self) -> i8 {
        match &self.ntype {
            NoteType::Heads(heads) => heads.top,
            _ => 0,
        }
    }

    pub fn bottom_level(&self) -> i8 {
        match &self.ntype {
            NoteType::Heads(heads) => heads.bottom,
            _ => 0,
        }
    }

    pub fn levels_accidentals(&self) -> Vec<(i8, Accidental)> {
        match &self.ntype {
            NoteType::Heads(heads) => heads.levels_accidentals(),
            _ => Vec::new(),
        }
    }

    pub fn is_heads(&self) -> bool {
        match &self.ntype {
            NoteType::Heads(_) => true,
            _ => false,
        }
    }

    pub fn is_pause(&self) -> bool {
        match &self.ntype {
            NoteType::Pause => true,
            _ => false,
        }
    }

    pub fn has_level(&self, level: i8) -> bool {
        match &self.ntype {
            NoteType::Heads(heads) => heads.has_level(level),
            _ => false,
        }
    }

    pub fn has_tie_to(&self, level: i8) -> Option<TieToData> {
        for tie_to in self.ties_to.iter() {
            if tie_to.level == level {
                return Some(tie_to.clone());
            }
        }
        None
    }
    pub fn get_level_tie_to(&self, level: i8) -> Option<TieToType> {
        match &self.ntype {
            NoteType::Heads(heads) => heads.get_level_tie_to(level),
            _ => None,
        }
    }

    pub fn get_head(&self, level: i8) -> Option<Rc<RefCell<Head>>> {
        match &self.ntype {
            NoteType::Heads(heads) => heads.get_head(level),
            _ => None,
        }
    }

    pub fn get_heads(&self) -> Option<Vec<Rc<RefCell<Head>>>> {
        match &self.ntype {
            NoteType::Heads(heads) => Some(heads.heads.clone()),
            _ => None,
        }
    }
}

pub type NotesChunk = Vec<Rc<RefCell<Note>>>;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum SyllableType {
    Text(String),
    TextWithHyphen(String),
    Hyphen,
    Extension(i32), // length
}

#[derive(Debug, PartialEq)]
pub struct Syllable {
    pub syllable_type: SyllableType,
}

impl Syllable {
    pub fn new(syllable_type: SyllableType) -> Self {
        Self { syllable_type }
    }
}

impl Debug for Note {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.ntype {
            NoteType::Heads(heads) => {
                write!(f, "Note id:{} pos:{} end:{} dur:{} heads:{:?}", self.id, self.position, self.end_position, self.duration, heads)
            }
            NoteType::Pause => {
                write!(f, "Note PAUSE id:{} pos:{} end:{} dur:{} pause", self.id, self.position, self.end_position, self.duration)
            }
            // NoteType::Lyric(syllable) => {
            //     write!(
            //         f,
            //         "Note LYRIC id:{} pos:{} end:{} dur:{} lyric:{:?}",
            //         self.id, self.position, self.end_position, self.duration, syllable
            //     )
            // }
            _ => {
                write!(f, "Note OTHER TYPE id:{} pos:{} end:{} dur:{}", self.id, self.position, self.end_position, self.duration)
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Notes {
    pub items: Vec<Rc<RefCell<Note>>>,
    pub duration: Duration,
}

impl Notes {
    pub fn new(items: Vec<Note>) -> Self {
        let items: Vec<Rc<RefCell<Note>>> = items.into_iter().map(|item| Rc::new(RefCell::new(item))).collect();

        let duration = items.iter().fold(0, |acc, item| {
            let mut item_mut = item.borrow_mut();
            item_mut.position = acc;
            item_mut.end_position = acc + item_mut.duration;
            acc + item_mut.duration
        });

        Self { items, duration }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct NoteAttributes {
    pub color: Option<u16>,
}

#[cfg(test)]
mod tests2 {
    use crate::prelude::*;
    #[test]
    fn example() {
        let notes = QCode::notes("nv8 0 1 2 nv16 3 2 0 1 0 1 nv8dot 2 3").unwrap();
        // let json = serde_json::to_string_pretty(&notes).unwrap();
        // println!("{}", json);
        // let notes2 = serde_json::from_str::<Notes>(&json).unwrap();
    }
}
