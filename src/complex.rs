use std::collections::{HashMap, HashSet};

use crate::core::*;
use crate::note::Note;
use crate::prelude::*;
use crate::voice::{BarPause, Voice, VoiceType};

#[derive(Debug)]
pub struct Complex<'a> {
    pub position: usize,
    pub duration: Duration,
    pub ctype: ComplexType<'a>,
}

#[derive(Debug)]
pub enum ComplexType<'a> {
    OneBarpause(&'a BarPause),
    TwoBarpauses(&'a BarPause, &'a BarPause),
    //
    OneNote(&'a Note, DirUAD),
    TwoNotes(&'a Note, &'a Note),
    BarpauseNote(&'a BarPause, &'a Note),
    NoteBarpause(&'a Note, &'a BarPause),
}

impl<'a> ComplexType<'a> {
    fn debug_str(&self) -> String {
        match self {
            ComplexType::OneBarpause(bp) => format!("OneBarpause({:?})", bp),
            ComplexType::OneNote(note, dir) => format!("OneNote({:?})", dir),
            ComplexType::TwoNotes(note1, note2) => format!("TwoNotes()"),
            ComplexType::BarpauseNote(bp, note) => format!("BarpauseNote({:?} note)", bp),
            ComplexType::NoteBarpause(note, bp) => format!("NoteBarpause(note/{:?})", bp),
            ComplexType::TwoBarpauses(bp1, bp2) => format!("TwoBarpauses({:?}/{:?})", bp1, bp2),
        }
    }
}

impl<'a> Complex<'a> {
    pub fn from_voices(voices: &'a Vec<Voice>) -> Result<Vec<Complex<'a>>> {
        let mut complexes: Vec<Complex> = vec![];

        match voices.len() {
            0 => return Err(ComplexError("Complex: no voices".to_string()).into()),
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
                            let duration = note.duration;
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
                    [VoiceType::VBarpause(ref bp), VoiceType::VNotes(ref notes)] => {
                        println!("barpause/notes");
                        let mut position = 0;
                        for (idx, note) in notes.iter().enumerate() {
                            let duration = note.duration;
                            complexes.push(Complex {
                                position,
                                duration,
                                ctype: if idx == 0 {
                                    ComplexType::BarpauseNote(bp, note)
                                } else {
                                    ComplexType::OneNote(note, DirUAD::Down)
                                },
                            });
                            position += duration;
                        }
                    }
                    [VoiceType::VNotes(ref notes), VoiceType::VBarpause(ref bp)] => {
                        println!("notes/barpause");
                        let mut position = 0;
                        for (idx, note) in notes.iter().enumerate() {
                            let duration = note.duration;
                            complexes.push(Complex {
                                position,
                                duration,
                                ctype: if idx == 0 {
                                    ComplexType::NoteBarpause(note, bp)
                                } else {
                                    ComplexType::OneNote(note, DirUAD::Up)
                                },
                            });
                            position += duration;
                        }
                    }
                    [VoiceType::VNotes(ref notes1), VoiceType::VNotes(ref notes2)] => {
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

                        let mut positions_hash: HashSet<usize> = HashSet::new();
                        map1.keys().for_each(|f| {
                            positions_hash.insert(*f);
                        });
                        map2.keys().for_each(|f| {
                            positions_hash.insert(*f);
                        });

                        let mut positions: Vec<usize> = positions_hash.into_iter().collect();
                        positions.sort();

                        let mut durations: Vec<usize> =
                            positions.windows(2).map(|f| f[1] - f[0]).collect();
                        durations.push(max_duration - positions[positions.len() - 1]);

                        //---------------------------------------------------------------------------------

                        for (idx, position) in positions.iter().enumerate() {
                            let duration = durations[idx];

                            match [map1.get(position), map2.get(position)] {
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
                                    return Err(ComplexError(
                                        "Complex match error - None/None".to_string(),
                                    )
                                    .into());
                                }
                            }
                        }
                    }
                    [VoiceType::VBarpause(ref bp1), VoiceType::VBarpause(ref bp2)] => {
                        println!("barpause/barpause");
                        complexes.push(Complex {
                            position: 0,
                            duration: bp1.0.max(bp2.0),
                            ctype: ComplexType::TwoBarpauses(bp1, bp2),
                        });
                    }
                }
            }
            _ => return Err(ComplexError("Complex: too many voices".to_string()).into()),
        }

        Ok(complexes)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        complex::{Complex, ComplexType},
        quick::QCode,
    };

    #[test]
    fn test1() {
        let voices = QCode::voices("Nv4 0 / Nv8 0 0").unwrap();
        let complexes = Complex::from_voices(&voices).unwrap();
        dbg!(complexes);
    }

    #[test]
    fn complex() {
        let voices = QCode::voices("Nv4 0 0 / Nv8 0 0 0 0 0").unwrap();
        let complexes = Complex::from_voices(&voices).unwrap();
        for complex in complexes {
            println!(
                "complex:{:?} {:?} {:?}",
                complex.position,
                complex.duration,
                complex.ctype.debug_str()
            );
        }
    }
    #[test]
    fn complex2() {
        let voices = QCode::voices(" Nv4 0 0 0 / bp Nv1").unwrap();
        let complexes = Complex::from_voices(&voices).unwrap();
        for complex in complexes {
            println!(
                "complex:{:?} {:?} {:?}",
                complex.position,
                complex.duration,
                complex.ctype.debug_str()
            );
        }
    }
    #[test]
    fn complex3() {
        let voices = QCode::voices(" bp nv2 / bp nv4").unwrap();
        let complexes = Complex::from_voices(&voices).unwrap();
        for complex in complexes {
            println!(
                "complex:{:?} {:?} {:?}",
                complex.position,
                complex.duration,
                complex.ctype.debug_str()
            );
        }
    }
}
