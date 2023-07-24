use std::collections::{HashMap, HashSet};

use crate::prelude::{fonts::ebgaramond::GLYPH_HEIGHT, *};

pub type Parts = Vec<Rc<RefCell<Part>>>;

#[derive(Debug, PartialEq)]
pub struct Part {
    pub ptype: PartType,
    pub duration: Duration,
    pub complexes: Option<Vec<Rc<RefCell<Complex>>>>,
}

impl Part {
    pub fn new(ptype: PartType) -> Self {
        let duration: Duration = ptype.get_duration();

        Self { ptype, duration, complexes: None }
    }

    pub fn from_voices(voices: Voices) -> Result<Part> {
        let ptype = PartType::Music(PartMusicType::Voices(voices));
        let duration: Duration = ptype.get_duration();
        let mut part = Self { ptype, duration, complexes: None };
        Ok(part)
    }

    pub fn from_lyrics(voices: Voices) -> Result<Part> {
        let ptype = PartType::Nonmusic(PartNonmusicType::Lyrics(voices));
        let duration: Duration = ptype.get_duration();
        let mut part = Self { ptype, duration, complexes: None };
        Ok(part)
    }

    pub fn setup_complexes(&mut self) -> Result<()> {
        match &self.ptype {
            PartType::Music(mtype) => match mtype {
                PartMusicType::Voices(voices) => {
                    self.set_voice_notes_references();
                    self.create_beamgroups(BeamingPattern::NValues(vec![NV4]));
                    self.create_complexes();
                    self.set_complex_durations();
                    self.set_beamgroups_directions(DirUAD::Auto);

                    self.calculate_beamgoups_properties();

                    self.set_note_directions();
                    self.create_complex_rects()?;
                }
                PartMusicType::RepeatBar(_) => todo!(),
            },
            PartType::Nonmusic(ntype) => match ntype {
                PartNonmusicType::Lyrics(voices) => {
                    self.set_voice_notes_references();
                    self.create_complexes();
                    self.set_complex_durations();
                    self.create_complex_rects()?;
                }
                PartNonmusicType::Other => todo!(),
            },
        }
        Ok(())
    }

    pub fn create_beamgroups(&self, pattern: BeamingPattern) {
        match &self.ptype {
            PartType::Music(mtype) => match mtype {
                PartMusicType::Voices(voices) => match voices {
                    Voices::One(v) => {
                        let mut voice = v.borrow_mut();
                        voice.create_beamgroups(&pattern);
                    }
                    Voices::Two(v1, v2) => {
                        let mut voice1 = v1.borrow_mut();
                        let mut voice2 = v2.borrow_mut();
                        voice1.create_beamgroups(&pattern);
                        voice2.create_beamgroups(&pattern);
                    }
                },
                PartMusicType::RepeatBar(_) => todo!(),
            },
            PartType::Nonmusic(_) => {}
        }
    }

    fn calculate_beamgoups_properties(&self) {
        fn do_beamgroup(beamgroups: &Option<Vec<Rc<RefCell<Beamgroup>>>>) {
            if let Some(beamgroups) = beamgroups {
                for beamgroup in beamgroups.iter() {
                    let mut beamgroup = beamgroup.borrow_mut();
                    // beamgroup.calculate_properties();
                    dbg!(beamgroup.notes.len());
                    match beamgroup.notes.len() {
                        0 => panic!("Beamgroup has no notes"),
                        1 => {
                            let note = beamgroup.notes[0].clone();
                            let mut note = note.borrow_mut();
                        }
                        2 => {
                            println!("Two notes");
                            let first = beamgroup.notes[0].clone();
                            let mut first = first.borrow_mut();
                            let last = beamgroup.notes[1].clone();
                            let mut last = last.borrow_mut();
                            let first_top_bottom = (first.top_level(), first.bottom_level());
                            let last_top_bottom = (last.top_level(), last.bottom_level());
                        }
                        _ => {
                            println!("Three notes or more");
                            let last_idx = beamgroup.notes.len() - 1;
                            let first = beamgroup.notes[0].clone();
                            let mut first = first.borrow_mut();
                            let last = beamgroup.notes[last_idx].clone();
                            let mut last = last.borrow_mut();
                            let first_top_bottom = (first.top_level(), first.bottom_level());
                            let last_top_bottom = (last.top_level(), last.bottom_level());
                            let betweens = beamgroup.notes[1..last_idx].to_vec();
                            let betweens_top_bottom: Vec<(i8, i8)> = betweens
                                .iter()
                                .map(|note| {
                                    let note = note.borrow();
                                    (note.top_level(), note.bottom_level())
                                })
                                .collect();

                            let middle_top = betweens_top_bottom.iter().map(|f| f.0).min().unwrap();
                            let middle_bottom = betweens_top_bottom.iter().map(|f| f.1).max().unwrap();
                            let middle_top_bottom = (middle_top, middle_bottom);
                            println!("======================================================");
                            let direction = beamgroup.direction.unwrap();
                            let tilt = match direction {
                                DirUD::Up => {
                                    if first_top_bottom.0 < last_top_bottom.0 {
                                        // println!("- First is LESS than Last - pointing DOWN?");
                                        if middle_top_bottom.0 <= first_top_bottom.0 {
                                            // println!("- - Middle is same or less than first > FLAT");
                                            (middle_top_bottom.0, middle_top_bottom.0)
                                        } else {
                                            // println!("- - Middle is more than first - DOWNWARDS");
                                            (first_top_bottom.0, middle_top_bottom.0.min(last_top_bottom.0))
                                        }
                                    } else if first_top_bottom.0 == last_top_bottom.0 {
                                        // println!("- First is SAME than Last");
                                        // println!("- FLAT SAME");
                                        let level = first_top_bottom.0.min(middle_top_bottom.0);
                                        (level, level)
                                    } else if first_top_bottom.0 > last_top_bottom.0 {
                                        // println!("- First is MORE than Last - pointing UP?");
                                        if middle_top_bottom.0 <= last_top_bottom.0 {
                                            // println!("- - Middle is same or less than last > FLAT");
                                            (middle_top_bottom.0, middle_top_bottom.0)
                                        } else {
                                            // println!("- - Middle is more than last - UPWARDS"); // 3 2 2 1
                                            (first_top_bottom.0.min(middle_top_bottom.0), last_top_bottom.0)
                                        }
                                    } else {
                                        panic!("SHOULD NOT HAPPEN");
                                    }
                                }

                                DirUD::Down => {
                                    if first_top_bottom.1 < last_top_bottom.1 {
                                        // println!("- First is LESS than Last - pointing DOWN?");
                                        if middle_top_bottom.1 >= last_top_bottom.1 {
                                            // println!("- - Middle is same or more than last > FLAT");
                                            (middle_top_bottom.1, middle_top_bottom.1)
                                        } else {
                                            // println!("- - Middle is less than last - DOWNWARDS");
                                            (first_top_bottom.1.max(middle_top_bottom.0), last_top_bottom.1)
                                        }
                                    } else if first_top_bottom.1 == last_top_bottom.1 {
                                        // println!("- First is SAME than Last");
                                        // println!("- FLAT SAME");
                                        let level = first_top_bottom.1.max(middle_top_bottom.1);
                                        (level, level)
                                    } else if first_top_bottom.1 > last_top_bottom.1 {
                                        // println!("- First is MORE than Last - pointing UP?");
                                        if middle_top_bottom.1 >= first_top_bottom.1 {
                                            // println!("- - Middle is same or more than last > FLAT");
                                            (middle_top_bottom.1, middle_top_bottom.1)
                                        } else {
                                            // println!("- - Middle is more than last - UPWARDS");
                                            (first_top_bottom.1, middle_top_bottom.1.max(last_top_bottom.1))
                                        }
                                    } else {
                                        panic!("SHOULD NOT HAPPEN");
                                    }
                                }
                            };

                            let mut tilt_left = tilt.0 as f32;
                            let mut tilt_right = tilt.1 as f32;
                            let angle = tilt_right - tilt_left;
                            dbg!(tilt_left, tilt_right, angle);

                            // dbg!(angle);
                            // if angle <
                            // match direction {
                            //     DirUD::Up => {}
                            //     DirUD::Down => {}
                            // }
                        }
                    }
                }
            }
        }

        match &self.ptype {
            PartType::Music(mtype) => match mtype {
                PartMusicType::Voices(voices) => match voices {
                    Voices::One(v) => {
                        let mut voice = v.borrow_mut();
                        // voice.create_beamgroups(&pattern);
                        do_beamgroup(&voice.beamgroups);
                    }
                    Voices::Two(upper, lower) => {
                        println!("Upper");
                        let mut upper = upper.borrow_mut();
                        do_beamgroup(&upper.beamgroups);
                        println!("Lower");
                        let mut lower = lower.borrow_mut();
                        do_beamgroup(&lower.beamgroups);
                    }
                },
                PartMusicType::RepeatBar(_) => todo!(),
            },
            PartType::Nonmusic(_) => {}
        }
        //
    }

    pub fn create_complexes(&mut self) {
        let mut complexes: Vec<Complex> = Vec::new();

        fn do_voices(voices: &Voices) -> Vec<Complex> {
            let mut complexes: Vec<Complex> = Vec::new();
            match voices {
                Voices::One(v) => match v.borrow().vtype {
                    VoiceType::Notes(ref notes) => {
                        //println!("One voice, notes");
                        for note in notes.items.iter() {
                            let complex = Complex::new(ComplexType::Single(note.clone()), note.borrow().position);
                            complexes.push(complex);
                        }
                    }
                    VoiceType::Barpause(_) => {
                        //println!("One voice, barpause");
                    }
                },
                Voices::Two(v1, v2) => {
                    match [&v1.borrow().vtype, &v2.borrow().vtype] {
                        [VoiceType::Barpause(_), VoiceType::Barpause(_)] => {
                            //
                            //println!("Two voices, barpause, barpause");
                        }

                        [VoiceType::Barpause(_), VoiceType::Notes(notes)] => {
                            //println!("Two voices, barpause, notes");
                            for note in notes.items.iter() {
                                let complex = Complex::new(ComplexType::Lower(note.clone(), false), note.borrow().position);
                                complexes.push(complex);
                            }
                            //
                        }

                        [VoiceType::Notes(notes), VoiceType::Barpause(_)] => {
                            //println!("Two voices, notes, barpause");
                            for note in notes.items.iter() {
                                let complex = Complex::new(ComplexType::Upper(note.clone(), false), note.borrow().position);
                                complexes.push(complex);
                            }
                            //
                        }

                        [VoiceType::Notes(notes_upper), VoiceType::Notes(notes_lower)] => {
                            //println!("Two voices, notes, notes");

                            let max_duration = notes_upper.duration.max(notes_lower.duration);
                            let min_duration = notes_upper.duration.min(notes_lower.duration);

                            let mut map_upper: HashMap<usize, Rc<RefCell<Note>>> = HashMap::new();
                            for np in notes_upper.items.iter() {
                                map_upper.insert(np.borrow().position, np.clone());
                            }
                            let mut map_lower: HashMap<usize, Rc<RefCell<Note>>> = HashMap::new();
                            for np in notes_lower.items.iter() {
                                map_lower.insert(np.borrow().position, np.clone());
                            }
                            let mut positions_hash: HashSet<usize> = HashSet::new();
                            map_upper.keys().for_each(|f| {
                                positions_hash.insert(*f);
                            });
                            map_lower.keys().for_each(|f| {
                                positions_hash.insert(*f);
                            });
                            let mut positions: Vec<usize> = positions_hash.into_iter().collect();
                            positions.sort();
                            let mut durations: Vec<usize> = positions.windows(2).map(|f| f[1] - f[0]).collect();
                            durations.push(max_duration - positions[positions.len() - 1]);

                            for (idx, position) in positions.iter().enumerate() {
                                let duration = durations[idx];

                                match [map_upper.get(position), map_lower.get(position)] {
                                    [Some(note1), Some(note2)] => {
                                        let complex = Complex::new(ComplexType::Two(note1.clone(), note2.clone(), crate::calc::complex_calculate_x_adjustment(note1, note2)), *position);
                                        complexes.push(complex);
                                    }
                                    [Some(note), None] => {
                                        let complex = Complex::new(ComplexType::Upper(note.clone(), position >= &min_duration), note.borrow().position);
                                        complexes.push(complex);
                                    }
                                    [None, Some(note)] => {
                                        let complex = Complex::new(ComplexType::Lower(note.clone(), position >= &min_duration), note.borrow().position);
                                        complexes.push(complex);
                                    }

                                    [None, None] => {}
                                }
                            }
                        }
                    }
                }
            }
            complexes
        }

        match &self.ptype {
            PartType::Music(mtype) => match mtype {
                PartMusicType::Voices(voices) => {
                    complexes = do_voices(voices);
                }
                PartMusicType::RepeatBar(_) => todo!(),
            },

            PartType::Nonmusic(nmtype) => match nmtype {
                PartNonmusicType::Lyrics(voices) => {
                    complexes = do_voices(voices);
                }
                PartNonmusicType::Other => todo!(),
            },
        }

        if !complexes.is_empty() {
            self.complexes = Some(complexes.into_iter().map(|item| Rc::new(RefCell::new(item))).collect::<Vec<_>>());
        }
    }

    pub fn set_beamgroups_directions(&self, force_overflow_dir: DirUAD) -> Option<()> {
        if self.complexes.is_none() {
            return None;
        }
        let complexes = self.complexes.as_ref().unwrap();
        for (idx, complex) in complexes.into_iter().enumerate() {
            match &complex.borrow().ctype {
                ComplexType::Single(note) => {
                    let note = note.borrow();
                    let mut beamgroup = note.beamgroup.as_ref().unwrap().borrow_mut();
                    if beamgroup.direction.is_none() {
                        let dir: DirUD = match force_overflow_dir {
                            DirUAD::Up => DirUD::Up,
                            DirUAD::Down => DirUD::Down,
                            DirUAD::Auto => beamgroup.calc_direction(),
                        };
                        // println!("{idx}- Single: Set beamgroup direction to {dir:?}");
                        beamgroup.direction = Some(dir);
                    } else {
                        println!("{idx}- Two upper: Beamgroup direction is already set");
                    }
                }
                ComplexType::Two(upper, lower, _) => {
                    let upper = upper.borrow();
                    let mut upper_beamgroup = upper.beamgroup.as_ref().unwrap().borrow_mut();
                    if upper_beamgroup.direction.is_none() {
                        // println!("{idx}- Two upper: Set beamgroup direction to Up");
                        upper_beamgroup.direction = Some(DirUD::Up);
                    } else {
                        println!("{idx}- Two upper: Beamgroup direction is already set");
                    }

                    let lower = lower.borrow();
                    let mut lower_beamgroup = lower.beamgroup.as_ref().unwrap().borrow_mut();
                    if lower_beamgroup.direction.is_none() {
                        // println!("{idx}- Two lower: Set beamgroup direction to Down");
                        lower_beamgroup.direction = Some(DirUD::Down);
                    } else {
                        println!("{idx}- Two lower: Beamgroup direction is already set");
                    }
                }
                ComplexType::Upper(upper, overflow) => {
                    let upper = upper.borrow();
                    let mut upper_beamgroup = upper.beamgroup.as_ref().unwrap().borrow_mut();
                    if upper_beamgroup.direction.is_none() {
                        let dir: DirUD = if !overflow {
                            DirUD::Up
                        } else {
                            match force_overflow_dir {
                                DirUAD::Up => DirUD::Up,
                                DirUAD::Down => DirUD::Down,
                                DirUAD::Auto => upper_beamgroup.calc_direction(),
                            }
                        };
                        upper_beamgroup.direction = Some(dir);
                    } else {
                        // println!("{idx}- Upper: Beamgroup direction is already set");
                    }
                }
                ComplexType::Lower(lower, overflow) => {
                    let lower = lower.borrow();
                    let mut lower_beamgroup = lower.beamgroup.as_ref().unwrap().borrow_mut();
                    if lower_beamgroup.direction.is_none() {
                        let dir: DirUD = if !overflow {
                            DirUD::Down
                        } else {
                            match force_overflow_dir {
                                DirUAD::Up => DirUD::Up,
                                DirUAD::Down => DirUD::Down,
                                DirUAD::Auto => lower_beamgroup.calc_direction(),
                            }
                        };

                        lower_beamgroup.direction = Some(dir);
                    } else {
                        // println!("{idx}- Lower: Beamgroup direction is already set");
                    }
                }
            }
        }

        None
    }

    pub fn set_voice_notes_references(&self) -> Option<()> {
        fn do_voices(voices: &Voices) {
            match voices {
                Voices::One(v) => match v.borrow().vtype {
                    VoiceType::Barpause(_) => {}
                    VoiceType::Notes(ref notes) => {
                        for note in notes.items.iter() {
                            note.borrow_mut().voice = Some(v.clone());
                        }
                    }
                },
                Voices::Two(v1, v2) => {
                    match v1.borrow().vtype {
                        VoiceType::Barpause(_) => {}
                        VoiceType::Notes(ref notes) => {
                            for note in notes.items.iter() {
                                note.borrow_mut().voice = Some(v1.clone());
                            }
                        }
                    }
                    match v2.borrow().vtype {
                        VoiceType::Barpause(_) => {}
                        VoiceType::Notes(ref notes) => {
                            for note in notes.items.iter() {
                                note.borrow_mut().voice = Some(v1.clone());
                            }
                        }
                    }
                }
            }
        }

        match &self.ptype {
            PartType::Music(mtype) => match mtype {
                PartMusicType::Voices(voices) => match voices {
                    Voices::One(v) => match v.borrow().vtype {
                        VoiceType::Barpause(_) => {}
                        VoiceType::Notes(ref notes) => {
                            for note in notes.items.iter() {
                                note.borrow_mut().voice = Some(v.clone());
                            }
                        }
                    },
                    Voices::Two(v1, v2) => {
                        match v1.borrow().vtype {
                            VoiceType::Barpause(_) => {}
                            VoiceType::Notes(ref notes) => {
                                for note in notes.items.iter() {
                                    note.borrow_mut().voice = Some(v1.clone());
                                }
                            }
                        }
                        match v2.borrow().vtype {
                            VoiceType::Barpause(_) => {}
                            VoiceType::Notes(ref notes) => {
                                for note in notes.items.iter() {
                                    note.borrow_mut().voice = Some(v1.clone());
                                }
                            }
                        }
                    }
                },
                PartMusicType::RepeatBar(_) => todo!(),
            },
            PartType::Nonmusic(nmtype) => match nmtype {
                PartNonmusicType::Lyrics(voices) => do_voices(voices),
                PartNonmusicType::Other => todo!(),
            },
        }
        None
    }

    pub fn set_complex_durations(&self) -> Option<()> {
        if let Some(complexes) = &self.complexes {
            for (idx, cpl) in complexes.windows(2).enumerate() {
                let mut left = cpl[0].borrow_mut();
                let mut right = cpl[1].borrow_mut();
                let duration = right.position - left.position;
                left.duration = duration;

                if idx == complexes.len() - 2 {
                    right.duration = self.duration - right.position;
                }
            }
        }

        None
    }

    pub fn set_note_directions(&self) -> Option<()> {
        fn fix(voice: &Voice) {
            match voice.vtype {
                VoiceType::Notes(ref notes) => {
                    for note in notes.items.iter() {
                        let mut note = note.borrow_mut();
                        let dir = {
                            let beamgroup = note.beamgroup.as_ref().unwrap().borrow();
                            beamgroup.direction
                        };
                        note.direction = dir.clone();
                    }
                }

                VoiceType::Barpause(_) => {}
            }
        }

        match &self.ptype {
            PartType::Music(mtype) => match mtype {
                PartMusicType::Voices(voices) => match voices {
                    Voices::One(voice) => {
                        fix(&voice.borrow());
                    }
                    Voices::Two(upper, lower) => {
                        fix(&upper.borrow());
                        fix(&lower.borrow());
                    }
                },
                PartMusicType::RepeatBar(_) => todo!(),
            },
            PartType::Nonmusic(_) => todo!(),
        }

        None
    }

    pub fn create_complex_rects(&self) -> Result<()> {
        if self.complexes.is_none() {
            return Ok(());
        }

        let complexes = self.complexes.as_ref().unwrap();

        for (idx, complex) in complexes.into_iter().enumerate() {
            let mut rects: Vec<NRectExt> = Vec::new();
            let mut complex = complex.borrow_mut();

            match complex.ctype {
                ComplexType::Single(ref note) => {
                    let placements = note_get_heads_placements(&note.borrow())?;
                    rects = create_note_rectangles(rects, &note.borrow(), &placements, 0.0, 0.0)?;
                    let mut levels_accidentals = note.borrow().levels_accidentals();
                    levels_accidentals.sort_by(|a, b| a.0.cmp(&b.0));
                    rects = create_accidentals_rectangles(rects, levels_accidentals)?;

                    //
                }
                ComplexType::Two(ref upper, ref lower, ref adjust) => {
                    let pause_up = std::cmp::min(lower.borrow().top_level() - 5, -3);

                    let upper_placements = note_get_heads_placements(&upper.borrow())?;
                    let upper_adjust: f32 = match adjust.as_ref() {
                        Some(adjust) => match adjust {
                            ComplexXAdjustment::UpperRight(upper_right) => *upper_right,
                            ComplexXAdjustment::LowerRight(lower_right) => 0.0,
                        },
                        None => 0.0,
                    };

                    rects = create_note_rectangles(rects, &upper.borrow(), &upper_placements, upper_adjust, pause_up as f32 * SPACE_HALF)?;

                    let pause_down = std::cmp::max(upper.borrow().bottom_level() + 5, 3);

                    let lower_placements = note_get_heads_placements(&lower.borrow())?;
                    let lower_adjust: f32 = match adjust.as_ref() {
                        Some(adjust) => match adjust {
                            ComplexXAdjustment::UpperRight(upper_right) => 0.0,
                            ComplexXAdjustment::LowerRight(lower_right) => *lower_right,
                        },
                        None => 0.0,
                    };

                    rects = create_note_rectangles(rects, &lower.borrow(), &lower_placements, lower_adjust, pause_down as f32 * SPACE_HALF)?;

                    //==================================================================
                    let mut levels_accidentals = upper.borrow().levels_accidentals();
                    let lower_levels_accidentals = lower.borrow().levels_accidentals();
                    for la in lower_levels_accidentals {
                        levels_accidentals.push(la);
                    }
                    levels_accidentals.sort_by(|a, b| a.0.cmp(&b.0));
                    rects = create_accidentals_rectangles(rects, levels_accidentals)?;

                    // dbg!(&levels_accidentals);
                    // dbg!(&levels_accidentals);

                    // let mut idx = 0;
                    // while levels_accidentals.len() > 0 {
                    //     let la = if idx % 2 == 0 {
                    //         levels_accidentals.remove(0)
                    //     } else {
                    //         levels_accidentals.pop().unwrap()
                    //     };
                    //     dbg!(&la);
                    //     idx += 1;
                    // }
                }
                ComplexType::Upper(ref note, overflow) => {
                    let placements = note_get_heads_placements(&note.borrow())?;
                    // dbg!(" - Upper", &placements, overflow);
                    rects = create_note_rectangles(rects, &note.borrow(), &placements, 0.0, -SPACE)?;
                    //
                    let mut levels_accidentals = note.borrow().levels_accidentals();
                    levels_accidentals.sort_by(|a, b| a.0.cmp(&b.0));
                    rects = create_accidentals_rectangles(rects, levels_accidentals)?;
                }
                ComplexType::Lower(ref note, overflow) => {
                    let placements = note_get_heads_placements(&note.borrow())?;
                    // dbg!(" - Lower", &placements, overflow);
                    rects = create_note_rectangles(rects, &note.borrow(), &placements, 0.0, SPACE)?;
                    //
                    let mut levels_accidentals = note.borrow().levels_accidentals();
                    levels_accidentals.sort_by(|a, b| a.0.cmp(&b.0));
                    rects = create_accidentals_rectangles(rects, levels_accidentals)?;
                }
            };

            if !rects.is_empty() {
                for nrect in rects {
                    complex.rects.push(Rc::new(RefCell::new(nrect)));
                }
            }
        }

        Ok(())
    }
}

fn create_accidentals_rectangles(mut rects: Vec<NRectExt>, mut levels_accidentals: Vec<(i8, Accidental)>) -> Result<Vec<NRectExt>> {
    let mut idx = 0;
    while levels_accidentals.len() > 0 {
        let level_accidental = if idx % 2 == 0 { levels_accidentals.remove(0) } else { levels_accidentals.pop().unwrap() };

        let (level, accidental) = level_accidental;

        let width = match accidental {
            Accidental::Sharp => SPACE * 1.2,
            Accidental::Flat => SPACE * 1.1,
            Accidental::Natural => SPACE * 1.0,
            Accidental::DblSharp => SPACE * 1.5,
            Accidental::DblFlat => SPACE * 1.5,
        };

        let y = match accidental {
            Accidental::Flat => -(SPACE * 2.0),
            _ => -(SPACE * 1.5),
        };

        let mut rect: NRect = NRect::new(0.0, (level as f32 * SPACE_HALF) + y, width, 3.0 * SPACE);

        let overlap = rect.overlap_multi_nrectexts_x(&rects);
        rect = rect.move_rect(-overlap, 0.0);
        rects.push(NRectExt(rect, NRectType::Accidental(accidental.clone())));

        idx += 1;
    }

    Ok(rects)
}

pub fn create_note_rectangles(mut rects: Vec<NRectExt>, note: &Note, placements: &HeadsPlacement, adjust_right: f32, adjust_y: f32) -> Result<Vec<NRectExt>> {
    match note.ntype {
        NoteType::Heads(_) => {
            rects = create_heads_and_dots_rectangles(rects, note, placements, adjust_right)?;
        }
        NoteType::Pause => {
            rects = create_pause_rectangles(rects, note, adjust_y)?;
        }
        NoteType::Lyric(_) => {
            rects = create_lyric_rectangles(rects, note, adjust_y)?;
        }
    }
    Ok(rects)
}

pub fn create_heads_and_dots_rectangles(mut rects: Vec<NRectExt>, note: &Note, placements: &HeadsPlacement, adjust_right: f32) -> Result<Vec<NRectExt>> {
    let note_head_type = duration_get_headtype(&note.duration);
    let note_shape = duration_get_headshape(&note.duration);
    let duration = note.duration;
    let dots_nr: u8 = duration_get_dots(&duration);
    let dots_width = dots_nr as f32 * DOT_WIDTH;
    let note_width: f32 = duration_get_headwidth(&note.duration);

    // DevStem

    if note.has_stem() {
        let dir = note.beamgroup.as_ref().unwrap().borrow().direction;

        const DEV_STEM_LENGHT: f32 = 6.5;
        let stem_length = (note.bottom_level() as f32 - note.top_level() as f32 + 0.5 + DEV_STEM_LENGHT) * SPACE_HALF;
        if let Some(d) = dir {
            match d {
                DirUD::Up => {
                    let rect: NRect = NRect::new(
                        adjust_right + note_width - STEM_WIDTH,
                        note.top_level() as f32 * SPACE_HALF - SPACE_HALF - (DEV_STEM_LENGHT * SPACE_HALF),
                        STEM_WIDTH,
                        stem_length,
                    );
                    rects.push(NRectExt(rect, NRectType::DevStem));
                    //
                }
                DirUD::Down => {
                    let rect: NRect = NRect::new(adjust_right, note.top_level() as f32 * SPACE_HALF, STEM_WIDTH, stem_length);
                    rects.push(NRectExt(rect, NRectType::DevStem));
                }
            }
        }
    };

    // Heads

    for placement in placements {
        let (level, place, head) = placement;

        let mut current_x: f32 = (place.as_f32() * note_width) + adjust_right;

        let rect: NRect = NRect::new(current_x, *level as f32 * SPACE_HALF - SPACE_HALF, note_width, SPACE);
        rects.push(NRectExt(rect, NRectType::Head(*note_head_type, *note_shape)));
        current_x += note_width;

        // Dots

        if dots_nr > 0 {
            let rect: NRect = NRect::new(current_x, *level as f32 * SPACE_HALF - SPACE_QUARTER, dots_width, SPACE_HALF);
            rects.push(NRectExt(rect, NRectType::Dotted(dots_nr)));
            // current_x += dots_width;
        }

        // Ties

        if head.borrow().tie.is_some() {
            let rect: NRect = NRect::new(current_x, *level as f32 * SPACE_HALF - SPACE_HALF, SPACE, SPACE);
            rects.push(NRectExt(rect, NRectType::Tie(head.borrow().tie.as_ref().unwrap().clone())));
        }
    }

    Ok(rects)
}

pub fn create_pause_rectangles(mut rects: Vec<NRectExt>, note: &Note, adjust_y: f32) -> Result<Vec<NRectExt>> {
    let avoid_y_collision = 0.0;
    match note.duration {
        NV1 | NV1DOT => {
            let rect = NRect::new(0., adjust_y + -SPACE + avoid_y_collision, SPACE, SPACE_HALF);
            rects.push(NRectExt(rect, NRectType::Pause(PauseShape::Whole)));
        }
        NV2 | NV2DOT | NV2TRI => {
            let rect = NRect::new(0., adjust_y + -SPACE_HALF + avoid_y_collision, SPACE, SPACE_HALF);
            rects.push(NRectExt(rect, NRectType::Pause(PauseShape::Half)));
        }
        NV4 | NV4DOT | NV4TRI => {
            let rect = NRect::new(0., adjust_y + -1.4 * SPACE + avoid_y_collision, SPACE, 2.8 * SPACE);
            rects.push(NRectExt(rect, NRectType::Pause(PauseShape::Quarter)));
        }
        NV8 | NV8DOT | NV8TRI => {
            let rect = NRect::new(0., adjust_y + -SPACE + avoid_y_collision, SPACE, 2. * SPACE);
            rects.push(NRectExt(rect, NRectType::Pause(PauseShape::Eighth)));
        }
        NV16 | NV16DOT | NV16TRI => {
            let rect = NRect::new(0., adjust_y + -SPACE + avoid_y_collision, SPACE * 1.3, 3. * SPACE);
            rects.push(NRectExt(rect, NRectType::Pause(PauseShape::Sixteenth)));
        }

        _ => {
            let rect = NRect::new(0., adjust_y + -SPACE_HALF + avoid_y_collision, SPACE, SPACE);
            rects.push(NRectExt(rect, NRectType::WIP("pause undefined".to_string())));
        }
    };

    Ok(rects)
}

fn create_lyric_rectangles(mut rects: Vec<NRectExt>, note: &Note, adjust_y: f32) -> Result<Vec<NRectExt>> {
    let mut char_height = GLYPH_HEIGHT * FONT_SCALE_LYRICS;

    match &note.ntype {
        NoteType::Lyric(syllable) => {
            match &syllable.syllable_type {
                SyllableType::Text(s) => {
                    //
                    let mut total_width = 0.0;
                    let mut char_widths = Vec::new();

                    for char in s.chars() {
                        let char_width = crate::render::fonts::ebgaramond::glyph_widths(char) * FONT_SCALE_LYRICS; // use the width of the current character
                        char_widths.push(char_width);
                        total_width += char_width;
                    }

                    let mut char_x = -(total_width / 2.0) + SPACE_HALF;
                    for (idx, char_width) in char_widths.iter().enumerate() {
                        let rect = NRect::new(char_x, adjust_y + -(char_height / 2.0) - SPACE_HALF, *char_width, char_height + SPACE);
                        rects.push(NRectExt(rect, NRectType::LyricChar(s.chars().nth(idx).unwrap())));
                        char_x += char_width;
                    }

                    // nrects.push(NRectExt(rect, NRectType::LyricChar(char.to_string())));
                }
                SyllableType::TextWithHyphen(_) => {
                    //
                }
                SyllableType::Hyphen => {
                    //
                }
                SyllableType::Extension(_) => {
                    //
                }
            }
            //
        }
        _ => {}
    }

    Ok(rects)
}

#[derive(Debug, PartialEq)]
pub enum PartType {
    Music(PartMusicType),
    Nonmusic(PartNonmusicType),
}

#[derive(Debug, PartialEq)]
pub enum PartMusicType {
    Voices(Voices),
    RepeatBar(usize),
}

#[derive(Debug, PartialEq)]
pub enum PartNonmusicType {
    Lyrics(Voices),
    Other,
}

impl PartType {
    pub fn get_duration(&self) -> Duration {
        let duration = match self {
            PartType::Music(mtype) => match mtype {
                PartMusicType::Voices(voices) => match voices {
                    Voices::One(voice) => voice.borrow().duration,
                    Voices::Two(upper, lower) => std::cmp::max(upper.borrow().duration, lower.borrow().duration),
                },
                PartMusicType::RepeatBar(_) => todo!(),
            },
            PartType::Nonmusic(ntype) => match ntype {
                PartNonmusicType::Lyrics(voices) => match voices {
                    Voices::One(voice) => voice.borrow().duration,
                    Voices::Two(upper, lower) => std::cmp::max(upper.borrow().duration, lower.borrow().duration),
                },
                PartNonmusicType::Other => todo!(),
            },
        };
        duration
    }
}

#[derive(Debug, PartialEq)]
pub enum Voices {
    One(Rc<RefCell<Voice>>),
    Two(Rc<RefCell<Voice>>, Rc<RefCell<Voice>>),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PartTemplate {
    Music,
    // Lyrics,
    Nonmusic,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BarTemplate(pub Vec<PartTemplate>);

#[cfg(test)]
mod tests2 {
    use crate::prelude::*;

    #[test]
    fn example() -> Result<()> {
        let voices = QCode::voices(" nv4 0 % nv4 1").unwrap();
        let mut part = Part::from_voices(voices).unwrap();
        part.setup_complexes().unwrap();
        for complex in part.complexes.unwrap() {
            complex.borrow().print();
        }

        Ok(())
    }
}
