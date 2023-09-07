use crate::prelude::NRect;
use crate::{prelude::*, types::some_cloneables::SomeCloneablePairs};
use std::cell::{Ref, RefMut};
use std::collections::BTreeMap;

#[derive(Debug, PartialEq)]
pub struct RMatrix {
    pub cols: Vec<Rc<RefCell<RCol>>>,
    pub rows: Vec<Rc<RefCell<RRow>>>,
    pub width: f32,
    pub height: f32,
    pub bartemplate: Option<BarTemplate>,
}

impl RMatrix {
    pub fn new(colitems: Vec<Rc<RefCell<RCol>>>, bartemplate: Option<BarTemplate>) -> Self {
        let row_count = &colitems[0].borrow().items.len();
        let mut rows: Vec<Rc<RefCell<RRow>>> = vec![];
        let mut rowitems: Vec<Vec<Option<Rc<RefCell<RItem>>>>> = vec![vec![]; *row_count];

        let firstcol = &colitems[0];
        // for item in firstcol.borrow().items.iter() {
        //     if item.is_none() {
        //         panic!("firstcol has None - shouldn't have!");
        //     }
        // }

        let mut colidx = 0;
        for col in &colitems {
            let col: &RefCell<RCol> = col;

            // check for rows integrity
            if col.borrow().items.len() != *row_count {
                panic!("part_count mismatch");
            }

            let mut rowidx = 0;
            for row in col.borrow_mut().items.iter_mut() {
                // set rowidx and colidx for item
                if let Some(row) = row {
                    let row: &RefCell<RItem> = row;
                    row.borrow_mut().col_idx = colidx;
                    row.borrow_mut().row_idx = rowidx;
                }

                // add cloned item to rowitems
                let rowitem = if let Some(row) = row {
                    let item = row.clone();
                    Some(row.clone())
                } else {
                    None
                };
                rowitems[rowidx].push(rowitem);

                rowidx += 1;
            }
            colidx += 1;
        }

        for ritems in rowitems {
            let row = RRow::new(ritems, 0.0);
            rows.push(Rc::new(RefCell::new(row)));
        }

        Self {
            cols: colitems,
            rows: rows,
            width: 0.0,
            height: 0.0,
            bartemplate,
        }
    }

    pub fn get_column(&self, idx: usize) -> Option<&Rc<RefCell<RCol>>> {
        if idx < self.cols.len() {
            return Some(&self.cols[idx]);
        }
        None
    }

    pub fn get_row(&self, idx: usize) -> Option<&Rc<RefCell<RRow>>> {
        if idx < self.rows.len() {
            return Some(&self.rows[idx]);
        }
        None
    }

    pub fn calculate_col_spacing(&self, spacing_fn: SpacingFn) {
        // spacing based on duration
        for col in self.cols.iter() {
            let mut col = col.borrow_mut();
            let allotment_w = spacing_fn(&col.duration);
            col.distance_x = allotment_w;
            col.spacing_duration = col.distance_x;
            col.alloted_duration = allotment_w;
        }

        // spacing correction based on overlap
        for row in self.rows.iter() {
            let row = row.borrow();

            let pairs = SomeCloneablePairs { items: row.items.clone() };
            for (left, left_idx, right, right_idx) in pairs.into_iter() {
                match [&left, &right] {
                    [Some(left), Some(right)] => {
                        let left = left.borrow_mut();
                        let right = right.borrow_mut();
                        let mut left_col = self.get_column(left.col_idx).unwrap().borrow_mut();
                        let mut right_col = self.get_column(right.col_idx).unwrap().borrow_mut();

                        let left_rects = &left.nrects.as_ref().unwrap().iter().map(|nrect| nrect.borrow().0).collect::<Vec<_>>();

                        let right_rects = &right.nrects.as_ref().unwrap().iter().map(|nrect| nrect.borrow().0).collect::<Vec<_>>();

                        // calculate spacings...
                        // let overlap_spacing: f32 =nrects_overlap_x(&left.rects, &right.rects).unwrap_or(0.0);
                        let overlap_spacing: f32 = nrects_overlap_x(&left_rects, &right_rects).unwrap_or(0.0);

                        let spacing = if (right_idx - 1) != left_idx.unwrap() {
                            let mut prev_col = self.get_column(right.col_idx - 1).unwrap().borrow();
                            overlap_spacing - prev_col.distance_x
                        } else {
                            overlap_spacing
                        };
                        left_col.distance_x = left_col.distance_x.max(spacing);
                        //

                        left_col.spacing_overlap = left_col.spacing_overlap.max(overlap_spacing);
                        left_col.overlap_overshoot = left_col.overlap_overshoot.max(left_col.spacing_overlap - left_col.spacing_duration);
                    }

                    [Some(left), None] => {
                        panic!("Should not happen - right should always be Some(T)");
                    }
                    [None, Some(right)] => {
                        let right = right.borrow();
                        let right_col = self.get_column(right.col_idx);
                        if let Some(right_col) = right_col {
                            let right_col_mut = right_col.borrow_mut();
                        }
                    }
                    [None, None] => {
                        panic!("Should not happen - right should always be Some(T)");
                    }
                }
            }
        }
    }

    pub fn calculate_row_spacing(&self) {
        let mut rowrects: Vec<Vec<NRect>> = Vec::new();

        for (rowidx, row) in self.rows.iter().enumerate() {
            let mut colx = 0.0;
            let row = row.borrow();
            let mut itemrects: Vec<NRect> = Vec::new();
            for (colidx, item) in row.items.iter().enumerate() {
                let col = self.get_column(colidx).unwrap().borrow();
                if let Some(item) = item {
                    let item = item.borrow();
                    // for rect in item.rects.iter() {
                    //     let rect = rect.move_rect(colx, 0.0);
                    //     itemrects.push(rect);
                    // }

                    if let Some(nrects) = &item.nrects {
                        // let nrects: Vec<Rc<RefCell<NRectExt>>> = nrects.borrow();
                        for nrect in nrects.iter() {
                            let mut nrect = nrect.borrow();
                            let rect = nrect.0.move_rect(colx, 0.0);
                            itemrects.push(rect);
                        }
                    }
                };
                colx += col.distance_x;
            }
            rowrects.push(itemrects);
        }

        let mut rowidx = 0;
        for pair in rowrects.windows(2) {
            let (uppers, lowers) = (&pair[0], &pair[1]);

            let overlap = nrects_overlap_y(uppers, lowers).unwrap_or(0.0);
            let mut row = self.get_row(rowidx).unwrap().borrow_mut();
            row.distance_y = row.distance_y.max(overlap) + SPACE; // Todo
            rowidx += 1;
        }
    }

    pub fn calculate_col_measurements(&mut self) {
        let mut x = 0.0;
        for col in &self.cols {
            let mut col = col.borrow_mut();
            col.x = x;
            x += col.distance_x.round();
        }
    }

    pub fn calculate_col_row_item_measurements(&mut self) {
        let mut x = 0.0;
        for col in &self.cols {
            let mut col = col.borrow_mut();
            let mut y = 0.0;
            let mut rowidx = 0;
            for item in &col.items {
                if let Some(item) = item {
                    let mut item: RefMut<RItem> = item.borrow_mut();
                    item.coords = Some(NPoint::new(x, y));
                }
                let mut row = self.get_row(rowidx).unwrap().borrow_mut();
                row.y = y;
                y += row.distance_y.round();
                rowidx += 1;
            }
            col.x = x;
            x += col.distance_x.round();
            //x += col.distance_x_after_allot;
        }
    }

    pub fn calculate_matrix_size(&mut self) {
        // matrix size
        let last_col: Ref<RCol> = self.cols.last().unwrap().borrow();
        let mut item_w: f32 = -1000.0;
        for item in &last_col.items {
            if let Some(item) = item {
                let item: Ref<RItem> = item.borrow();
                // for rect in item.rects.iter() {
                //     item_w = item_w.max(rect.0 + rect.2);
                // }

                // let nrects = item.nrects.as_ref().unwrap();

                for rect in item.nrects.as_ref().unwrap().iter() {
                    let rect: NRect = rect.borrow().0;
                    item_w = item_w.max(rect.0 + rect.2);
                }
            }
        }
        self.width = last_col.x + item_w;

        let last_row: Ref<RRow> = self.rows.last().unwrap().borrow();
        let mut item_h: f32 = -1000.0;
        for item in &last_row.items {
            if let Some(item) = item {
                let item: Ref<RItem> = item.borrow();
                // for rect in item.rects.iter() {
                //     item_h = item_h.max(rect.1 + rect.3);
                // }

                for rect in item.nrects.as_ref().unwrap().iter() {
                    let rect: NRect = rect.borrow().0;
                    item_h = item_h.max(rect.1 + rect.3);
                }
            }
        }
        self.height = last_row.y + item_h;
    }

    pub fn add_vertical_space(&self, add_space: f32) {
        if add_space <= 1.0 {
            return;
        }

        let current_height = self.height;

        let sum_distance_y = self.rows.iter().fold(0.0, |acc, row| {
            let row = row.borrow();
            acc + row.distance_y
        });

        for row in self.rows.iter() {
            let mut row = row.borrow_mut();
            let factor = row.distance_y / sum_distance_y;
            row.distance_y += add_space * factor;
            // println!("row.distance_y:{} {}", row.distance_y, factor);
        }
    }

    pub fn add_horizontal_space(&self, add_space: f32) {
        if add_space <= 1.0 {
            return;
        }

        let mut sum_allotment_duration = 0.0;
        for col in self.cols.iter() {
            let col = col.borrow();
            if col.duration == 0 {
                continue;
            };
            sum_allotment_duration += col.alloted_duration;
        }

        let mut current_add = add_space;
        let mut loopcount = 0;

        while current_add > 0.5 && loopcount < 5 {
            let current_factor = current_add / sum_allotment_duration as f32;
            for col in self.cols.iter() {
                let mut col = col.borrow_mut();
                if col.duration == 0 {
                    continue;
                };

                let mut increase = current_factor * col.alloted_duration;

                if col.overlap_overshoot > 0.0 {
                    if increase > col.overlap_overshoot {
                        let new_increase = increase - col.overlap_overshoot;
                        current_add = (current_add - new_increase).max(0.0);
                        col.distance_x = (col.distance_x + new_increase).max(0.0);
                        col.overlap_overshoot = 0.0;
                    } else {
                        col.overlap_overshoot = col.overlap_overshoot - increase;
                    };
                } else {
                    current_add = (current_add - increase).max(0.0);
                    col.distance_x = (col.distance_x + increase).max(0.0);
                };
            }
            loopcount += 1;
        }
        println!("add_horizontal_count passes:{}", loopcount);
    }

    pub fn calculate_beamgroups(&self) {
        for row in self.rows.iter() {
            let row = row.borrow();

            let mut note_idx_in_beamgroup = 0;
            // let mut note_steminfos = vec![];

            let mut note2_idx_in_beamgroup = 0;
            // let mut note2_steminfos = vec![];

            for (itemidx, item) in row.items.iter().enumerate() {
                if item.is_none() {
                    continue;
                }

                let mut item: RefMut<RItem> = item.as_ref().unwrap().borrow_mut();

                // NOTE 1 ==================================================================================

                match item.note_beamdata {
                    RItemBeam::None => {}
                    RItemBeam::Single(ref data) | RItemBeam::Start(ref data) | RItemBeam::End(ref data) => {
                        let note_x = if let Some(adjust_x) = data.adjustment_x {
                            match adjust_x {
                                ComplexXAdjustment::UpperRight(x) => x,
                                ComplexXAdjustment::LowerRight(_) => 0.0,
                            }
                        } else {
                            0.0
                        };

                        let mut adjust_x = note_x.clone();

                        match data.has_stem {
                            true => {
                                match item.note_beamdata {
                                    RItemBeam::Start(_) => note_idx_in_beamgroup = 0,
                                    _ => {}
                                };

                                match data.direction {
                                    DirUD::Up => {
                                        adjust_x += data.head_width - STEM_WIDTH;
                                    }
                                    _ => {}
                                }

                                let (y, y2) = match data.direction {
                                    DirUD::Up => ((data.tip_level - STEM_LENGTH) * SPACE_HALF, data.bottom_level as f32 * SPACE_HALF),
                                    DirUD::Down => (data.top_level as f32 * SPACE_HALF, (data.tip_level + STEM_LENGTH) as f32 * SPACE_HALF),
                                };
                                let h = y2 - y;

                                let rect = NRect::new(adjust_x, y, STEM_WIDTH, h);
                                // store stem coordinates for use in articulation etc
                                item.note_steminfo = StemInfo::FullInfo(note_x, y, data.head_width, h);
                                // note_steminfos.clear();
                                // note_steminfos.push(item.note_steminfo.clone());

                                // spacer for stem
                                let nrect = NRectExt::new(rect, NRectType::Spacer("stem upper".to_string()));
                                let mut nrects = item.nrects.as_mut().unwrap();
                                nrects.push(Rc::new(RefCell::new(nrect)));

                                match item.note_beamdata {
                                    RItemBeam::Start(ref data) => {
                                        let y = match data.direction {
                                            DirUD::Up => y,
                                            DirUD::Down => y2,
                                        };
                                        let rect = NRect::new(0.0, y - SPACE_HALF, SPACE * 2.0, SPACE);
                                        let nrect = NRectExt::new(rect, NRectType::Spacer("stem upper".to_string()));
                                        let mut nrects = item.nrects.as_mut().unwrap();
                                        nrects.push(Rc::new(RefCell::new(nrect)));
                                    }

                                    RItemBeam::End(ref data) => {
                                        let y = match data.direction {
                                            DirUD::Up => y,
                                            DirUD::Down => y2,
                                        };
                                        let rect = NRect::new(0.0, y - SPACE_HALF, data.head_width, SPACE);
                                        let nrect = NRectExt::new(rect, NRectType::Spacer("stem upper".to_string()));
                                        let mut nrects = item.nrects.as_mut().unwrap();
                                        nrects.push(Rc::new(RefCell::new(nrect)));
                                    }
                                    _ => {}
                                }
                            }
                            false => {
                                let y = data.top_level as f32 * SPACE_HALF;
                                let y2 = data.bottom_level as f32 * SPACE_HALF;
                                let h = y2 - y;
                                item.note_steminfo = StemInfo::FullInfo(note_x, y, data.head_width, h);
                            }
                        }
                    }
                    RItemBeam::Middle(ref data) => {
                        note_idx_in_beamgroup += 1;
                        let note_x = if let Some(adjust_x) = data.adjustment_x {
                            match adjust_x {
                                ComplexXAdjustment::UpperRight(x) => x,
                                ComplexXAdjustment::LowerRight(_) => 0.0,
                            }
                        } else {
                            0.0
                        };
                        let mut adjust_x = note_x.clone();
                        item.note_steminfo = StemInfo::BeamMiddle(note_idx_in_beamgroup, note_x, 0.0, data.head_width);
                    } // RItemBeam::End(ref data) => {
                }

                // NOTE 2 ==================================================================================

                match item.note2_beamdata {
                    RItemBeam::None => {}
                    RItemBeam::Single(ref data) | RItemBeam::Start(ref data) | RItemBeam::End(ref data) => {
                        let note_x = if let Some(adjust_x) = data.adjustment_x {
                            match adjust_x {
                                ComplexXAdjustment::UpperRight(_) => 0.0,
                                ComplexXAdjustment::LowerRight(x) => x,
                            }
                        } else {
                            0.0
                        };
                        let mut adjust_x = note_x.clone();

                        match data.has_stem {
                            true => {
                                match item.note2_beamdata {
                                    RItemBeam::Start(_) => note2_idx_in_beamgroup = 0,
                                    _ => {}
                                };

                                match data.direction {
                                    DirUD::Up => {
                                        adjust_x += data.head_width - STEM_WIDTH;
                                    }
                                    _ => {}
                                }

                                let (y, y2) = match data.direction {
                                    DirUD::Up => ((data.tip_level - STEM_LENGTH) * SPACE_HALF, data.bottom_level as f32 * SPACE_HALF),
                                    DirUD::Down => (data.top_level as f32 * SPACE_HALF, (data.tip_level + STEM_LENGTH) as f32 * SPACE_HALF),
                                };
                                // item.note2_beam_xyy2 = Some((adjust_x, y, y2));
                                let h = y2 - y;

                                let rect = NRect::new(adjust_x, y, STEM_WIDTH, h);
                                // store stem coordinates for use in articulation etc
                                item.note2_steminfo = StemInfo::FullInfo(note_x, y, data.head_width, h);

                                // spacer for stem
                                let nrect = NRectExt::new(rect, NRectType::Spacer("stem lower".to_string()));
                                let mut nrects = item.nrects.as_mut().unwrap();
                                nrects.push(Rc::new(RefCell::new(nrect)));

                                // spacer for stem tips
                                match item.note2_beamdata {
                                    RItemBeam::Start(ref data) => {
                                        let y = match data.direction {
                                            DirUD::Up => y,
                                            DirUD::Down => y2,
                                        };
                                        let rect = NRect::new(0.0, y - SPACE_HALF, SPACE * 2.0, SPACE);

                                        let nrect = NRectExt::new(rect, NRectType::Spacer("stem lower".to_string()));
                                        let mut nrects = item.nrects.as_mut().unwrap();
                                        nrects.push(Rc::new(RefCell::new(nrect)));
                                    }

                                    RItemBeam::End(ref data) => {
                                        let y = match data.direction {
                                            DirUD::Up => y,
                                            DirUD::Down => y2,
                                        };
                                        let rect = NRect::new(0.0, y - SPACE_HALF, data.head_width, SPACE);
                                        let nrect = NRectExt::new(rect, NRectType::Spacer("stem lower".to_string()));
                                        let mut nrects = item.nrects.as_mut().unwrap();
                                        nrects.push(Rc::new(RefCell::new(nrect)));
                                    }
                                    _ => {}
                                }
                            }
                            false => {
                                let y = data.top_level as f32 * SPACE_HALF;
                                let y2 = data.bottom_level as f32 * SPACE_HALF;
                                let h = y2 - y;
                                item.note2_steminfo = StemInfo::FullInfo(note_x, y, data.head_width, h);
                            }
                        }
                    }

                    RItemBeam::Middle(ref data) => {
                        note2_idx_in_beamgroup += 1;
                        let note_x = if let Some(adjust_x) = data.adjustment_x {
                            match adjust_x {
                                ComplexXAdjustment::UpperRight(x) => x,
                                ComplexXAdjustment::LowerRight(_) => 0.0,
                            }
                        } else {
                            0.0
                        };
                        let mut adjust_x = note_x.clone();
                        item.note2_steminfo = StemInfo::BeamMiddle(note2_idx_in_beamgroup, note_x, 0.0, data.head_width);
                    } // RItemBeam::End(ref data) => {
                }

                match item.note_beamdata {
                    RItemBeam::Single(ref data) => {
                        if let Some(nrect) = add_flag(data) {
                            let mut nrects = item.nrects.as_mut().unwrap();
                            nrects.push(Rc::new(RefCell::new(nrect)));
                        }
                    }
                    _ => {}
                }
                match item.note2_beamdata {
                    RItemBeam::Single(ref data) => {
                        if let Some(nrect) = add_flag(data) {
                            let mut nrects = item.nrects.as_mut().unwrap();
                            nrects.push(Rc::new(RefCell::new(nrect)));
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn calculate_attachment_points(&self, item2note: &BTreeMap<usize, Rc<RefCell<Note>>>) {
        const DRAW_POINTS: bool = false;
        for row in self.rows.iter() {
            let row = row.borrow();

            let mut note_steminfos = vec![];
            let mut note2_steminfos = vec![];

            for (itemidx, item) in row.items.iter().enumerate() {
                if item.is_none() {
                    continue;
                }
                let mut item = item.as_ref().unwrap().borrow_mut();

                // note1 ===============================================================================
                match item.note_beamdata {
                    RItemBeam::Single(ref data) => {
                        if let Some(nid) = item.note_id {
                            let stem_info = &item.note_steminfo.clone();
                            let apoint_outer = do_attachmentpoint_articulation_outer(&nid, stem_info, &item2note);
                            let apoint_inner = do_attachmentpoint_articulation_inner(&nid, stem_info, &item2note);
                            let fpoint_outer = do_attachmentpoint_slurfrom_outer(&nid, stem_info, &item2note, false);
                            let fpoint_inner = do_attachmentpoint_slurfrom_inner(&nid, stem_info, &item2note, false);
                            let tpoint_outer = do_attachmentpoint_slurto_outer(&nid, stem_info, &item2note, false);
                            let tpoint_inner = do_attachmentpoint_slurto_inner(&nid, stem_info, &item2note, false);

                            if DRAW_POINTS {
                                for point in [apoint_outer, apoint_inner, fpoint_outer, fpoint_inner, tpoint_outer, tpoint_inner] {
                                    item.nrects
                                        .as_mut()
                                        .unwrap()
                                        .push(Rc::new(RefCell::new(NRectExt::new(point.to_rect(3.0), NRectType::Dev(true, "Orange".to_string())))));
                                }
                            }
                        }
                    }

                    RItemBeam::Start(ref data) => {
                        if let Some(nid) = item.note_id {
                            let stem_info = &item.note_steminfo.clone();
                            note_steminfos = vec![];
                            note_steminfos.push(stem_info.clone());
                            let apoint_outer = do_attachmentpoint_articulation_outer(&nid, stem_info, &item2note);
                            let apoint_inner = do_attachmentpoint_articulation_inner(&nid, stem_info, &item2note);
                            let fpoint_outer = do_attachmentpoint_slurfrom_outer(&nid, stem_info, &item2note, true);
                            let fpoint_inner = do_attachmentpoint_slurfrom_inner(&nid, stem_info, &item2note, true);
                            let tpoint_outer = do_attachmentpoint_slurto_outer(&nid, stem_info, &item2note, false);
                            let tpoint_inner = do_attachmentpoint_slurto_inner(&nid, stem_info, &item2note, false);

                            if DRAW_POINTS {
                                for point in [apoint_outer, apoint_inner, fpoint_outer, fpoint_inner, tpoint_outer, tpoint_inner] {
                                    item.nrects
                                        .as_mut()
                                        .unwrap()
                                        .push(Rc::new(RefCell::new(NRectExt::new(point.to_rect(3.0), NRectType::Dev(true, "Orange".to_string())))));
                                }
                            }
                        }
                    }

                    RItemBeam::Middle(ref data) => {
                        if let Some(nid) = item.note_id {
                            let stem_info = &item.note_steminfo.clone();
                            note_steminfos.push(stem_info.clone());
                        }
                    }

                    RItemBeam::End(ref data) => {
                        if let Some(nid) = item.note_id {
                            //-------------------------------------------------------------------------------
                            // calculate for middle items
                            let note = item2note.get(&nid).expect(format!("could not get note id {} from item2note", nid).as_str()).borrow();
                            let beamgroup = note.beamgroup.as_ref().unwrap().borrow();
                            let stem_info = &item.note_steminfo.clone();
                            note_steminfos.push(stem_info.clone());

                            let (first_x, first_y, first_headw, first_h) = match note_steminfos[0] {
                                StemInfo::FullInfo(x, y, hw, h) => (x, y, hw, h),
                                StemInfo::BeamMiddle(_, _, _, _) => todo!(),
                                StemInfo::None => todo!(),
                            };
                            let (last_x, last_y, last_headw, last_h) = match stem_info {
                                StemInfo::FullInfo(x, y, hw, h) => (x, y, hw, h),
                                StemInfo::BeamMiddle(_, _, _, _) => todo!(),
                                StemInfo::None => todo!(),
                            };

                            for beaminfo in &note_steminfos {
                                match beaminfo {
                                    StemInfo::BeamMiddle(idx, stem_x, stem_y, head_width) => {
                                        let current = beamgroup.note_durations.iter().take(*idx).sum::<usize>();
                                        let sum = beamgroup.note_durations.iter().take(beamgroup.note_durations.len() - 1).sum::<usize>();
                                        let fraction = current as f32 / sum as f32;
                                        let middle_nid = beamgroup.notes[*idx].borrow().id;
                                        let middle_itemidx = itemidx - beamgroup.notes.len() + *idx + 1;
                                        let mut middle_item = row.items[middle_itemidx].as_ref().unwrap().borrow_mut();
                                        let middle_y = first_y + (last_y - first_y) * fraction;
                                        let middle_h = first_h + (last_h - first_h) * fraction;

                                        let middle_note = item2note.get(&middle_nid).expect(format!("could not get note id {} from item2note", middle_nid).as_str()).borrow();
                                        let inner_level = match beamgroup.direction.unwrap() {
                                            DirUD::Up => middle_note.bottom_level() as f32 * SPACE_HALF,
                                            DirUD::Down => middle_note.top_level() as f32 * SPACE_HALF,
                                        };

                                        let mut middle_stem_info_outer = StemInfo::FullInfo(*stem_x, middle_y, *head_width, middle_h);
                                        let mut middle_stem_info_inner = StemInfo::FullInfo(*stem_x, inner_level, *head_width, 0.0);

                                        let apoint_outer = do_attachmentpoint_articulation_outer(&middle_nid, &middle_stem_info_outer, &item2note);
                                        let apoint_inner = do_attachmentpoint_articulation_inner(&middle_nid, &middle_stem_info_inner, &item2note);
                                        let fpoint_outer = do_attachmentpoint_slurfrom_outer(&middle_nid, &middle_stem_info_outer, &item2note, true);
                                        let fpoint_inner = do_attachmentpoint_slurfrom_inner(&middle_nid, &middle_stem_info_inner, &item2note, true);
                                        let tpoint_outer = do_attachmentpoint_slurto_outer(&middle_nid, &middle_stem_info_outer, &item2note, true);
                                        let tpoint_inner = do_attachmentpoint_slurto_inner(&middle_nid, &middle_stem_info_inner, &item2note, true);

                                        if DRAW_POINTS {
                                            for point in [apoint_outer, apoint_inner, fpoint_outer, fpoint_inner, tpoint_outer, tpoint_inner] {
                                                middle_item
                                                    .nrects
                                                    .as_mut()
                                                    .unwrap()
                                                    .push(Rc::new(RefCell::new(NRectExt::new(point.to_rect(3.0), NRectType::Dev(true, "Orange".to_string())))));
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            //-------------------------------------------------------------------------------
                            let apoint_outer = do_attachmentpoint_articulation_outer(&nid, stem_info, &item2note);
                            let apoint_inner = do_attachmentpoint_articulation_inner(&nid, stem_info, &item2note);
                            let fpoint_outer = do_attachmentpoint_slurfrom_outer(&nid, stem_info, &item2note, false);
                            let fpoint_inner = do_attachmentpoint_slurfrom_inner(&nid, stem_info, &item2note, false);
                            let tpoint_outer = do_attachmentpoint_slurto_outer(&nid, stem_info, &item2note, true);
                            let tpoint_inner = do_attachmentpoint_slurto_inner(&nid, stem_info, &item2note, true);

                            if DRAW_POINTS {
                                for point in [apoint_outer, apoint_inner, fpoint_outer, fpoint_inner, tpoint_outer, tpoint_inner] {
                                    item.nrects
                                        .as_mut()
                                        .unwrap()
                                        .push(Rc::new(RefCell::new(NRectExt::new(point.to_rect(3.0), NRectType::Dev(true, "Orange".to_string())))));
                                }
                            }
                        }
                    }
                    RItemBeam::None => {}
                }

                // note2 ===============================================================================
                match item.note2_beamdata {
                    RItemBeam::Single(ref data) => {
                        if let Some(nid) = item.note2_id {
                            let stem_info = &item.note2_steminfo.clone();

                            let apoint_outer = do_attachmentpoint_articulation_outer(&nid, stem_info, &item2note);
                            let apoint_inner = do_attachmentpoint_articulation_inner(&nid, stem_info, &item2note);
                            let fpoint_outer = do_attachmentpoint_slurfrom_outer(&nid, stem_info, &item2note, false);
                            let fpoint_inner = do_attachmentpoint_slurfrom_inner(&nid, stem_info, &item2note, false);
                            let tpoint_outer = do_attachmentpoint_slurto_outer(&nid, stem_info, &item2note, false);
                            let tpoint_inner = do_attachmentpoint_slurto_inner(&nid, stem_info, &item2note, false);
                            if DRAW_POINTS {
                                for point in [apoint_outer, apoint_inner, fpoint_outer, fpoint_inner, tpoint_outer, tpoint_inner] {
                                    item.nrects
                                        .as_mut()
                                        .unwrap()
                                        .push(Rc::new(RefCell::new(NRectExt::new(point.to_rect(3.0), NRectType::Dev(true, "Orange".to_string())))));
                                }
                            }
                        }
                    }

                    RItemBeam::Start(ref data) => {
                        if let Some(nid) = item.note2_id {
                            let stem_info = &item.note2_steminfo.clone();
                            note2_steminfos = vec![];
                            note2_steminfos.push(stem_info.clone());
                            let apoint_outer = do_attachmentpoint_articulation_outer(&nid, stem_info, &item2note);
                            let apoint_inner = do_attachmentpoint_articulation_inner(&nid, stem_info, &item2note);
                            let fpoint_outer = do_attachmentpoint_slurfrom_outer(&nid, stem_info, &item2note, true);
                            let fpoint_inner = do_attachmentpoint_slurfrom_inner(&nid, stem_info, &item2note, true);
                            let tpoint_outer = do_attachmentpoint_slurto_outer(&nid, stem_info, &item2note, false);
                            let tpoint_inner = do_attachmentpoint_slurto_inner(&nid, stem_info, &item2note, false);
                            if DRAW_POINTS {
                                for point in [apoint_outer, apoint_inner, fpoint_outer, fpoint_inner, tpoint_outer, tpoint_inner] {
                                    item.nrects
                                        .as_mut()
                                        .unwrap()
                                        .push(Rc::new(RefCell::new(NRectExt::new(point.to_rect(3.0), NRectType::Dev(true, "Orange".to_string())))));
                                }
                            }
                        }
                    }

                    RItemBeam::Middle(ref data) => {
                        if let Some(nid) = item.note2_id {
                            let stem_info = &item.note2_steminfo.clone();
                            note2_steminfos.push(stem_info.clone());
                        }
                    }

                    RItemBeam::End(ref data) => {
                        if let Some(nid) = item.note2_id {
                            //-------------------------------------------------------------------------------
                            // calculate for middle items
                            let note = item2note.get(&nid).expect(format!("could not get note id {} from item2note", nid).as_str()).borrow();
                            let beamgroup = note.beamgroup.as_ref().unwrap().borrow();
                            let stem_info = &item.note2_steminfo.clone();
                            note2_steminfos.push(stem_info.clone());

                            let (first_x, first_y, first_headw, first_h) = match note2_steminfos[0] {
                                StemInfo::FullInfo(x, y, hw, h) => (x, y, hw, h),
                                StemInfo::BeamMiddle(_, _, _, _) => todo!(),
                                StemInfo::None => todo!(),
                            };
                            let (last_x, last_y, last_headw, last_h) = match stem_info {
                                StemInfo::FullInfo(x, y, hw, h) => (x, y, hw, h),
                                StemInfo::BeamMiddle(_, _, _, _) => todo!(),
                                StemInfo::None => todo!(),
                            };
                            for beaminfo in &note2_steminfos {
                                match beaminfo {
                                    StemInfo::BeamMiddle(idx, stem_x, stem_y, head_width) => {
                                        let current = beamgroup.note_durations.iter().take(*idx).sum::<usize>();
                                        let sum = beamgroup.note_durations.iter().take(beamgroup.note_durations.len() - 1).sum::<usize>();
                                        let fraction = current as f32 / sum as f32;
                                        let middle_nid = beamgroup.notes[*idx].borrow().id;
                                        let middle_itemidx = itemidx - beamgroup.notes.len() + *idx + 1;
                                        let mut middle_item = row.items[middle_itemidx].as_ref().unwrap().borrow_mut();
                                        //
                                        let middle_y = first_y + (last_y - first_y) * fraction;
                                        let middle_h = first_h + (last_h - first_h) * fraction;

                                        let middle_note = item2note.get(&middle_nid).expect(format!("could not get note id {} from item2note", middle_nid).as_str()).borrow();
                                        let inner_level = match beamgroup.direction.unwrap() {
                                            DirUD::Up => middle_note.bottom_level() as f32 * SPACE_HALF,
                                            DirUD::Down => middle_note.top_level() as f32 * SPACE_HALF,
                                        };

                                        let mut middle_stem_info_outer = StemInfo::FullInfo(*stem_x, middle_y, *head_width, middle_h);
                                        let mut middle_stem_info_inner = StemInfo::FullInfo(*stem_x, inner_level, *head_width, 0.0);

                                        let apoint_outer = do_attachmentpoint_articulation_outer(&middle_nid, &middle_stem_info_outer, &item2note);
                                        let apoint_inner = do_attachmentpoint_articulation_inner(&middle_nid, &middle_stem_info_inner, &item2note);
                                        let fpoint_outer = do_attachmentpoint_slurfrom_outer(&middle_nid, &middle_stem_info_outer, &item2note, true);
                                        let fpoint_inner = do_attachmentpoint_slurfrom_inner(&middle_nid, &middle_stem_info_inner, &item2note, true);
                                        let tpoint_outer = do_attachmentpoint_slurto_outer(&middle_nid, &middle_stem_info_outer, &item2note, true);
                                        let tpoint_inner = do_attachmentpoint_slurto_inner(&middle_nid, &middle_stem_info_inner, &item2note, true);
                                        if DRAW_POINTS {
                                            for point in [apoint_outer, apoint_inner, fpoint_outer, fpoint_inner, tpoint_outer, tpoint_inner] {
                                                middle_item
                                                    .nrects
                                                    .as_mut()
                                                    .unwrap()
                                                    .push(Rc::new(RefCell::new(NRectExt::new(point.to_rect(3.0), NRectType::Dev(true, "Orange".to_string())))));
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            //-------------------------------------------------------------------------------
                            let stem_info = &item.note2_steminfo.clone();

                            let apoint_outer = do_attachmentpoint_articulation_outer(&nid, stem_info, &item2note);
                            let apoint_inner = do_attachmentpoint_articulation_inner(&nid, stem_info, &item2note);
                            let tpoint_outer = do_attachmentpoint_slurto_outer(&nid, stem_info, &item2note, true);
                            let tpoint_inner = do_attachmentpoint_slurto_inner(&nid, stem_info, &item2note, true);
                            let fpoint_outer = do_attachmentpoint_slurfrom_outer(&nid, stem_info, &item2note, false);
                            let fpoint_inner = do_attachmentpoint_slurfrom_inner(&nid, stem_info, &item2note, false);
                            if DRAW_POINTS {
                                for point in [apoint_outer, apoint_inner, fpoint_outer, fpoint_inner, tpoint_outer, tpoint_inner] {
                                    item.nrects
                                        .as_mut()
                                        .unwrap()
                                        .push(Rc::new(RefCell::new(NRectExt::new(point.to_rect(3.0), NRectType::Dev(true, "Orange".to_string())))));
                                }
                            }
                        }
                    }
                    RItemBeam::None => {}
                }
            }
        }
    }
}

fn do_attachmentpoint_articulation_inner(nid: &usize, stem_info: &StemInfo, item2note: &BTreeMap<usize, Rc<RefCell<Note>>>) -> NPoint {
    let note = item2note.get(nid).expect(format!("could not get note id {} from item2note", nid).as_str()).borrow();
    match stem_info {
        StemInfo::FullInfo(stem_x, stem_y, stem_w, stem_h) => {
            if let Some(direction) = note.beamgroup.as_ref().unwrap().borrow().direction {
                match direction {
                    DirUD::Up => NPoint::new(stem_x + (stem_w / 2.0), stem_y + stem_h + SPACE),
                    DirUD::Down => NPoint::new(stem_x + (stem_w / 2.0), stem_y - SPACE),
                }
            } else {
                todo!()
            }
        }
        StemInfo::BeamMiddle(idx, _, _, _) => todo!(),
        StemInfo::None => todo!(),
    }
}

fn do_attachmentpoint_articulation_outer(nid: &usize, stem_info: &StemInfo, item2note: &BTreeMap<usize, Rc<RefCell<Note>>>) -> NPoint {
    let note = item2note.get(nid).expect(format!("could not get note id {} from item2note", nid).as_str()).borrow();
    let extra_y = if note.has_stem() { SPACE_HALF } else { SPACE };
    match stem_info {
        StemInfo::FullInfo(stem_x, stem_y, stem_w, stem_h) => {
            if let Some(direction) = note.beamgroup.as_ref().unwrap().borrow().direction {
                match direction {
                    DirUD::Up => NPoint::new(stem_x + (stem_w / 2.0), stem_y - extra_y),
                    DirUD::Down => NPoint::new(stem_x + (stem_w / 2.0), stem_y + stem_h + extra_y),
                }
            } else {
                todo!()
            }
        }
        StemInfo::BeamMiddle(idx, _, _, _) => todo!(),
        StemInfo::None => todo!(),
    }
}

fn do_attachmentpoint_slurfrom_outer(nid: &usize, stem_info: &StemInfo, item2note: &BTreeMap<usize, Rc<RefCell<Note>>>, beamed: bool) -> NPoint {
    let note = item2note.get(nid).expect(format!("could not get note id {} from item2note", nid).as_str()).borrow();
    let direction = if let Some(direction) = note.beamgroup.as_ref().unwrap().borrow().direction {
        direction
    } else {
        todo!()
    };

    match stem_info {
        StemInfo::FullInfo(stem_x, stem_y, stem_w, stem_h) => {
            let note_w = match (note.has_stem(), beamed, direction) {
                (true, true, DirUD::Up) => *stem_w,
                (true, true, DirUD::Down) => 0.,
                (true, false, DirUD::Up) => stem_w + SPACE_HALF,
                (true, false, DirUD::Down) => stem_w / 2.0,
                (false, _, _) => *stem_w,
            };

            let extra_y = match (note.has_stem(), beamed) {
                (true, true) => -SPACE_HALF,
                (true, false) => SPACE,
                (false, _) => -SPACE,
            };

            match direction {
                DirUD::Up => NPoint::new(stem_x + note_w, stem_y + extra_y),
                DirUD::Down => NPoint::new(stem_x + note_w, stem_y + stem_h - extra_y),
            }
        }
        StemInfo::BeamMiddle(idx, _, _, _) => todo!(),
        StemInfo::None => todo!(),
    }
}

fn do_attachmentpoint_slurfrom_inner(nid: &usize, stem_info: &StemInfo, item2note: &BTreeMap<usize, Rc<RefCell<Note>>>, beamed: bool) -> NPoint {
    let note = item2note.get(nid).expect(format!("could not get note id {} from item2note", nid).as_str()).borrow();
    match stem_info {
        StemInfo::FullInfo(stem_x, stem_y, stem_w, stem_h) => {
            if let Some(direction) = note.beamgroup.as_ref().unwrap().borrow().direction {
                match direction {
                    DirUD::Up => NPoint::new(stem_x + stem_w, stem_y + stem_h + SPACE),
                    DirUD::Down => NPoint::new(stem_x + stem_w, stem_y - SPACE),
                }
            } else {
                todo!()
            }
        }
        StemInfo::BeamMiddle(idx, _, _, _) => todo!(),
        StemInfo::None => todo!(),
    }
}

fn do_attachmentpoint_slurto_outer(nid: &usize, stem_info: &StemInfo, item2note: &BTreeMap<usize, Rc<RefCell<Note>>>, beamed: bool) -> NPoint {
    let note = item2note.get(nid).expect(format!("could not get note id {} from item2note", nid).as_str()).borrow();
    let direction = if let Some(direction) = note.beamgroup.as_ref().unwrap().borrow().direction {
        direction
    } else {
        todo!()
    };
    match stem_info {
        StemInfo::FullInfo(stem_x, stem_y, stem_w, stem_h) => {
            let note_w = match note.has_stem() {
                true => match direction {
                    DirUD::Up => stem_w - SPACE_HALF,
                    DirUD::Down => -SPACE_HALF,
                },
                false => 0.,
            };

            let note_w = match (note.has_stem(), beamed, direction) {
                (true, true, DirUD::Up) => *stem_w,
                (true, true, DirUD::Down) => 0.,
                (true, false, DirUD::Up) => stem_w - SPACE_HALF,
                (true, false, DirUD::Down) => -SPACE_HALF,
                (false, _, _) => 0.,
            };

            let extra_y = match (note.has_stem(), beamed) {
                (true, true) => -SPACE_HALF,
                (true, false) => SPACE,
                (false, _) => -SPACE,
            };

            match direction {
                DirUD::Up => NPoint::new(stem_x + note_w, stem_y + extra_y),
                DirUD::Down => NPoint::new(stem_x + note_w, stem_y + stem_h - extra_y),
            }
        }
        StemInfo::BeamMiddle(idx, _, _, _) => todo!(),
        StemInfo::None => todo!(),
    }
}

fn do_attachmentpoint_slurto_inner(nid: &usize, stem_info: &StemInfo, item2note: &BTreeMap<usize, Rc<RefCell<Note>>>, beamed: bool) -> NPoint {
    let note = item2note.get(nid).expect(format!("could not get note id {} from item2note", nid).as_str()).borrow();
    match stem_info {
        StemInfo::FullInfo(stem_x, stem_y, stem_w, stem_h) => {
            if let Some(direction) = note.beamgroup.as_ref().unwrap().borrow().direction {
                match direction {
                    DirUD::Up => NPoint::new(*stem_x, stem_y + stem_h + SPACE),
                    DirUD::Down => NPoint::new(*stem_x, stem_y - SPACE),
                }
            } else {
                todo!()
            }
        }
        StemInfo::BeamMiddle(idx, _, _, _) => todo!(),
        StemInfo::None => todo!(),
    }
}

#[derive(Debug, PartialEq)]
enum ArticulationAttachment {
    Outer,
    Inner,
    None,
}
