use crate::{
    core::{Duration, DurationTools, NV4},
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
                    let s = &segment[2..];
                    cur_val = DurationTools::from_str(s).ok();
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
                    let mut heads: Vec<Head> = vec![];
                    for segment in &segments {
                        let level = crate::utils::parse_string_to_int(segment)?;
                        let accidental = crate::utils::parse_accidental(segment);
                        heads.push(Head::new(level as i8, accidental, HeadAttributes {}));
                    }

                    let n = Note::new(
                        cur_val.unwrap_or(NV4),
                        NoteType::Heads(Heads::new(heads)),
                        NoteAttributes { color: None },
                    );
                    // let n = Note::new(24, NoteType::Dummy, NoteAttributes { color: None });
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
            let mut barpause_value: Duration = 0;
            for segment in segments {
                if segment.to_lowercase().starts_with("nv") {
                    let dur = DurationTools::from_str(&segment[2..]);
                    match dur {
                        Ok(d) => barpause_value += d,
                        Err(e) => {}
                    }
                }
            }

            let barpause_value: Option<Duration> = if barpause_value == 0 {
                None
            } else {
                Some(barpause_value)
            };

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
            println!("- note:{:?}", note.duration);
        }
    }

    #[test]
    fn test_accidentals() {
        let notes: Notes = QCode::notes("1 b2 #-3 n4").unwrap();
        for note in &notes {
            println!("- note:{:?}", note);
        }
    }

    #[test]
    fn test_voice() {
        // let voice = QCode::voice("nv4 0 0 0 0");
        let voice = QCode::voice("bp x nv2 y nv8 z").unwrap();
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
