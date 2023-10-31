use std::cell::RefMut;

use crate::prelude::*;
use itertools::Itertools;

impl Bars {
    pub fn resolve_stuff(&mut self) {
        self.map_id1_to_note();
        self.resolve_ties();
        self.resolve_lines();
        self.resolve_slurs();
    }
    pub fn resolve_lines(&self) {
        let items = self.consecutive_note_chunks();
        for item in items {
            let notes = item.2;
            match notes.len() {
                1 => {}
                _ => {
                    for (noteidx, (left, right)) in notes.iter().tuple_windows().enumerate() {
                        let mut left: RefMut<Note> = left.borrow_mut();
                        let mut right: RefMut<Note> = right.borrow_mut();

                        if left.lines.len() > 0 {
                            for line in left.lines.iter() {
                                let left_levels = &left.head_levels();
                                let left_level = left_levels.get(line.0 as usize);
                                let right_levels = &right.head_levels();
                                let right_level = right_levels.get(line.1 as usize);

                                if let Some(left_level) = left_level {
                                    if let Some(right_level) = right_level {
                                        right.lines_to.push(HeadLineTo(
                                            *left_level,
                                            *right_level,
                                            line.2,
                                        ))
                                    } else {
                                        println!("Right level does not exist! {:?}", line.1);
                                    }
                                } else {
                                    println!("Left level does not exist! {:?}", line.0);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn resolve_ties(&self) {
        let items = self.consecutive_note_chunks();

        for item in items {
            // println!("partidx {}, voiceidx {} -------------------------------", item.0, item.1);
            let notes = item.2;

            match notes.len() {
                1 => {
                    let mut note = notes[0].borrow_mut();
                    if note.ties.len() > 0 {
                        let mut ties_to_change_to_unresolved: Vec<usize> = Vec::new();
                        for (tiedataidx, tiedata) in note.ties.iter().enumerate() {
                            let level = tiedata.level;
                            let mut ttype = &tiedata.ttype;

                            match ttype {
                                TieFromType::Standard => {
                                    // println!("Standard tie in a one note chunk - should be let ring {}", tiedataidx);
                                    ties_to_change_to_unresolved.push(tiedataidx);
                                }
                                _ => {}
                            }
                        }

                        if ties_to_change_to_unresolved.len() > 0 {
                            for idx in ties_to_change_to_unresolved {
                                note.ties[idx].ttype = TieFromType::UnresolvedInChunk;
                            }
                        }
                    }
                }

                _ => {
                    for (noteidx, (left, right)) in notes.iter().tuple_windows().enumerate() {
                        let mut left = left.borrow_mut();

                        // dbg!(left.top_level(), &left.ties, left.ties_to.len());

                        if left.ties.len() > 0 {
                            // dbg!(left.id, left.ties.len(), left.ties_to.len());
                            let mut ties_to_change_to_unresolved: Vec<usize> = Vec::new();
                            let mut right: RefMut<Note> = right.borrow_mut();

                            for (tiedataidx, tiedata) in left.ties.iter().enumerate() {
                                let level = tiedata.level;
                                // dbg!(&level, &tiedata.ttype, right.has_level(level));
                                let ttype = &tiedata.ttype;

                                if right.has_level(level) {
                                    if let Some(tie_to) = right.get_level_tie_to(level) {
                                        println!("Right Level already has a tie_to! {:?}", tie_to);
                                    } else {
                                        let right_id = right.id;
                                        right.ties_to.push(TieToData {
                                            id1: right_id,
                                            level,
                                            ttype: TieToType::ResolveTieFrom(left.id, level),
                                        });
                                    }
                                } else {
                                    // println!("Right Level {} does not exist!", level);
                                    // println!("Turn left tie into a let ring one...");
                                    match ttype {
                                        TieFromType::Standard => {
                                            // println!("Standard tie in a one note chunk - should be unresolved {}", tiedataidx);
                                            ties_to_change_to_unresolved.push(tiedataidx);
                                        }
                                        _ => {}
                                    }
                                }
                            }

                            if ties_to_change_to_unresolved.len() > 0 {
                                for idx in ties_to_change_to_unresolved {
                                    // let level = note.ties[idx].level;
                                    left.ties[idx].ttype = TieFromType::UnresolvedInChunk;
                                }
                            }

                            // take care of last note in chunk -----------------------------------
                            if noteidx == notes.len() - 2 {
                                let mut right_ties_to_change_to_unresolved: Vec<usize> = Vec::new();
                                for (tiedataidx, tiedata) in right.ties.iter().enumerate() {
                                    match tiedata.ttype {
                                        TieFromType::Standard => {
                                            // println!("Standard tie in a one note chunk - should be unresolved {}", tiedataidx);
                                            right_ties_to_change_to_unresolved.push(tiedataidx);
                                        }
                                        _ => {}
                                    }
                                }
                                if right_ties_to_change_to_unresolved.len() > 0 {
                                    for idx in right_ties_to_change_to_unresolved {
                                        // let level = note.ties[idx].level;
                                        right.ties[idx].ttype = TieFromType::UnresolvedInChunk;
                                    }
                                }
                            }
                            //-----------------------------------------------------------------------
                        }
                    }
                }
            }
        }
    }

    fn resolve_slurs(&self) {
        let items = self.consecutive_note_chunks();
        for item in items {
            let notes = item.2;
            // dbg!(&item.0, &item.1, notes.len());
        }
    }

    fn map_id1_to_note(&mut self) {
        for (baridx, bar) in self.items.iter().enumerate() {
            let bar = bar.borrow();
            match bar.btype {
                BarType::Standard(ref parts) => {
                    for part in parts {
                        // let part = part.borrow();
                        let part = part.borrow();
                        let complexes = part
                            .complexes
                            .as_ref()
                            .expect("Part should have complexes!");
                        for complex in complexes {
                            let complex = complex.borrow();
                            let mut ritem = complex.matrix_item.as_ref().unwrap().borrow_mut();
                            match complex.ctype {
                                ComplexType::Single(ref note, _)
                                | ComplexType::Upper(ref note, _) => {
                                    self.id1_map.insert(note.borrow().id, note.clone());
                                    ritem.notedata.id1 = RItemNoteType::Note(note.borrow().id);
                                    // ritem.id1 = Some(note.borrow().id);
                                }
                                ComplexType::Lower(ref note, _) => {
                                    self.id1_map.insert(note.borrow().id, note.clone());
                                    ritem.notedata.id2 = RItemNoteType::Note(note.borrow().id);
                                }
                                ComplexType::Two(ref upper, ref lower, _) => {
                                    self.id1_map.insert(upper.borrow().id, upper.clone());
                                    self.id1_map.insert(lower.borrow().id, lower.clone());
                                    ritem.notedata.id1 = RItemNoteType::Note(upper.borrow().id);
                                    ritem.notedata.id2 = RItemNoteType::Note(lower.borrow().id);
                                }
                                ComplexType::OneBarpause(_) => {}
                                ComplexType::TwoBarpauses(_, _) => {}
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
