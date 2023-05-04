use std::collections::{HashMap, HashSet};

use crate::core::*;
use crate::note::Note;
use crate::notes::*;
use crate::voice::{BarPause, Voice, VoiceType};

#[derive(Debug)]
pub struct Complex<'a> {
    pub position: usize,
    pub duration: Duration,
    pub ctype: ComplexType<'a>,
    // pub attr: ComplexAttributes,
}

#[derive(Debug)]
pub enum ComplexType<'a> {
    OneBarpause(&'a BarPause),
    OneNote(&'a Note, DirUAD),
    TwoNotes(&'a Note, &'a Note),
    BarpauseNote(Note),
    NoteBarpause(Note),
}

impl<'a> Complex<'a> {
    pub fn from_voices(voices: &'a Vec<Voice>) -> Vec<Complex<'a>> {
        let mut complexes: Vec<Complex> = vec![];

        match voices.len() {
            1 => {
                println!("one voice");

                match voices[0].vtype {
                    VoiceType::VBarpause(ref bp) => {
                        println!("barpause");
                        complexes.push(Complex {
                            position: 0,
                            duration: bp.0,
                            ctype: ComplexType::OneBarpause(bp),
                        });
                    }
                    VoiceType::VNotes(ref notes) => {
                        println!("notes");
                        let mut position = 0;
                        for note in notes {
                            println!("- note:{:?}", note);
                            let duration = note.duration.into();
                            complexes.push(Complex {
                                position,
                                duration,
                                ctype: ComplexType::OneNote(note, DirUAD::Auto),
                            });
                            position += duration;
                        }
                    }
                }
            }
            2 => {
                println!("two voices");
                match [&voices[0].vtype, &voices[1].vtype] {
                    [&VoiceType::VBarpause(_), &VoiceType::VNotes(ref notes)] => {
                        println!("barpause/notes");
                    }
                    [&VoiceType::VNotes(_), &VoiceType::VBarpause(_)] => {
                        println!("notes/barpause");
                    }
                    [&VoiceType::VNotes(ref notes1), &VoiceType::VNotes(ref notes2)] => {
                        println!("notes/notes");
                        let max_duration = notes1.duration.max(notes2.duration);
                        let min_duration = notes1.duration.min(notes2.duration);

                        let mut map1: HashMap<usize, &Note> = HashMap::new();
                        for np in notes1.get_note_positions() {
                            map1.insert(np.1, np.0);
                        }
                        let mut map2: HashMap<usize, &Note> = HashMap::new();
                        for np in notes2.get_note_positions() {
                            map2.insert(np.1, np.0);
                        }

                        let mut positionsHash: HashSet<usize> = HashSet::new();
                        map1.keys().for_each(|f| {
                            positionsHash.insert(*f);
                        });
                        map2.keys().for_each(|f| {
                            positionsHash.insert(*f);
                        });

                        let mut positions: Vec<usize> = positionsHash.into_iter().collect();
                        positions.sort();

                        let mut durations: Vec<usize> =
                            positions.windows(2).map(|f| f[1] - f[0]).collect();
                        durations.push(max_duration - positions[positions.len() - 1]);

                        //---------------------------------------------------------------------------------

                        for (idx, position) in positions.iter().enumerate() {
                            let duration = durations[idx];

                            match [map1.get(&position), map2.get(&position)] {
                                [Some(note1), Some(note2)] => {
                                    complexes.push(Complex {
                                        position: *position,
                                        duration,
                                        ctype: ComplexType::TwoNotes(note1, note2),
                                    });
                                }
                                [Some(note1), None] => {
                                    let dir = if *position >= min_duration {
                                        DirUAD::Auto
                                    } else {
                                        DirUAD::Up
                                    };
                                    complexes.push(Complex {
                                        position: *position,
                                        duration,
                                        ctype: ComplexType::OneNote(note1, dir),
                                    });
                                }
                                [None, Some(note2)] => {
                                    let dir = if *position >= min_duration {
                                        DirUAD::Auto
                                    } else {
                                        DirUAD::Down
                                    };
                                    complexes.push(Complex {
                                        position: *position,
                                        duration,
                                        ctype: ComplexType::OneNote(note2, dir),
                                    });
                                }
                                [None, None] => {
                                    panic!("Complex match error - None/None");
                                }
                            }
                        }
                    }
                    [&VoiceType::VBarpause(_), &VoiceType::VBarpause(_)] => {
                        println!("barpause/barpause");
                    }
                }
            }
            _ => {
                println!("too many voices");
            }
        }

        complexes
        // vec![]
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        complex::{Complex, ComplexType},
        quick::QCode,
    };

    #[test]
    fn complex() {
        let voices = QCode::voices("Nv2 0 / Nv8 0 0 0 0 0");
        let complexes = Complex::from_voices(&voices);
        for complex in complexes {
            let s = match complex.ctype {
                ComplexType::OneBarpause(bp) => format!("OneBarpause:{}", bp.0),
                ComplexType::OneNote(note, dir) => format!("OneNote:{:?} ", dir),
                ComplexType::TwoNotes(note1, note2) => format!("TwoNotes:"),
                ComplexType::BarpauseNote(note) => format!("barpause-note:{:?}", note),
                ComplexType::NoteBarpause(note) => format!("note-barpause:{:?}", note),
            };

            println!(
                "complex:{:?} {:?} {:?}",
                complex.position, complex.duration, s
            );
        }
    }
}
