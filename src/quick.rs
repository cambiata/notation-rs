use crate::{
    core::NValue,
    head::*,
    heads::*,
    note::*,
    notes::*,
    voice::{Voice, VoiceAttributes, VoiceType},
    voices::Voices,
};
pub struct QCode;

impl QCode {
    pub fn notes(code: &str) -> Notes {
        let segments: Vec<&str> = code.trim().split(' ').collect();
        let mut cur_val: NValue = NValue::Nv4;
        let mut notes: Vec<Note> = vec![];
        for segment in segments {
            match segment {
                a if a.to_lowercase().starts_with("nv") => {
                    cur_val = NValue::from_str(segment);
                }
                "p" => {
                    println!("pause:{}", segment);
                    let n = Note {
                        value: cur_val,
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
                        cur_val,
                        NoteType::Heads(heads),
                        NoteAttributes { color: None },
                    );
                    notes.push(n);
                }
            }
        }
        Notes::new(notes)
    }

    pub fn voice(code: &str) -> Voice {
        let vtype = if code.contains("bp") {
            let c = code.replace("bp", "");
            let c2 = c.trim();
            let segments = c2.split(' ').collect::<Vec<&str>>();
            let mut barpause_value: usize = 0;
            for segment in segments {
                if let Some(v) = NValue::from_str_option(segment) {
                    barpause_value += v as usize;
                }
            }
            VoiceType::VBarpause(barpause_value)
        } else {
            let notes = QCode::notes(code);
            VoiceType::VNotes(notes)
        };
        Voice::new(vtype, VoiceAttributes {})
    }

    pub fn voices(code: &str) -> Voices {
        let segments: Vec<&str> = code.trim().split('/').collect();
        let mut voices: Vec<Voice> = vec![];
        if segments.len() > 2 {
            panic!("too many voices: {}", voices.len());
        }
        for segment in segments {
            let voice = QCode::voice(segment);
            voices.push(voice);
        }
        Voices::new(voices)
    }
}

#[cfg(test)]
mod tests {
    use super::QCode;

    use crate::notes::*;

    #[test]
    fn test_notes() {
        let notes: Notes = QCode::notes("0 1,2 nv2 -4 p ");
        println!("notes:{:?}", notes);
        for note in &notes {
            println!("- note:{:?}", note);
        }
    }

    #[test]
    fn test_voice() {
        // let voice = QCode::voice("nv4 0 0 0 0");
        let voice = QCode::voice("bp nv2 nv8 x");
        println!("voice:{:?}", voice);
    }

    #[test]
    fn test_voices() {
        let voices = QCode::voices("nv4 0 0 0 0 / bp Nv2dot");
        for voice in voices.items.iter() {
            println!("- voice:{:?}", voice);
        }
    }
}
