use crate::prelude::*;

pub struct QCode;

impl QCode {
    pub fn notes(code: &str) -> Result<Notes> {
        let code = code.replace("  ", " ");
        let segments: Vec<&str> = code.trim().split(' ').collect();
        let mut cur_val: Option<usize> = None;
        let mut notes: Vec<Note> = vec![];
        for segment in segments {
            match segment {
                a if a.to_lowercase().starts_with("nv") => {
                    let s = &segment[2..];
                    cur_val = duration_from_str(s).ok();
                }

                a if a.starts_with("$lyr:") => {
                    let mut s = &segment[5..];
                    s = s.trim();
                    dbg!(s);
                    let syllable = Syllable::new(SyllableType::Text(s.to_string()));
                    let n = Note::new(NoteType::Lyric(syllable), cur_val.unwrap_or(NV4));
                    notes.push(n);
                }

                "p" => {
                    let n = Note::new(NoteType::Pause, cur_val.unwrap_or(NV4)); // NoteAttributes { color: None });
                    notes.push(n);
                }
                _ => {
                    let segments: Vec<&str> = segment.split(',').collect();
                    let mut heads: Vec<Head> = vec![];
                    for segment in &segments {
                        let level = crate::utils::parse_string_to_int(segment)?;
                        let accidental = crate::utils::parse_accidental(segment);
                        let tie = crate::utils::parse_tie(segment);
                        heads.push(Head::new_with_attributes(level as i8, accidental, tie));
                        // , HeadAttributes {}
                    }

                    let n = Note::new(
                        NoteType::Heads(Heads::new(heads)),
                        cur_val.unwrap_or(NV4),
                        // NoteAttributes { color: None },
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
                    let dur = duration_from_str(&segment[2..]);
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

            VoiceType::Barpause(barpause_value)
        } else {
            let notes = QCode::notes(code)?;
            VoiceType::Notes(notes)
        };
        Ok(Voice::new(vtype)) // , VoiceAttributes {}
    }

    pub fn voices(code: &str) -> Result<Voices> {
        if code.contains("/") {
            panic!("code contains /: {}", code);
        }

        let segments: Vec<&str> = code.trim().split('%').collect();
        let nr_of_voices = segments.len();

        match nr_of_voices {
            0 => Err(Generic("no voice in code".to_string()).into()),
            1 => {
                let voice = QCode::voice(segments[0])?;
                Ok(Voices::One(Rc::new(RefCell::new(voice))))
            }
            2 => {
                let voice1 = QCode::voice(segments[0])?;
                let voice2 = QCode::voice(segments[1])?;
                Ok(Voices::Two(
                    Rc::new(RefCell::new(voice1)),
                    Rc::new(RefCell::new(voice2)),
                ))
            }
            3 => {
                let voice1 = QCode::voice(segments[1])?;
                let voice2 = QCode::voice(segments[2])?;
                Ok(Voices::Two(
                    Rc::new(RefCell::new(voice1)),
                    Rc::new(RefCell::new(voice2)),
                ))
            }
            _ => Err(Generic(format!("too many voices in code: {}", nr_of_voices)).into()),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::prelude::*;

    #[test]
    fn test_notes() {
        let notes: Notes = QCode::notes("0 1,2 nv2 -4 p ").unwrap();
        println!("notes:{:?}", notes);
        // for note in &notes {
        //     println!("- note:{:?}", note.duration);
        // }
    }

    // #[test]
    // fn test_accidentals() {
    //     let notes: Notes = QCode::notes("1 b2 #-3 n4").unwrap();
    //     for note in &notes {
    //         println!("- note:{:?}", note);
    //     }
    // }

    #[test]
    fn test_voice() {
        // let voice = QCode::voice("nv4 0 0 0 0");
        let voice = QCode::voice("bp x nv2 y nv8 z").unwrap();
        println!("voice:{:?}", voice);
    }

    // #[test]
    // fn test_voices() {
    //     let voices = QCode::voices("nv4 0 0 0 0 % bp").unwrap();
    //     dbg!(voices);
    // }

    // #[test]
    // fn test_voicetype() {
    //     let voices = QCode::voices("% 1 1 % 3 3").unwrap();
    //     dbg!(voices);
    // }

    #[test]
    fn test_lyric() {
        let notes = QCode::notes("$lyr:Hej ").unwrap();
        dbg!(notes);
    }
}
