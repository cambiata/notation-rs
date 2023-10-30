use crate::prelude::NRect;
use crate::{prelude::*, types::some_cloneables::SomeCloneablePairs};
use std::cell::{Ref, RefMut};

#[derive(Debug, PartialEq)]
pub enum RItemType {
    Unset,
    Cluster,
    Clef,
    Key,
    Time,
    Space,
    NonContent,
    Barline,
    Other,
}

#[derive(Debug, PartialEq)]
pub enum RItemNoteType {
    Unset,
    Note(usize),
    BarPause(Duration),
    Other,
}

#[derive(Debug, PartialEq)]
pub struct RItemNoteData {
    pub id1: RItemNoteType,
    pub id2: RItemNoteType,
    pub steminfo1: StemInfo,
    pub steminfo2: StemInfo,
    pub beamdata1: RItemBeam,
    pub beamdata2: RItemBeam,
}

#[derive(Debug, PartialEq)]
pub struct RItem {
    pub id: usize,
    pub itype: RItemType,
    pub duration: Duration,
    pub col_idx: usize,
    pub row_idx: usize,
    pub coord_x: Option<f32>,
    pub coord_y: Option<f32>,
    pub nrects: Option<Vec<Rc<RefCell<NRectExt>>>>,
    pub notedata: RItemNoteData,
    pub lines: Vec<(i8, i8, HeadLineType)>,
}

impl RItem {
    pub fn new(itype: RItemType, rects: Vec<NRect>, dur: Duration) -> Self {
        let nrects = rects
            .iter()
            .map(|r| NRectExt::new(*r, NRectType::DUMMY))
            .collect::<Vec<_>>();

        Self {
            id: ID_COUNTER.fetch_add(1, Ordering::Relaxed),
            itype,
            duration: dur,
            col_idx: 0,
            row_idx: 0,
            // coords: None,
            coord_x: None,
            coord_y: None,

            nrects: None,

            notedata: RItemNoteData {
                id1: RItemNoteType::Unset,
                id2: RItemNoteType::Unset,
                beamdata1: RItemBeam::None,
                beamdata2: RItemBeam::None,
                steminfo1: StemInfo::None,
                steminfo2: StemInfo::None,
            },
            lines: vec![],
        }
    }

    pub fn new_with_nrectsext(itype: RItemType, rects: Vec<NRect>, dur: Duration) -> Self {
        let nrects: Vec<Rc<RefCell<NRectExt>>> = rects
            .iter()
            .map(|r| {
                Rc::new(RefCell::new(NRectExt::new(
                    *r,
                    NRectType::WIP("hoho".to_string()),
                )))
            })
            .collect::<Vec<_>>();

        Self {
            id: ID_COUNTER.fetch_add(1, Ordering::Relaxed),
            itype,
            duration: dur,
            col_idx: 0,
            row_idx: 0,
            // coords: None,
            coord_x: None,
            coord_y: None,
            nrects: Some(nrects),

            notedata: RItemNoteData {
                id1: RItemNoteType::Unset,
                id2: RItemNoteType::Unset,
                beamdata1: RItemBeam::None,
                beamdata2: RItemBeam::None,
                steminfo1: StemInfo::None,
                steminfo2: StemInfo::None,
            },
            lines: vec![],
        }
    }

    pub fn new_from_nrects(
        itype: RItemType,
        nrects: Vec<Rc<RefCell<NRectExt>>>,
        dur: Duration,
    ) -> Self {
        let mut rects: Vec<NRect> = vec![];

        for nrect in nrects.iter() {
            let nrect: Ref<NRectExt> = nrect.borrow();
            rects.push(nrect.0.clone());
        }

        let nrects_clones: Vec<Rc<RefCell<NRectExt>>> =
            nrects.iter().map(|nrect| nrect.clone()).collect::<Vec<_>>();

        Self {
            // rects,
            id: ID_COUNTER.fetch_add(1, Ordering::Relaxed),
            itype,
            duration: dur,
            col_idx: 0,
            row_idx: 0,
            // coords: None,
            coord_x: None,
            coord_y: None,
            nrects: Some(nrects_clones),

            notedata: RItemNoteData {
                id1: RItemNoteType::Unset,
                id2: RItemNoteType::Unset,
                beamdata1: RItemBeam::None,
                beamdata2: RItemBeam::None,
                steminfo1: StemInfo::None,
                steminfo2: StemInfo::None,
            },
            lines: vec![],
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum StemInfo {
    FullInfo(f32, f32, f32, f32), // stem x, stem y, head_width, stem h
    BeamMiddle(usize, f32, f32, f32),
    None,
}

#[derive(Debug, PartialEq)]
pub enum RItemBeam {
    None,
    Single(RItemBeamData),
    Start(RItemBeamData),
    Middle(RItemBeamData),
    End(RItemBeamData),
}

#[derive(Debug, PartialEq, Clone)]
pub struct RItemBeamData {
    pub id: usize,
    pub id1: usize,
    pub note_position: usize,
    pub direction: DirUD,
    pub duration: Duration,

    pub tip_level: f32,
    pub top_level: i8,
    pub bottom_level: i8,

    pub has_stem: bool,
    pub adjustment_x: Option<ComplexXAdjustment>,
    pub head_width: f32,
    pub note_durations: Option<Vec<Duration>>,
    pub lower_voice: bool,
}
