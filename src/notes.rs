use crate::core::*;
use crate::note::*;

use serde::{Deserialize, Serialize};
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Notes {
    pub items: Vec<Note>,
    pub value: usize,
}

impl Notes {
    pub fn new(items: Vec<Note>) -> Self {
        let value = &items.iter().fold(0, |sum, item| item.value as i32 + sum);

        Self {
            items,
            value: *value as usize,
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Note> {
        self.into_iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Note> {
        self.into_iter()
    }
}

impl<'a> IntoIterator for &'a Notes {
    type Item = &'a Note;

    type IntoIter = std::slice::Iter<'a, Note>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}

impl<'a> IntoIterator for &'a mut Notes {
    type Item = &'a mut Note;

    type IntoIter = std::slice::IterMut<'a, Note>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter_mut()
    }
}

//===============================================================

pub struct NotesPositions<'a> {
    notes: &'a Notes,
    idx: usize,
    pos: usize,
}

impl<'a> NotesPositions<'a> {
    pub fn new(notes: &'a Notes) -> Self {
        Self {
            notes,
            idx: 0,
            pos: 0,
        }
    }
}

impl<'a> Iterator for NotesPositions<'a> {
    type Item = (usize, usize, usize, &'a Note);
    fn next(&mut self) -> Option<Self::Item> {
        // if self.idx < self.values.len() {
        if self.idx < self.notes.items.len() {
            let n = &self.notes.items[self.idx];
            let cur_idx = self.idx;
            let cur_pos = self.pos;
            let end_pos = cur_pos + n.value as usize;
            self.idx += 1;
            self.pos += n.value as usize;
            return Some((cur_idx, cur_pos, end_pos, n));
        }
        None
    }
}

//===============================================================
struct NotesPairs<'a> {
    notes: &'a Notes,
    idx: usize,
}

impl<'a> NotesPairs<'a> {
    fn new(notes: &'a Notes) -> Self {
        Self { notes, idx: 0 }
    }
}

impl<'a> Iterator for NotesPairs<'a> {
    type Item = (usize, Option<&'a Note>, Option<&'a Note>);
    fn next(&mut self) -> Option<Self::Item> {
        match self.notes.items.len() {
            0 => None,
            1 => {
                if (self.idx == 0) {
                    self.idx += 1;
                    return Some((0, Some(&self.notes.items[0]), None));
                }
                None
            }
            _ => {
                if self.idx < self.notes.items.len() - 1 {
                    let n1 = &self.notes.items[self.idx];
                    let n2 = &self.notes.items[self.idx + 1];
                    let cur_idx = self.idx;
                    self.idx += 1;
                    return Some((cur_idx, Some(n1), Some(n2)));
                }
                None
            }
        }
    }
}

//===============================================================

#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    use super::Note;
    use super::NoteAttributes;
    use super::Notes;
    use super::NotesPairs;
    use super::NotesPositions;
    use crate::quick::QCode;
    #[test]
    fn test_notes_constructor() {
        let notes = QCode::notes("nv4 0 nv8 1");

        for note in &notes {
            println!("- note:{:?}", note);
        }
        println!("notes:{:?}", notes);
    }

    #[test]
    fn notes_positions() {
        let notes = QCode::notes("0 1 2 3");
        let notes_positions = NotesPositions::new(&notes);
        for n in notes_positions {
            println!("v:{:?}", n);
        }
    }
    #[test]
    fn notes_pairs() {
        let notes = QCode::notes("0");
        let pairs = NotesPairs::new(&notes);
        for n in pairs {
            println!("n:{:?}", n);
        }
    }
}
