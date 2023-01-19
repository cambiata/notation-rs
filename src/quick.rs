use crate::{core::NValue, head::*, heads::*, note::*, notes::*};
pub struct QCode;

impl QCode {
    pub fn notes(code: &str) -> Notes {
        let segments: Vec<&str> = code.trim().split(' ').collect();
        let mut cur_val: NValue = NValue::Nv4;
        let mut notes: Vec<Note> = vec![];
        for segment in segments {
            match segment {
                a if a.starts_with("nv") => {
                    cur_val = NValue::from_str(segment);
                }
                "p" => {
                    println!("pause:{}", segment);
                    let n = Note {
                        val: cur_val.clone(),
                        ntype: NoteType::Pause,
                        attr: NoteAttributes { color: None },
                    };
                    notes.push(n);
                }
                _ => {
                    let segments: Vec<&str> = segment.split(',').collect();
                    let levels: Vec<i8> = segments
                        .iter()
                        .map(|s| str::parse::<i8>(s).unwrap_or(0))
                        .collect();
                    let items: Vec<Head> = levels
                        .iter()
                        .map(|level| Head::new(*level, HeadAttributes { accidental: None }))
                        .collect();
                    let heads = Heads::new(items);
                    let n = Note::new(
                        cur_val.clone(),
                        NoteType::Heads(heads),
                        NoteAttributes { color: None },
                    );
                    notes.push(n);
                }
            }
        }
        Notes::new(notes)
    }
}

#[cfg(test)]
mod tests {
    use super::QCode;
    use crate::core::*;
    use crate::head::*;
    use crate::note::*;
    use crate::notes::*;
    #[test]
    fn test_code() {
        let notes: Notes = QCode::notes("0 1,2 nv2 -4 p ");
        println!("notes:{:?}", notes);
        for note in &notes {
            println!("- note:{:?}", note);
        }
    }
}
