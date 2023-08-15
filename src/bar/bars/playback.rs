use std::{cell::Ref, collections::BTreeMap};

use crate::prelude::*;
use core::any::type_name;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlayNote {
    midinote: u8,
    volocity: u8,
    duration: Duration,
    position: Position,
    partidx: u8,
    voiceidx: u8,
    noteidx: u8,
}

pub type PlayNotes = Vec<PlayNote>;

#[derive(Debug)]
pub struct PlayNotesData {
    playnotes: PlayNotes,
    duration: Duration,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlayPosition {
    position: Option<Position>,
    x: f32,
}

pub type PlayPositions = Vec<PlayPosition>;

#[derive(Debug)]
pub struct PlayPositionsData {
    positions: PlayPositions,
    top_y: f32,
    bottom_y: f32,
}

impl Bars {
    pub fn calc_playback(&self) -> PlayNotesData {
        let parts_voices = self.count_music_parts_voices();

        let mut part_clef: BTreeMap<usize, Clef> = BTreeMap::new();
        let mut part_key: BTreeMap<usize, Key> = BTreeMap::new();
        let mut part_time: BTreeMap<usize, Time> = BTreeMap::new();

        for (partidx, part_voice) in parts_voices.iter().enumerate() {
            part_clef.insert(partidx, Clef::G);
            part_key.insert(partidx, Key::Open);
            part_time.insert(partidx, Time::Common);
        }

        let mut playnotes: PlayNotes = Vec::new();

        for (baridx, bar) in self.items.iter().enumerate() {
            let bar: Ref<Bar> = bar.borrow();
            match &bar.btype {
                BarType::Standard(parts) => {
                    for (partidx, part) in parts.iter().enumerate() {
                        let part: Ref<Part> = part.borrow();
                        let (part_note_map, mut part_sign_map) = setup_level_to_midinote_maps(part_clef.get(&partidx).unwrap(), part_key.get(&partidx).unwrap());
                        match &part.ptype {
                            PartType::Music(music_type) => match music_type {
                                PartMusicType::Voices(voices) => match voices {
                                    Voices::One(voice) => match voice.borrow().vtype {
                                        VoiceType::Notes(ref notes) => {
                                            for (noteidx, note) in notes.items.iter().enumerate() {
                                                let note = note.borrow();
                                                match note.ntype {
                                                    NoteType::Heads(ref heads) => {
                                                        for (headidx, head) in heads.heads.iter().enumerate() {
                                                            let head = head.borrow();
                                                            let level = head.level;
                                                            let mapnote = part_note_map.get(&level).unwrap();
                                                            let mut mapsign = part_sign_map.get(&level).unwrap();
                                                            if let Some(accidental) = &head.accidental {
                                                                part_sign_map.insert(level, accidental.clone());
                                                                mapsign = &accidental;
                                                            }
                                                            let signed_note = mapnote + (*mapsign as i8);
                                                            let position = bar.position + note.position;
                                                            playnotes.push(PlayNote {
                                                                midinote: signed_note as u8,
                                                                volocity: 100,
                                                                duration: note.duration,
                                                                position,
                                                                partidx: partidx as u8,
                                                                voiceidx: 0,
                                                                noteidx: noteidx as u8,
                                                            })
                                                        }
                                                    }
                                                    _ => {}
                                                }
                                            }
                                        }
                                        _ => {}
                                    },
                                    Voices::Two(upper, lower) => {
                                        match upper.borrow().vtype {
                                            VoiceType::Notes(ref notes) => {
                                                for (noteidx, note) in notes.items.iter().enumerate() {
                                                    let note = note.borrow();
                                                    match note.ntype {
                                                        NoteType::Heads(ref heads) => {
                                                            for (headidx, head) in heads.heads.iter().enumerate() {
                                                                let head = head.borrow();
                                                                let level = head.level;
                                                                let mapnote = part_note_map.get(&level).unwrap();
                                                                let mut mapsign = part_sign_map.get(&level).unwrap();
                                                                if let Some(accidental) = &head.accidental {
                                                                    part_sign_map.insert(level, accidental.clone());
                                                                    mapsign = &accidental;
                                                                }
                                                                let signed_note = mapnote + (*mapsign as i8);
                                                                let position = bar.position + note.position;
                                                                playnotes.push(PlayNote {
                                                                    midinote: signed_note as u8,
                                                                    volocity: 100,
                                                                    duration: note.duration,
                                                                    position,
                                                                    partidx: partidx as u8,
                                                                    voiceidx: 0,
                                                                    noteidx: noteidx as u8,
                                                                })
                                                            }
                                                        }
                                                        _ => {}
                                                    }
                                                }
                                            }
                                            _ => {}
                                        }
                                        match lower.borrow().vtype {
                                            VoiceType::Notes(ref notes) => {
                                                for (noteidx, note) in notes.items.iter().enumerate() {
                                                    let note = note.borrow();
                                                    match note.ntype {
                                                        NoteType::Heads(ref heads) => {
                                                            for (headidx, head) in heads.heads.iter().enumerate() {
                                                                let head = head.borrow();
                                                                let level = head.level;
                                                                let mapnote = part_note_map.get(&level).unwrap();
                                                                let mut mapsign = part_sign_map.get(&level).unwrap();
                                                                if let Some(accidental) = &head.accidental {
                                                                    part_sign_map.insert(level, accidental.clone());
                                                                    mapsign = &accidental;
                                                                }
                                                                let signed_note = mapnote + (*mapsign as i8);
                                                                let position = bar.position + note.position;
                                                                playnotes.push(PlayNote {
                                                                    midinote: signed_note as u8,
                                                                    volocity: 100,
                                                                    duration: note.duration,
                                                                    position,
                                                                    partidx: partidx as u8,
                                                                    voiceidx: 0,
                                                                    noteidx: noteidx as u8,
                                                                })
                                                            }
                                                        }
                                                        _ => {}
                                                    }
                                                }
                                            }
                                            _ => {}
                                        }
                                    }
                                },
                                PartMusicType::RepeatBar(_) => {}
                            },
                            PartType::Nonmusic(_) => {}
                        }
                    }
                }
                BarType::MultiRest(_) => {}
                BarType::NonContent(_) => {}
                BarType::Invisible(ref notes) => {
                    for (noteidx, note) in notes.items.iter().enumerate() {
                        let note = note.borrow();
                        match note.ntype {
                            NoteType::Heads(ref heads) => {
                                for (headidx, head) in heads.heads.iter().enumerate() {
                                    let head = head.borrow();
                                    let position = bar.position + note.position;
                                    playnotes.push(PlayNote {
                                        midinote: 60,
                                        volocity: 100,
                                        duration: note.duration,
                                        position,
                                        partidx: 255,
                                        voiceidx: 0,
                                        noteidx: noteidx as u8,
                                    })
                                }
                            }
                            _ => {}
                        }
                    }
                }
                BarType::BarAttribute(attribute) => match attribute {
                    BarAttributeType::Clefs(clefs) => {
                        for (partidx, item) in clefs.iter().enumerate() {
                            if let Some(item) = item {
                                if let Some(item) = item {
                                    part_clef.insert(partidx, item.clone());
                                }
                            }
                        }
                    }
                    BarAttributeType::Times(times) => {
                        for (partidx, item) in times.iter().enumerate() {
                            if let Some(item) = item {
                                if let Some(item) = item {
                                    part_time.insert(partidx, item.clone());
                                }
                            }
                        }
                    }
                    BarAttributeType::Keys(keys) => {
                        for (partidx, item) in keys.iter().enumerate() {
                            if let Some(item) = item {
                                if let Some(item) = item {
                                    part_key.insert(partidx, item.clone());
                                }
                            }
                        }
                    }
                },
            }
        }
        let lastbar = self.items.last().unwrap().borrow();
        PlayNotesData {
            playnotes: playnotes,
            duration: lastbar.position + lastbar.duration,
        }
    }

    pub fn count_music_parts_voices(&self) -> Vec<u8> {
        let mut parts_voices: Vec<u8> = Vec::new();

        for (baridx, bar) in self.items.iter().enumerate() {
            let bar: Ref<Bar> = bar.borrow();
            match &bar.btype {
                BarType::Standard(parts) => {
                    for (partidx, part) in parts.iter().enumerate() {
                        if partidx + 1 > parts_voices.len() {
                            parts_voices.push(0);
                        }

                        let part: Ref<Part> = part.borrow();
                        match &part.ptype {
                            PartType::Music(music_type) => {
                                match music_type {
                                    PartMusicType::Voices(voices) => {
                                        match voices {
                                            Voices::One(voice) => {
                                                let voice = voice.borrow();
                                                match voice.vtype {
                                                    VoiceType::Notes(_) => {
                                                        if parts_voices[partidx] < 1 {
                                                            parts_voices[partidx] = 1;
                                                        }
                                                    }
                                                    _ => {}
                                                }

                                                //
                                            }
                                            Voices::Two(upper, lower) => {
                                                let voice = lower.borrow(); // no need to check upper
                                                match voice.vtype {
                                                    VoiceType::Notes(_) => {
                                                        if parts_voices[partidx] < 2 {
                                                            parts_voices[partidx] = 2;
                                                        }
                                                    }
                                                    _ => {}
                                                }
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
        parts_voices
    }
}

impl RMatrix {
    pub fn calculate_playpositions(&self) -> PlayPositionsData {
        let mut positions: PlayPositions = Vec::new();

        for (colidx, col) in self.cols.iter().enumerate() {
            let col: Ref<RCol> = col.borrow();
            let position = col.position;
            let x = col.x;
            positions.push(PlayPosition { position, x });
        }

        let top_y = self.rows.first().unwrap().borrow().y;
        let bottom_y = self.rows.last().unwrap().borrow().y;

        PlayPositionsData { positions, top_y, bottom_y }
    }
}

fn setup_level_to_midinote_maps(clef: &Clef, key: &Key) -> (BTreeMap<i8, i8>, BTreeMap<i8, Accidental>) {
    const LEVEL_LIMIT: i8 = 10;
    let mut levels_notes: BTreeMap<i8, i8> = BTreeMap::new();
    let mut levels_keymapsign: BTreeMap<i8, Accidental> = BTreeMap::new();
    let dia: Vec<i8> = vec![0, 2, 4, 5, 7, 9, 11];

    let clef_adjust = match clef {
        Clef::G => 6,
        Clef::F => -6,
        Clef::C => 0,
    };

    for lev in -LEVEL_LIMIT..=LEVEL_LIMIT {
        let inv = -lev + clef_adjust;
        let oct = (inv + 35) / 7;
        let stam = (inv + 49) % 7;
        let note = oct * 12 + dia[stam as usize];

        // println!("{lev} {note} {stam}");
        levels_notes.insert(lev, note);

        let mut sign = Accidental::Natural;
        match &key {
            Key::Sharps(sharp_count) => {
                if (*sharp_count as i8 >= 1) && (stam == 3) {
                    sign = Accidental::Sharp
                };
                if (*sharp_count as i8 >= 2) && (stam == 0) {
                    sign = Accidental::Sharp
                };
            }
            Key::Open => {}
            Key::Flats(flat_count) => {
                if (*flat_count as i8 >= 1) && (stam == 6) {
                    sign = Accidental::Flat
                };
                if (*flat_count as i8 >= 2) && (stam == 2) {
                    sign = Accidental::Flat
                };
            }
            Key::Naturals(_) => todo!(),
        }
        levels_keymapsign.insert(lev, sign.clone());
    }

    // for lev in -LEVEL_LIMIT..=LEVEL_LIMIT {
    //     println!("{lev} {:?} {:?}", levels_notes[&lev], levels_keymapsign[&lev]);
    // }

    (levels_notes, levels_keymapsign)
}

#[cfg(test)]
mod tests2 {
    use super::setup_level_to_midinote_maps;
    use crate::prelude::*;
    use std::collections::BTreeMap;

    #[test]
    fn example() {
        setup_level_to_midinote_maps(&Clef::F, &Key::Flats(2));
    }
}

// #[derive(Debug, Clone, Copy)]
// enum KeymapSign {
//     Sharp = 1,
//     Natural = 0,
//     Flat = -1,
// }
