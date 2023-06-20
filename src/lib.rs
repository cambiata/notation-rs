#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(clippy::uninlined_format_args)]
#![allow(clippy::useless_format)]

//  cargo watch -q -c -w src -x "test beaming"

pub mod beaming;
pub mod chord;
pub mod complex;
pub mod complexext;
pub mod core;
pub mod dynamic;
pub mod error;
pub mod head;
pub mod heads;
pub mod note;
pub mod noterects;
pub mod notes;
pub mod part;
pub mod prelude;
pub mod quick;
pub mod render;
pub mod syllable;
pub mod utils;
pub mod voice;

#[cfg(test)]
mod tests {
    use super::core::*;
    use super::head::*;
    use super::heads::*;
    use super::note::*;
    use super::notes::*;
    use super::quick::QCode;

    #[test]
    fn lib_serialize() {
        let heads = Heads::new(vec![
            Head::new(1, None, HeadAttributes {}),
            Head::new(0, None, HeadAttributes {}),
            Head::new(-2, None, HeadAttributes {}),
            Head::new(4, None, HeadAttributes {}),
        ]);

        let note = Note::new(NV2, NoteType::Heads(heads), NoteAttributes { color: None });
        let note_json = serde_json::to_string(&note).unwrap();
        println!("note_json:{:?}", note_json);
        std::fs::write("test_note.json", &note_json).unwrap();
        let note2: Note = serde_json::from_str(&note_json).unwrap();
        println!("note2:{:?}", note2);
    }

    #[test]
    fn lib_code() {
        let notes: Notes = QCode::notes("0 1,2 nv2 -4 p ").unwrap();
        println!("notes:{:?}", notes);
        let notes_json = serde_json::to_string(&notes).unwrap();
        println!("notes_json:{:?}", notes_json);
        std::fs::write("test_notes.json", &notes_json).unwrap();
    }
}
