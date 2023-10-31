use std::{cell::Ref, fmt::Formatter};

use crate::prelude::*;

#[derive(Debug, PartialEq)]
pub struct Complex {
    pub id: usize,
    // pub notes: Vec<Rc<RefCell<Note>>>,
    pub ctype: ComplexType,
    pub position: Position,
    pub duration: Duration,
    pub rects: Vec<Rc<RefCell<NRectExt>>>,
    pub matrix_item: Option<Rc<RefCell<RItem>>>,
    // pub ties: Vec<TieData>,
    // pub ties_to: Vec<TieToData>,
}

impl Complex {
    pub fn new(ctype: ComplexType, position: Position) -> Self {
        // // collect ties from heads------------------------------------------------
        // let mut ties: Vec<TieData> = Vec::new();
        // let mut ties_to: Vec<TieToData> = Vec::new();
        // let mut do_ties = |heads: &Heads| {
        //     for head in heads.heads.iter() {
        //         let head: Ref<Head> = head.borrow();
        //         if let Some(tie) = &head.tie {
        //             ties.push(TieData {
        //                 ttype: tie.clone(),
        //                 level: head.level,
        //             });
        //         }
        //         if let Some(tie) = &head.tie_to {
        //             ties_to.push(TieToData {
        //                 ttype: tie.clone(),
        //                 level: head.level,
        //             });
        //         }
        //     }
        // };
        // match &ctype {
        //     ComplexType::Single(note, _) | ComplexType::Upper(note, _) | ComplexType::Lower(note, _) => match &note.borrow().ntype {
        //         NoteType::Heads(heads) => {
        //             do_ties(&heads);
        //         }
        //         _ => {}
        //     },
        //     ComplexType::Two(upper, lower, _) => {
        //         //
        //         match &upper.borrow().ntype {
        //             NoteType::Heads(heads) => {
        //                 do_ties(&heads);
        //             }
        //             _ => {}
        //         }
        //         match &lower.borrow().ntype {
        //             NoteType::Heads(heads) => {
        //                 do_ties(&heads);
        //             }
        //             _ => {}
        //         }
        //     }
        // }

        // // ------------------------------------------------------
        // dbg!(&ties);
        // dbg!(&ties_to);
        Self {
            id: ID_COUNTER.fetch_add(1, Ordering::Relaxed),
            position,
            ctype,
            duration: 0,
            rects: Vec::new(),
            matrix_item: None,
        }
    }

    pub fn print(&self) {
        match &self.ctype {
            ComplexType::Single(note, _) => {
                println!(
                    "Complex pos {}: Single({:?})",
                    &self.position,
                    note.borrow().position
                );
            }
            ComplexType::Two(note1, note2, overlap) => {
                println!(
                    "Complex pos {}: Two({:?} {:?}, {:?} {:?}, {:?})",
                    &self.position,
                    note1.borrow().position,
                    note1.borrow().head_levels(),
                    note2.borrow().position,
                    note2.borrow().head_levels(),
                    overlap
                );
            }
            ComplexType::Upper(note, overflow) => {
                println!(
                    "Complex pos {}: Upper({:?} {:?} overflow:{:?})",
                    &self.position,
                    note.borrow().position,
                    note.borrow().head_levels(),
                    overflow
                );
            }
            ComplexType::Lower(note, overflow) => {
                println!(
                    "Complex pos {}: Lower({:?} {:?} overflow:{:?})",
                    &self.position,
                    note.borrow().position,
                    note.borrow().head_levels(),
                    overflow
                );
            }
            ComplexType::OneBarpause(duration) => {
                println!("Complex pos {}: OneBarpause", &self.position);
            }
            ComplexType::TwoBarpauses(_, _) => {
                println!("Complex pos {}: TwoBarpauses", &self.position);
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ComplexType {
    Single(Rc<RefCell<Note>>, bool),
    //
    Two(
        Rc<RefCell<Note>>,
        Rc<RefCell<Note>>,
        Option<ComplexXAdjustment>,
    ),
    Upper(Rc<RefCell<Note>>, bool),
    Lower(Rc<RefCell<Note>>, bool),
    OneBarpause(Duration),
    TwoBarpauses(Duration, Duration),
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ComplexXAdjustment {
    UpperRight(f32),
    LowerRight(f32),
}

impl ComplexXAdjustment {
    pub fn as_f32(&self) -> f32 {
        match self {
            ComplexXAdjustment::UpperRight(f) => *f,
            ComplexXAdjustment::LowerRight(f) => *f,
        }
    }
}
