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
                // note value
                a if a.to_lowercase().starts_with("nv") => {
                    let s = &segment[2..];
                    cur_val = duration_from_str(s).ok();
                }

                // lyric note
                a if a.starts_with("$lyr:") => println!("Dont use $lyr: anymore!"),

                // lyric note
                a if a.starts_with("lyr:") => {
                    let mut s = &segment[4..];
                    s = s.trim();
                    let syllable = Syllable::new(SyllableType::Text(s.to_string()));
                    let n = Note::new(NoteType::Lyric(syllable), cur_val.unwrap_or(NV4));
                    notes.push(n);
                }

                // tonplats note
                a if a.starts_with("tpl:") => {
                    let s = segment.trim();
                    let mut s = &segment[4..];

                    let subsegments = s.split(':').collect::<Vec<_>>();

                    let level: i8 = subsegments.get(0).unwrap_or(&"").parse().unwrap_or(0);

                    let figure_char: char = if subsegments.len() > 1 {
                        subsegments[1].chars().next().unwrap_or('0')
                    } else {
                        '0'
                    };
                    dbg!(figure_char);

                    let n = Note::new(
                        NoteType::Tpl(figure_char, TplOctave::Mid, TplAccidental::Neutral, level),
                        cur_val.unwrap_or(NV4),
                    );
                    notes.push(n);
                }

                // function symbol note
                a if a.starts_with("fun:") => {
                    let s = segment.trim();
                    let mut s = &segment[4..];
                    let func_pars = crate::utils::parse_function(s).unwrap();

                    let n = Note::new(
                        NoteType::Function(
                            func_pars.0,
                            func_pars.1,
                            func_pars.2,
                            func_pars.3,
                            func_pars.4,
                        ),
                        cur_val.unwrap_or(NV4),
                    );
                    notes.push(n);
                }

                // chord symbol note
                a if a.starts_with("chd:") => {
                    let s = segment.trim();
                    let mut s = &segment[4..];
                    let chord_pars = crate::utils::parse_chord(s).unwrap();

                    let n = Note::new(
                        NoteType::ChordSymbol(
                            chord_pars.0,
                            chord_pars.1,
                            chord_pars.2,
                            chord_pars.3,
                        ),
                        cur_val.unwrap_or(NV4),
                    );
                    notes.push(n);
                }

                // symbol
                a if a.starts_with("sym:") => {
                    let s = segment.trim();
                    let mut s = &segment[4..];
                    let stype = crate::utils::parse_symbol(s).unwrap();

                    let n = Note::new(NoteType::Symbol(stype.1), cur_val.unwrap_or(NV4));
                    notes.push(n);
                }

                // pause
                "p" => {
                    let n = Note::new(NoteType::Pause, cur_val.unwrap_or(NV4)); // NoteAttributes { color: None });
                    notes.push(n);
                }

                // sparcer
                "s" => {
                    let n = Note::new(NoteType::Spacer(0), cur_val.unwrap_or(NV4));
                    notes.push(n);
                }

                _ => {
                    let (segment, articulation) = crate::utils::parse_articulation(segment);
                    //
                    let segments: Vec<&str> = segment.split(',').collect();
                    let mut heads: Vec<Head> = vec![];
                    for segment in &segments {
                        let level = crate::utils::parse_string_to_int(segment)?;
                        let accidental = crate::utils::parse_accidental(segment);
                        let tie = crate::utils::parse_tie(segment);
                        let tie_to = crate::utils::parse_tie_to(segment);

                        let line = crate::utils::parse_line(segment);

                        let head =
                            Head::new_with_attributes(level as i8, accidental, tie, tie_to, line);
                        heads.push(head);
                    }

                    let mut n =
                        Note::new(NoteType::Heads(Heads::new(heads)), cur_val.unwrap_or(NV4));
                    n.articulation = articulation;

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
            Err(Generic("other part not implemented".to_string()).into())
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
                    return Err(Generic(format!(
                        "bar template mismatch: {:?} != {:?}",
                        bartemplate, first_bar_template
                    ))
                    .into());
                }
            } else {
                if !(bartemplate.0.is_empty()) {
                    first_bar_template = Some(bartemplate);
                }
            }

            bars.push(Rc::new(RefCell::new(bar)));
        }

        if bars.is_empty() || first_bar_template.is_none() {
            return Err(Generic(format!("no bars in code: {}", code)).into());
        }

        //======================================================================
        // include vertical line first
        // bars.insert(0, Rc::new(RefCell::new(Bar::new(BarType::NonContent(NonContentType::VerticalLine)))));
        // add vertical line to end
        // bars.push(Rc::new(RefCell::new(Bar::new(BarType::NonContent(NonContentType::VerticalLine)))));
        //======================================================================

        Ok((first_bar_template.unwrap(), Bars::new(bars)))
    }

    pub fn bar(mut code: &str) -> Result<(BarTemplate, Bar)> {
        code = code.trim();

        if code.starts_with("bld") {
            let bar = Bar::new(BarType::NonContent(NonContentType::Barline(
                BarlineType::Double,
            )));
            return Ok((BarTemplate(vec![]), bar));
        } else if code.starts_with("blt") {
            let bar = Bar::new(BarType::NonContent(NonContentType::Barline(
                BarlineType::FraseTick,
            )));
            return Ok((BarTemplate(vec![]), bar));
        } else if code.starts_with("bl") {
            let bar = Bar::new(BarType::NonContent(NonContentType::Barline(
                BarlineType::Single,
            )));
            return Ok((BarTemplate(vec![]), bar));
        } else if code.starts_with("vl") {
            let bar = Bar::new(BarType::NonContent(NonContentType::VerticalLine));
            return Ok((BarTemplate(vec![]), bar));
        } else if code.starts_with("ci") {
            let code = code.split(' ').skip(1).collect::<Vec<_>>().join(" ");
            let notes = QCode::notes(&code).unwrap();
            let bar = Bar::new(BarType::CountIn(notes));
            return Ok((BarTemplate(vec![]), bar));
        } else if code.starts_with("sp1") {
            let bar = Bar::new(BarType::NonContent(NonContentType::Spacer(5., 150.)));
            return Ok((BarTemplate(vec![]), bar));
        } else if code.starts_with("sp2") {
            let bar = Bar::new(BarType::NonContent(NonContentType::Spacer(10., 150.)));
            return Ok((BarTemplate(vec![]), bar));
        } else if code.starts_with("sp3") {
            let bar = Bar::new(BarType::NonContent(NonContentType::Spacer(30., 150.)));
            return Ok((BarTemplate(vec![]), bar));
        } else if code.starts_with("sp") {
            let segments = code.split(' ').skip(1).collect::<Vec<_>>();
            let mut width = 30.0;
            if segments.len() >= 1 {
                width = segments[0].parse::<f32>().unwrap_or(30.)
            };
            let mut height = 150.;
            if segments.len() >= 2 {
                height = segments[1].parse::<f32>().unwrap_or(100.);
            }
            let bar = Bar::new(BarType::NonContent(NonContentType::Spacer(width, height)));
            return Ok((BarTemplate(vec![]), bar));
        } else if code.starts_with("mul") {
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
                    _ => println!("other clef {}", segment),
                }
                match segment.to_uppercase().as_str() {
                    "-" => parttemplates.push(PartTemplate::Nonmusic),
                    "G" | "F" | "C" => parttemplates.push(PartTemplate::Music),
                    _ => println!("other clef {}", segment),
                }
            }

            let bar = Bar::from_clefs(clefs);
            return Ok((BarTemplate(parttemplates), bar));
        } else if code.starts_with("tim") {
            let code = code.split(' ').skip(1).collect::<Vec<_>>().join(" ");
            let segments = code.split(' ').collect::<Vec<_>>();
            let mut times = vec![];
            let mut parttemplates = vec![];
            for segment in segments {
                match segment {
                    n if n.contains(":") => {
                        let nsegments = n.split(':').collect::<Vec<_>>();
                        let nominator = TimeNominator::from_str(nsegments[0]);
                        let denominator = TimeDenominator::from_str(nsegments[1]);
                        times.push(Some(Some(Time::Standard(nominator, denominator))));
                    }
                    "c" => times.push(Some(Some(Time::Cut))),
                    "C" => times.push(Some(Some(Time::Common))),
                    "-" => times.push(None),
                    _ => println!("other time signature {}", segment),
                }
                match segment.to_uppercase().as_str() {
                    "-" => parttemplates.push(PartTemplate::Nonmusic),
                    n if n.contains(":") => parttemplates.push(PartTemplate::Music),
                    "c" => parttemplates.push(PartTemplate::Music),
                    "C" => parttemplates.push(PartTemplate::Music),
                    _ => println!("other time signature {}", segment),
                }
            }
            let bar = Bar::from_times(times);
            return Ok((BarTemplate(parttemplates), bar));
        } else if code.starts_with("key") {
            let segments = code.split(' ').skip(1).collect::<Vec<_>>();
            let mut keys = vec![];
            let mut parttemplates = vec![];
            for segment in segments {
                dbg!(&segment);
                let mut key_segment = segment.to_lowercase();
                let mut key_clef = Clef::G;

                if key_segment.starts_with('f') {
                    key_segment = key_segment[1..].to_string();
                    println!("F-klav förtecekn!");
                    dbg!(&key_segment);
                    key_clef = Clef::F;
                }

                if key_segment.starts_with('c') {
                    key_segment = key_segment[1..].to_string();
                    println!("C-klav förtecekn!");
                    key_clef = Clef::C;
                }

                // if &key_segment.chars()[0] == 'f' {
                //     key_segment = &key_segment[1..];
                // }

                match key_segment.as_str() {
                    "#" => keys.push(Some(Some(Key::Sharps(1, key_clef)))),
                    "##" => keys.push(Some(Some(Key::Sharps(2, key_clef)))),
                    "###" => keys.push(Some(Some(Key::Sharps(3, key_clef)))),
                    "####" => keys.push(Some(Some(Key::Sharps(4, key_clef)))),
                    "#####" => keys.push(Some(Some(Key::Sharps(5, key_clef)))),
                    "######" => keys.push(Some(Some(Key::Sharps(6, key_clef)))),
                    "b" => keys.push(Some(Some(Key::Flats(1, key_clef)))),
                    "bb" => keys.push(Some(Some(Key::Flats(2, key_clef)))),
                    "bbb" => keys.push(Some(Some(Key::Flats(3, key_clef)))),
                    "bbbb" => keys.push(Some(Some(Key::Flats(4, key_clef)))),
                    "bbbbb" => keys.push(Some(Some(Key::Flats(5, key_clef)))),
                    "bbbbbb" => keys.push(Some(Some(Key::Flats(6, key_clef)))),
                    "n" => keys.push(Some(Some(Key::Open))),
                    "-" => keys.push(None),
                    _ => println!("other keys {}", segment),
                }

                match key_segment.as_str() {
                    "-" => parttemplates.push(PartTemplate::Nonmusic),
                    a if a.starts_with("#") => parttemplates.push(PartTemplate::Music),
                    a if a.starts_with("b") => parttemplates.push(PartTemplate::Music),
                    a if a.starts_with("n") => parttemplates.push(PartTemplate::Music),
                    _ => println!("other time signature {}", segment),
                }
            }
            // let bar = Bar::new(BarType::NonContent(NonContentType::Barline));
            let bar = Bar::from_keys(keys);
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
