use std::fmt::Formatter;

use crate::prelude::*;

#[derive(Debug, PartialEq)]
pub enum NoteType {
    Heads(crate::head::Heads),
    Pause,
    // Slash,
    Lyric(Syllable),
    // Dynamic(DynamicItem),
    // Chord(ChordItem),
    // Spacer,
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
}

impl Note {
    pub fn new(mut ntype: NoteType, duration: Duration) -> Self {
        Self {
            id: ID_COUNTER.fetch_add(1, Ordering::Relaxed),
            ntype,
            duration,
            attr: NoteAttributes { color: None },
            position: 0,
            end_position: 0,
            beamgroup: None,
            voice: None,
            direction: None,
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
}

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
                write!(
                    f,
                    "Note id:{} pos:{} end:{} dur:{} heads:{:?}",
                    self.id, self.position, self.end_position, self.duration, heads
                )
            }
            NoteType::Pause => {
                write!(
                    f,
                    "Note PAUSE id:{} pos:{} end:{} dur:{} pause",
                    self.id, self.position, self.end_position, self.duration
                )
            }
            // NoteType::Lyric(syllable) => {
            //     write!(
            //         f,
            //         "Note LYRIC id:{} pos:{} end:{} dur:{} lyric:{:?}",
            //         self.id, self.position, self.end_position, self.duration, syllable
            //     )
            // }
            _ => {
                write!(
                    f,
                    "Note OTHER TYPE id:{} pos:{} end:{} dur:{}",
                    self.id, self.position, self.end_position, self.duration
                )
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
        let items: Vec<Rc<RefCell<Note>>> = items
            .into_iter()
            .map(|item| Rc::new(RefCell::new(item)))
            .collect();

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
