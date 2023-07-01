use crate::{chord::ChordItem, dynamic::DynamicItem, prelude::*};

use serde::{Deserialize, Serialize};
use std::{sync::atomic::AtomicUsize, sync::atomic::Ordering};

static ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

use crate::heads::Heads;

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Note {
    pub id: usize,
    pub duration: Duration,
    pub ntype: NoteType,
    pub attr: NoteAttributes,
}

impl Note {
    pub fn from_heads(duration: usize, heads: Heads) -> Note {
        Note::new(
            duration,
            NoteType::Heads(heads),
            NoteAttributes { color: None },
        )
    }

    pub fn new(duration: usize, ntype: NoteType, attr: NoteAttributes) -> Note {
        Note {
            id: ID_COUNTER.fetch_add(1, Ordering::Relaxed),
            duration,
            ntype,
            attr,
        }
    }

    pub fn is_beamable(self: &Note) -> bool {
        match self.ntype {
            // normal note
            NoteType::Heads(_) => duration_is_beamable(self.duration),
            _ => false,
        }
    }

    pub fn get_heads_top(self: &Note) -> i8 {
        match self.ntype {
            NoteType::Heads(ref heads) => heads.get_level_top(),
            _ => 0,
        }
    }

    pub fn get_heads_bottom(self: &Note) -> i8 {
        match self.ntype {
            NoteType::Heads(ref heads) => heads.get_level_bottom(),
            _ => 0,
        }
    }

    pub fn get_heads_placements<'a>(self: &Note, dir: &DirUD) -> Option<HeadsPlacement> {
        if let NoteType::Heads(ref heads) = self.ntype {
            let levels = heads.get_levels();
            if levels.len() == 1 {
                return Some(vec![(levels[0], &HeadPlacement::Center, &heads.items[0])]);
            }
            //------------------------------------------------------------
            let mut result: HeadsPlacement = Vec::new();
            return match dir {
                DirUD::Up => {
                    for (idx, level_pair) in levels
                        .into_iter()
                        .rev()
                        .collect::<Vec<i8>>()
                        .windows(2)
                        .enumerate()
                    {
                        let lower_level = level_pair[0];
                        let upper_level = level_pair[1];
                        let diff = lower_level - upper_level;
                        let head = &heads.items[idx];

                        if idx == 0 {
                            result.push((lower_level, &HeadPlacement::Center, head));
                            if diff < 2 {
                                result.push((upper_level, &HeadPlacement::Right, head));
                            } else {
                                result.push((upper_level, &HeadPlacement::Center, head));
                            }
                        } else {
                            let (current_level, current_placement, head) = &result[idx];
                            match diff {
                                0 | 1 => {
                                    if let HeadPlacement::Center = current_placement {
                                        result.push((upper_level, &HeadPlacement::Right, head));
                                    } else {
                                        result.push((upper_level, &HeadPlacement::Center, head));
                                    }
                                }
                                _ => {
                                    result.push((upper_level, &HeadPlacement::Center, head));
                                }
                            }
                        }
                    }
                    Some(result)
                }
                DirUD::Down => {
                    for (idx, level_pair) in levels.windows(2).enumerate() {
                        let upper_level = level_pair[0];
                        let lower_level = level_pair[1];
                        let diff = lower_level - upper_level;
                        let head = &heads.items[idx];

                        if idx == 0 {
                            result.push((upper_level, &HeadPlacement::Center, head));
                            if diff < 2 {
                                result.push((lower_level, &HeadPlacement::Left, head));
                            } else {
                                result.push((lower_level, &HeadPlacement::Center, head));
                            }
                        } else {
                            let (current_level, current_placement, _) = &result[idx];
                            match diff {
                                0 | 1 => {
                                    if let HeadPlacement::Center = current_placement {
                                        result.push((lower_level, &HeadPlacement::Left, head));
                                    } else {
                                        result.push((lower_level, &HeadPlacement::Center, head));
                                    }
                                }
                                _ => {
                                    result.push((lower_level, &HeadPlacement::Center, head));
                                }
                            }
                        }
                    }
                    Some(result)
                }
            };
        }
        None
    }

    pub fn get_accidentals(self: &Note) -> Option<Vec<(i8, &Accidental)>> {
        if let NoteType::Heads(ref heads) = self.ntype {
            let mut result: Vec<(i8, &Accidental)> = Vec::new();
            for head in heads {
                if let Some(accidental) = &head.accidental {
                    result.push((head.level, accidental));
                }
            }
            return Some(result);
        }
        None
    }
}

#[derive(Debug, Clone)]
pub enum HeadPlacement {
    Left,
    Center,
    Right,
}

impl HeadPlacement {
    pub fn as_f32(self: &HeadPlacement) -> f32 {
        match self {
            HeadPlacement::Left => -1.0,
            HeadPlacement::Center => 0.0,
            HeadPlacement::Right => 1.0,
        }
    }
}

pub type HeadsPlacement<'a> = Vec<(i8, &'a HeadPlacement, &'a Head)>;

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum NoteType {
    Heads(Heads),
    Pause,
    Slash,
    Lyric(Syllable),
    Dynamic(DynamicItem),
    Chord(ChordItem),
    Spacer,
}

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct NoteAttributes {
    pub color: Option<u16>,
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn example() {
        let notes = QCode::notes("4,3,0,-2 -2,-3,0,2").unwrap();
        let note0 = notes.get_note_idx(0).unwrap();
        dbg!(note0.get_heads_placements(&DirUD::Up));
        let note1 = notes.get_note_idx(1).unwrap();
        dbg!(note1.get_heads_placements(&DirUD::Down));
    }
}
