use crate::{
    core::{Dur, NV4},
    // core::Duration,
    head::*,
    heads::*,
    note::*,
    notes::*,
    prelude::*,
    voice::{BarPause, Voice, VoiceAttributes, VoiceType},
};
pub struct QCode;

impl QCode {
    pub fn notes(code: &str) -> Result<Notes> {
        let segments: Vec<&str> = code.trim().split(' ').collect();
        let mut cur_val: Option<usize> = None;
        let mut notes: Vec<Note> = vec![];
        for segment in segments {
            match segment {
                a if a.to_lowercase().starts_with("nv") => {
                    cur_val = Dur::from_str(segment);
                }
                "p" => {
                    println!("pause:{segment}");
                    let value: usize = cur_val.unwrap_or(NV4);
                    let n = Note {
                        duration: value,
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
                        cur_val.unwrap_or(NV4),
                        NoteType::Heads(heads),
                        NoteAttributes { color: None },
                    );
                    notes.push(n);
                }
            }
        }
        Ok(Notes::new(notes))
    }

    pub fn voice(code: &str) -> Result<Voice> {
        let vtype = if code.contains("bp") {
            let c = code.replace("bp", "");
            let c2 = c.trim();
            let segments = c2.split(' ').collect::<Vec<&str>>();
            let mut barpause_value: usize = 0;
            for segment in segments {
                if let Some(v) = Dur::from_str(segment) {
                    barpause_value += v;
                }
            }
            VoiceType::VBarpause(BarPause(barpause_value))
        } else {
            let notes = QCode::notes(code)?;
            VoiceType::VNotes(notes)
        };
        Ok(Voice::new(vtype, VoiceAttributes {}))
    }

    pub fn voices(code: &str) -> Result<Vec<Voice>> {
        let segments: Vec<&str> = code.trim().split('/').collect();
        let mut voices: Vec<Voice> = vec![];
        if segments.len() > 2 {
            panic!("too many voices: {}", voices.len());
        }
        for segment in segments {
            let voice = QCode::voice(segment)?;
            voices.push(voice);
        }
        Ok(voices)
    }
}

#[cfg(test)]
mod tests {
    use super::QCode;

    use crate::notes::*;

    #[test]
    fn test_notes() {
        let notes: Notes = QCode::notes("0 1,2 nv2 -4 p ").unwrap();
        println!("notes:{:?}", notes);
        for note in &notes {
            println!("- note:{:?}", note);
        }
    }

    #[test]
    fn test_voice() {
        // let voice = QCode::voice("nv4 0 0 0 0");
        let voice = QCode::voice("bp nv2 nv8 x").unwrap();
        println!("voice:{:?}", voice);
    }

    #[test]
    fn test_voices() {
        let voices = QCode::voices("nv4 0 0 0 0 / bp Nv2dot").unwrap();
        for voice in voices.iter() {
            println!("- voice:{:?}", voice);
        }
    }
}
