use crate::core::*;
use crate::note::*;
use serde::{Deserialize, Serialize};
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Notes {
    pub items: Vec<Note>,
    pub val: u32,
}

impl Notes {
    pub fn new(items: Vec<Note>) -> Self {
        let val = &items.iter().fold(0, |sum, item| item.val as i32 + sum);

        Self {
            items,
            val: *val as u32,
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

struct NotesPositions<'a> {
    items: &'a Vec<&'a Note>,
    count: usize,
    pos: usize,
}

impl<'a> NotesPositions<'a> {
    fn new(items: &'a Vec<&'a Note>) -> Self {
        Self {
            items,
            count: 0,
            pos: 0,
        }
    }
}

impl<'a> Iterator for NotesPositions<'a> {
    type Item = (usize, usize, &'a Note);
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.items.len() {
            let item = self.items[self.count];
            self.count += 1;
            let current_pos = self.pos;
            self.pos += item.val as usize;
            return Some((self.count, current_pos, item));
        }
        None
    }
}

#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    use super::Note;
    use super::NoteAttributes;
    use super::Notes;
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
        let notes = QCode::notes("0 0 0 0");
        let items: Vec<&Note> = notes.items.iter().map(|i| i).collect();
        // let items2: Vec<&Note> = notes.into();
        // NotesPositions::new(&items);
    }
}
