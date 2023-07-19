use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Hash)]
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

impl Default for HeadPlacement {
    fn default() -> Self {
        Self::Center
    }
}

pub type HeadsPlacement = Vec<(i8, HeadPlacement, Rc<RefCell<Head>>)>;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Head {
    pub level: i8,
    pub accidental: Option<Accidental>,
    pub tie: Option<Tie>,
    // calculated
    pub placement: HeadPlacement,
}

impl Head {
    pub fn new(level: i8) -> Self {
        Self {
            level,
            tie: None,
            accidental: None,
            placement: HeadPlacement::Center,
        }
    }

    pub fn new_with_attributes(
        level: i8,
        accidental: Option<Accidental>,
        tie: Option<Tie>,
    ) -> Self {
        Self {
            level,
            accidental,
            tie,
            placement: HeadPlacement::Center,
        }
    }
}

impl Default for Head {
    fn default() -> Self {
        Self {
            level: 0,
            accidental: None,
            tie: None,
            placement: HeadPlacement::Center,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Heads {
    pub heads: Vec<Rc<RefCell<Head>>>,
    pub top: i8,
    pub bottom: i8,
}

impl Heads {
    pub fn new(mut heads: Vec<Head>) -> Self {
        heads.sort_by_key(|item| item.level);
        let heads: Vec<Rc<RefCell<Head>>> = heads
            .into_iter()
            .map(|item| Rc::new(RefCell::new(item)))
            .collect();
        let top = heads[0].borrow().level;
        let bottom = heads[heads.len() - 1].borrow().level;

        Self { heads, top, bottom }
    }

    pub fn levels(&self) -> Vec<i8> {
        self.heads.iter().map(|head| head.borrow().level).collect()
    }

    pub fn levels_heads(&self) -> Vec<(i8, Rc<RefCell<Head>>)> {
        self.heads
            .iter()
            .map(|head| (head.borrow().level, head.clone()))
            .collect()
    }

    // pub fn levels_accidentals(&self) -> Vec<(i8, Accidental)> {
    // &self
    //     .heads
    //     .into_iter()
    //     .filter(|head| head.borrow().accidental.is_some())
    //     .map(|head| {
    //         let head = head.borrow();
    //         (head.level.clone(), head.accidental.unwrap().clone())
    //     })
    //     .collect::<Vec<_>>();

    //     Vec::new()
    // }

    pub fn levels_accidentals(&self) -> Vec<(i8, Accidental)> {
        let mut result: Vec<(i8, Accidental)> = Vec::new();
        for head in &self.heads {
            let head = head.borrow();
            if head.accidental.is_some() {
                result.push((
                    head.level.clone(),
                    head.accidental.as_ref().unwrap().clone(),
                ));
            }
        }
        result
    }

    pub fn levels_ties(&self) -> Vec<(i8, Tie)> {
        let mut result: Vec<(i8, Tie)> = Vec::new();
        for head in &self.heads {
            let head = head.borrow();
            if head.tie.is_some() {
                result.push((head.level.clone(), head.tie.as_ref().unwrap().clone()));
            }
        }
        result
    }

    // pub fn head_from_level(&self, level: i8) -> Option<Rc<RefCell<Head>>> {
    //     for head in &self.heads {
    //         if head.borrow().level == level {
    //             return Some(head.clone());
    //         }
    //     }
    //     None
    // }

    // pub fn levels_ties(&self) -> Vec<(&i8, &Tie)> {
    //     self.heads
    //         .into_iter()
    //         .filter(|head| head.borrow().tie.is_some())
    //         .map(|head| (&head.borrow().level, &head.borrow().tie.unwrap()))
    //         .collect::<Vec<_>>()
    // }
}

#[derive(Debug)]
pub struct HeadAttributes {
    // pub accidental: Option<Accidental>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum HeadType {
    NormalHead,
    WideHead,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum HeadShape {
    BlackHead,
    WhiteHead,
    WholeHead,
}

#[cfg(test)]
mod tests2 {
    use crate::prelude::*;
    #[test]
    fn example() {
        let notes = QCode::notes("nv4 #0,b1 0_,2n_ bb1 ##1 n2 b1").unwrap();
        // dbg!(notes);
        // let json = serde_json::to_string_pretty(&notes).unwrap();
        // println!("{}", json);
    }

    #[test]
    fn levels_accidentals() {
        let notes = QCode::notes("#1,2,bb3").unwrap();
        let note = notes.items[0].borrow();
        let la = note.levels_accidentals();
        dbg!(&la);
    }
}
