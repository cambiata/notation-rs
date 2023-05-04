mod beaming;
mod complex;
mod core;
mod head;
mod heads;
mod note;
mod notes;
mod quick;
mod voice;

#[cfg(test)]
mod tests {
    use super::core::*;
    use super::head::*;
    use super::heads::*;
    use super::note::*;
    use super::notes::*;
    use super::quick::QCode;

    // #[test]
    // fn it_works() {
    //     let head = Head::new(1, HeadAttributes { accidental: None });
    //     println!("{:?}", head);

    //     let note = Note::new(
    //         NValue::Nv2,
    //         NoteType::Heads(Heads { items: vec![head] }),
    //         NoteAttributes { color: None },
    //     );
    //     println!("{:?}", note);

    //     let heads = Heads::new(vec![
    //         Head::new(1, HeadAttributes { accidental: None }),
    //         Head::new(0, HeadAttributes { accidental: None }),
    //         Head::new(-2, HeadAttributes { accidental: None }),
    //         Head::new(4, HeadAttributes { accidental: None }),
    //     ]);
    //     println!("{:?}", heads);
    //     println!("head:{:?}", head);
    // }

    #[test]
    fn test_serialize() {
        let heads = Heads::new(vec![
            Head::new(1, HeadAttributes { accidental: None }),
            Head::new(0, HeadAttributes { accidental: None }),
            Head::new(-2, HeadAttributes { accidental: None }),
            Head::new(4, HeadAttributes { accidental: None }),
        ]);

        let note = Note::new(NV2, NoteType::Heads(heads), NoteAttributes { color: None });

        let note_json = serde_json::to_string(&note).unwrap();
        println!("note_json:{:?}", note_json);
        std::fs::write("test_note.json", &note_json).unwrap();
        let note2: Note = serde_json::from_str(&note_json).unwrap();
        println!("note2:{:?}", note2);
    }

    #[test]
    fn test_code() {
        let notes: Notes = QCode::notes("0 1,2 nv2 -4 p ");
        println!("notes:{:?}", notes);
        let notes_json = serde_json::to_string(&notes).unwrap();
        println!("notes_json:{:?}", notes_json);
        std::fs::write("test_notes.json", &notes_json).unwrap();
    }
}
