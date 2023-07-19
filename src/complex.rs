use std::fmt::Formatter;

use crate::prelude::*;

#[derive(Debug, PartialEq)]
pub struct Complex {
    // pub notes: Vec<Rc<RefCell<Note>>>,
    pub ctype: ComplexType,
    pub position: Position,
    pub duration: Duration,

    pub rects: Rc<RefCell<Vec<NRectExt>>>,
}

impl Complex {
    pub fn new(ctype: ComplexType, position: Position) -> Self {
        // let mut duration = 0;
        // let mut position = 0;
        // match &notes.len() {
        //     0 => panic!("Complex must have at least one note"),
        //     1 => {
        //         position = notes[0].borrow().position;
        //         duration = notes[0].borrow().duration;
        //     }
        //     2 => {
        //         if notes[0].borrow().position != notes[1].borrow().position {
        //             panic!("Complex notes must have same position and duration");
        //         }
        //         position = notes[0].borrow().position;
        //         duration = std::cmp::max(notes[0].borrow().duration, notes[1].borrow().duration);
        //     }
        //     _ => panic!("Complex must have at most two notes"),
        // }

        Self {
            position,
            ctype,
            duration: 0,
            rects: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn print(&self) {
        match &self.ctype {
            ComplexType::Single(note) => {
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
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ComplexType {
    Single(Rc<RefCell<Note>>),
    //
    Two(
        Rc<RefCell<Note>>,
        Rc<RefCell<Note>>,
        Option<ComplexXAdjustment>,
    ),
    Upper(Rc<RefCell<Note>>, bool),
    Lower(Rc<RefCell<Note>>, bool),
}

#[derive(Debug, PartialEq)]
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
