use crate::prelude::*;

use crate::beamgroup::calc::*;

#[derive(Debug, PartialEq)]
pub struct Beamgroup {
    pub id: usize,
    pub notes: Vec<Rc<RefCell<Note>>>,
    pub duration: usize,
    pub direction: Option<DirUD>,
    pub top: i8,
    pub bottom: i8,
    // pub tilt: Option<(f32, f32)>,
    pub start_level: f32,
    pub end_level: f32,
    pub note_durations: Vec<Duration>,
}

impl Beamgroup {
    pub fn new(notes: Vec<Rc<RefCell<Note>>>) -> Self {
        let mut duration = 0;
        let mut note_durations = vec![];
        for note in notes.iter() {
            let note_duration = note.borrow().duration;
            note_durations.push(note_duration);
            duration += note.borrow().duration;
        }

        let top = notes.iter().map(|note| note.borrow().top_level()).min().unwrap();
        let bottom = notes.iter().map(|note| note.borrow().bottom_level()).max().unwrap();

        Self {
            id: ID_COUNTER.fetch_add(1, Ordering::Relaxed),
            notes,
            duration,
            direction: None,
            top,
            bottom,
            // tilt: None,
            start_level: 0.0,
            end_level: 0.0,
            note_durations,
        }
    }

    pub fn calc_direction(&self) -> DirUD {
        let balance = self.bottom + self.top;
        if balance <= 0 {
            DirUD::Down
        } else {
            DirUD::Up
        }
    }

    pub fn is_single_note(&self) -> bool {
        self.notes.len() == 1
    }
}

pub type Beamgroups = Vec<Rc<RefCell<Beamgroup>>>;

#[derive(Debug, Clone)]
pub enum BeamingPattern {
    NoBeams,
    NValues(Vec<usize>),
}

pub fn get_beamgroups(notes: &Notes, pattern: &BeamingPattern) -> Result<Beamgroups> {
    if notes.items.is_empty() {
        return Err(Generic(format!("notes is empty")).into());
    }

    if notes.items.len() == 1 {
        return Ok(vec![Rc::new(RefCell::new(Beamgroup::new(vec![notes.items[0].clone()])))]);
    }

    let result = match pattern {
        BeamingPattern::NoBeams => get_beamgroups_nobeams(notes)?,
        BeamingPattern::NValues(ref values) => get_beamgroups_nvalues(notes, values)?,
    };

    Ok(result.into_iter().map(|item| Rc::new(RefCell::new(item))).collect::<Vec<_>>())
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    #[test]
    fn example() {
        let notes = QCode::notes("nv4 0 1 2").unwrap();
        let groups = get_beamgroups(&notes, &BeamingPattern::NoBeams).unwrap();
        let mut iter = groups.iter();
        assert_eq!(iter.next().unwrap().borrow().notes.len(), 1);
        assert_eq!(iter.next().unwrap().borrow().notes.len(), 1);
        assert_eq!(iter.next().unwrap().borrow().notes.len(), 1);

        let mut iter = groups.iter();
        assert_eq!(iter.next().unwrap().borrow().duration, NV4);
        assert_eq!(iter.next().unwrap().borrow().duration, NV4);
        assert_eq!(iter.next().unwrap().borrow().duration, NV4);
    }
    #[test]
    fn example2() {
        let notes = QCode::notes("nv8 0 1 2 nv16 0 0 0 nv8 1 nv16 0").unwrap();
        let groups = get_beamgroups(&notes, &BeamingPattern::NValues(vec![NV4])).unwrap();
        let mut iter = groups.iter();
        assert_eq!(iter.next().unwrap().borrow().notes.len(), 2);
        assert_eq!(iter.next().unwrap().borrow().notes.len(), 3);
        assert_eq!(iter.next().unwrap().borrow().notes.len(), 3);

        let mut iter = groups.iter();
        assert_eq!(iter.next().unwrap().borrow().duration, NV4);
        assert_eq!(iter.next().unwrap().borrow().duration, NV4);
        assert_eq!(iter.next().unwrap().borrow().duration, NV4);
    }
    #[test]
    fn example3() {
        let notes = QCode::notes("nv8dot 0 nv8 p nv16 0 0 0 0").unwrap();
        let groups = get_beamgroups(&notes, &BeamingPattern::NValues(vec![NV4])).unwrap();
        let mut iter = groups.iter();
        assert_eq!(iter.next().unwrap().borrow().notes.len(), 1);
        assert_eq!(iter.next().unwrap().borrow().notes.len(), 1);
        assert_eq!(iter.next().unwrap().borrow().notes.len(), 3);
        assert_eq!(iter.next().unwrap().borrow().notes.len(), 1);

        let mut iter = groups.iter();
        assert_eq!(iter.next().unwrap().borrow().duration, NV8DOT);
        assert_eq!(iter.next().unwrap().borrow().duration, NV8);
        assert_eq!(iter.next().unwrap().borrow().duration, NV8DOT);
        assert_eq!(iter.next().unwrap().borrow().duration, NV16);
    }
    #[test]
    fn example4() {
        let notes = QCode::notes("nv8 p 0").unwrap();
        let groups = get_beamgroups(&notes, &BeamingPattern::NValues(vec![NV4])).unwrap();
        let mut iter = groups.iter();
        assert_eq!(iter.next().unwrap().borrow().notes.len(), 1);
        assert_eq!(iter.next().unwrap().borrow().notes.len(), 1);
    }
    #[test]
    fn example5() {
        let notes = QCode::notes("nv1 0 nv2 0 nv4 0 nv8 0 0 nv16 0 0 0 0").unwrap();
        let groups = get_beamgroups(&notes, &BeamingPattern::NValues(vec![NV4])).unwrap();
        let mut iter = groups.iter();
        assert_eq!(iter.next().unwrap().borrow().duration, NV1);
        assert_eq!(iter.next().unwrap().borrow().duration, NV2);
        assert_eq!(iter.next().unwrap().borrow().duration, NV4);
        assert_eq!(iter.next().unwrap().borrow().duration, NV4);
        assert_eq!(iter.next().unwrap().borrow().duration, NV4);
    }
}
