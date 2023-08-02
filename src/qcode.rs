use crate::prelude::*;

pub struct QCode;

impl QCode {
    pub fn notes(code: &str) -> Result<Notes> {
        let mut code = code.replace("  ", " ");
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
                    todo!("Remove $ from lyrics!");
                }

                a if a.starts_with("lyr:") => {
                    let mut s = &segment[4..];
                    s = s.trim();
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
                        let tie_to = crate::utils::parse_tie_to(segment);
                        let head = Head::new_with_attributes(level as i8, accidental, tie, tie_to);
                        heads.push(head);
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

    pub fn voice(mut code: &str) -> Result<Voice> {
        code = code.trim();
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

            let barpause_value: Option<Duration> = if barpause_value == 0 { None } else { Some(barpause_value) };

            VoiceType::Barpause(barpause_value)
        } else {
            let notes = QCode::notes(code)?;
            VoiceType::Notes(notes)
        };
        Ok(Voice::new(vtype)) // , VoiceAttributes {}
    }

    pub fn voices(mut code: &str) -> Result<Voices> {
        code = code.trim();

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
                Ok(Voices::Two(Rc::new(RefCell::new(voice1)), Rc::new(RefCell::new(voice2))))
            }
            3 => {
                let voice1 = QCode::voice(segments[1])?;
                let voice2 = QCode::voice(segments[2])?;
                Ok(Voices::Two(Rc::new(RefCell::new(voice1)), Rc::new(RefCell::new(voice2))))
            }
            _ => Err(Generic(format!("too many voices in code: {}", nr_of_voices)).into()),
        }
    }

    pub(crate) fn parts(code: &str) -> Result<(BarTemplate, Parts)> {
        let mut part_templates = vec![];
        let mut parts: Parts = vec![];

        //
        let segments: Vec<&str> = code.trim().split('/').collect();
        for segment in segments {
            if segment.is_empty() {
                continue;
            }
            let part_data = QCode::part(segment)?;
            let template_type = part_data.0;
            part_templates.push(template_type);
            let part = part_data.1;
            parts.push(Rc::new(RefCell::new(part)));
        }

        Ok((BarTemplate(part_templates), parts))
    }

    pub fn part(mut code: &str) -> Result<(PartTemplate, Part)> {
        code = code.trim();
        if code.starts_with("lyr") {
            let code = code.split(' ').collect::<Vec<_>>()[1..].join(" ");
            let voices = QCode::voices(code.as_str())?;
            let part = Part::from_lyrics(voices)?;
            Ok((PartTemplate::Nonmusic, part))
        } else if code.starts_with("oth") {
            todo!("other part");
        } else {
            // Music part
            let voices = QCode::voices(code)?;
            let mut part = Part::from_voices(voices)?;
            // part.setup()?;
            Ok((PartTemplate::Music, part))
        }
    }

    pub fn bars(mut code: &str) -> Result<(BarTemplate, Bars)> {
        code = code.trim();
        let segments: Vec<&str> = code.split('|').collect();
        let mut first_bar_template: Option<BarTemplate> = None;
        let mut bars = vec![];

        for segment in segments {
            if segment.is_empty() {
                continue;
            }

            let bar_data = QCode::bar(segment)?;
            let (bartemplate, bar) = bar_data;

            if let Some(first_bar_template) = &first_bar_template {
                if bartemplate.0.len() > 0 && bartemplate != *first_bar_template {
                    return Err(Generic(format!("bar template mismatch: {:?} != {:?}", bartemplate, first_bar_template)).into());
                }
            } else {
                if !(bartemplate.0.is_empty()) {
                    first_bar_template = Some(bartemplate);
                }
            }

            bars.push(Rc::new(RefCell::new(bar)));
        }

        if bars.is_empty() {
            return Err(Generic(format!("no bars in code: {}", code)).into());
        }

        //======================================================================
        // include vertical line first
        bars.insert(0, Rc::new(RefCell::new(Bar::new(BarType::NonContent(NonContentType::VerticalLine)))));
        // add vertical line to end
        bars.push(Rc::new(RefCell::new(Bar::new(BarType::NonContent(NonContentType::VerticalLine)))));
        //======================================================================

        Ok((first_bar_template.unwrap(), Bars::new(bars)))
    }

    pub fn bar(mut code: &str) -> Result<(BarTemplate, Bar)> {
        // pub fn bar(mut code: &str) -> Result<()> {
        code = code.trim();

        if code.starts_with("bl") {
            let code = code.split(' ').skip(1).collect::<Vec<_>>().join(" ");
            let bar = Bar::new(BarType::NonContent(NonContentType::Barline));
            return Ok((BarTemplate(vec![]), bar));
        } else if code.starts_with("mul") {
            // todo!("multi rest");
            let bar = Bar::new(BarType::MultiRest(0));
            return Ok((BarTemplate(vec![]), bar));
        } else if code.starts_with("cle") {
            let code = code.split(' ').skip(1).collect::<Vec<_>>().join(" ");
            let segments = code.split(' ').collect::<Vec<_>>();
            let mut clefs = vec![];
            let mut parttemplates = vec![];
            for segment in segments {
                match segment.to_uppercase().as_str() {
                    "G" => clefs.push(Some(Some(Clef::G))),
                    "F" => clefs.push(Some(Some(Clef::F))),
                    "C" => clefs.push(Some(Some(Clef::C))),
                    "-" => clefs.push(None),
                    _ => todo!("other clefs {}", segment),
                }
                match segment.to_uppercase().as_str() {
                    "-" => parttemplates.push(PartTemplate::Nonmusic),
                    "G" | "F" | "C" => parttemplates.push(PartTemplate::Music),
                    _ => todo!("other clefs {}", segment),
                }
            }
            let bar = Bar::from_clefs(clefs);
            return Ok((BarTemplate(parttemplates), bar));
        } else {
            let parts_data = QCode::parts(code)?;
            let (template, parts) = parts_data;
            let bar = Bar::from_parts(parts);
            return Ok((template, bar));
        }
        // Err(Generic(format!("unknown bar code: {}", code)).into())
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
