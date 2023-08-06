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
        for item in firstcol.borrow().items.iter() {
            if item.is_none() {
                panic!("firstcol has None - shouldn't have!");
            }
        }

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
                //println!("==========================");
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
            row.distance_y = row.distance_y.max(overlap);
            rowidx += 1;
        }
    }

    pub fn calculate_col_measurements(&mut self) {
        let mut x = 0.0;
        for col in &self.cols {
            let mut col = col.borrow_mut();
            // let mut y = 0.0;
            // let mut rowidx = 0;
            // for item in &col.items {
            //     if let Some(item) = item {
            //         let mut item: RefMut<RItem> = item.borrow_mut();
            //         item.coords = Some(NPoint::new(x, y));
            //     }
            //     // let mut row = self.get_row(rowidx).unwrap().borrow_mut();
            //     // row.y = y;
            //     // y += row.distance_y.round();
            //     // rowidx += 1;
            // }
            col.x = x;
            x += col.distance_x.round();
            //x += col.distance_x_after_allot;
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
            for item in row.items.iter() {
                if item.is_none() {
                    continue;
                }

                let mut item: RefMut<RItem> = item.as_ref().unwrap().borrow_mut();
                // let mut item: Ref<RItem> = item.as_ref().unwrap().borrow();
                // let coords = item.coords.expect("RItem coords should always be calculated!");

                match item.note_beam {
                    RItemBeam::None => {
                        // Not a note
                    }
                    RItemBeam::Single(ref data) | RItemBeam::Start(ref data) | RItemBeam::End(ref data) => {
                        println!("Single or Start or End");

                        let note_x = if let Some(adjust_x) = data.adjustment_x {
                            match adjust_x {
                                ComplexXAdjustment::UpperRight(x) => x,
                                ComplexXAdjustment::LowerRight(_) => 0.0,
                            }
                        } else {
                            0.0
                        };

                        let mut adjust_x = note_x.clone();

                        if !data.has_stem {
                            // notes with no stem
                            let y = data.top_level as f32 - SPACE_HALF;
                            let y2 = data.bottom_level as f32 + SPACE_HALF;
                            let h = y2 - y;
                            item.note_beam_rect = Some((note_x, y, data.head_width, h));

                            continue;
                        }

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

                        // item.note_beam_xyy2 = Some((adjust_x, y, y2));
                        let h = y2 - y;

                        let rect = NRect::new(adjust_x, y, STEM_WIDTH, h);
                        // store stem coordinates for use in articulation etc
                        item.note_beam_rect = Some((note_x, y, data.head_width, h));

                        // spacer for stem
                        let nrect = NRectExt::new(rect, NRectType::Spacer("stem upper".to_string()));
                        let mut nrects = item.nrects.as_mut().unwrap();
                        nrects.push(Rc::new(RefCell::new(nrect)));

                        match item.note_beam {
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
                    RItemBeam::Middle(ref data) => {
                        println!("Middle");
                    } // RItemBeam::End(ref data) => {
                }

                match item.note2_beam {
                    RItemBeam::None => {}
                    RItemBeam::Single(ref data) | RItemBeam::Start(ref data) | RItemBeam::End(ref data) => {
                        // println!("SINGLE single upper");

                        let note_x = if let Some(adjust_x) = data.adjustment_x {
                            match adjust_x {
                                ComplexXAdjustment::UpperRight(_) => 0.0,
                                ComplexXAdjustment::LowerRight(x) => x,
                            }
                        } else {
                            0.0
                        };
                        let mut adjust_x = note_x.clone();

                        if !data.has_stem {
                            // notes with no stem
                            let y = data.top_level as f32 - SPACE_HALF;
                            let y2 = data.bottom_level as f32 + SPACE_HALF;
                            let h = y2 - y;
                            item.note_beam_rect = Some((note_x, y, data.head_width, h));

                            continue;
                        }

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
                        item.note2_beam_rect = Some((note_x, y, data.head_width, h));

                        // spacer for stem
                        let nrect = NRectExt::new(rect, NRectType::Spacer("stem lower".to_string()));
                        let mut nrects = item.nrects.as_mut().unwrap();
                        nrects.push(Rc::new(RefCell::new(nrect)));

                        // spacer for stem tips
                        match item.note2_beam {
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

                    RItemBeam::Middle(ref data) => {
                        // println!("MIDDLE  upper");
                    } // RItemBeam::End(ref data) => {
                }

                match item.note_beam {
                    RItemBeam::Single(ref data) => {
                        if let Some(nrect) = add_flag(data) {
                            let mut nrects = item.nrects.as_mut().unwrap();
                            nrects.push(Rc::new(RefCell::new(nrect)));
                        }
                    }
                    _ => {}
                }
                match item.note2_beam {
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

    pub fn calculate_articulations(&self, item2note: BTreeMap<usize, Rc<RefCell<Note>>>) {
        for row in self.rows.iter() {
            let row = row.borrow();
            for item in row.items.iter() {
                if item.is_none() {
                    continue;
                }
                let mut item = item.as_ref().unwrap().borrow_mut();

                // let rect = NRect::new(0.0, 0.0, 10.0, 10.0);
                // let nrect = NRectExt::new(rect, NRectType::Dev(true, "Red".to_string()));
                // nrects.push(Rc::new(RefCell::new(nrect)));
                // item.nrects.as_ref().unwrap().push(Rc::new(RefCell::new(nrect)));
                // item.nrects.unwrap().push(Rc::new(RefCell::new(nrect)));
                // item.nrects.as_mut().unwrap().push(Rc::new(RefCell::new(nrect)));

                match item.note_beam {
                    RItemBeam::Single(ref data) => {
                        if let Some(nid) = item.note_id {
                            let rect = &item.note_beam_rect.unwrap();
                            item.nrects.as_mut().unwrap().extend(do_articulations(&nid, &rect, &item2note));
                        } else {
                            println!("Hoho1");
                        }

                        if let Some(nid) = item.note2_id {
                            let rect = &item.note2_beam_rect.unwrap();
                            item.nrects.as_mut().unwrap().extend(do_articulations(&nid, &rect, &item2note));
                        } else {
                            println!("Hoho2");
                        }
                    }
                    RItemBeam::Start(ref data) => {
                        println!("Articulation Multi:Start");
                        if let Some(nid) = item.note_id {
                            let rect = &item.note_beam_rect.unwrap();
                            item.nrects.as_mut().unwrap().extend(do_articulations(&nid, &rect, &item2note));
                        }
                        if let Some(nid) = item.note2_id {
                            let rect = &item.note2_beam_rect.unwrap();
                            item.nrects.as_mut().unwrap().extend(do_articulations(&nid, &rect, &item2note));
                        }
                    }
                    RItemBeam::Middle(ref data) => {
                        println!("Articulation Multi:Middle");
                    }
                    RItemBeam::End(ref data) => {
                        if let Some(nid) = item.note_id {
                            let rect = &item.note_beam_rect.unwrap();
                            item.nrects.as_mut().unwrap().extend(do_articulations(&nid, &rect, &item2note));
                        }
                        if let Some(nid) = item.note2_id {
                            let rect = &item.note2_beam_rect.unwrap();
                            item.nrects.as_mut().unwrap().extend(do_articulations(&nid, &rect, &item2note));
                        }
                    }
                    RItemBeam::None => {}
                }
            }
        }
    }
}

fn do_articulations(nid: &usize, stem_info: &StemInfo, item2note: &BTreeMap<usize, Rc<RefCell<Note>>>) -> Vec<Rc<RefCell<NRectExt>>> {
    let mut nrects = Vec::new();

    // let rect = item.note_beam_rect.expect("note_beam_rect should be calculated by now!");
    let note = item2note.get(nid).expect(format!("could not get note id {} from item2note", nid).as_str()).borrow();

    // create rects here...
    if let Some(direction) = note.direction {
        match direction {
            DirUD::Up => {
                let rect = NRect::new(stem_info.0 + (stem_info.2 / 2.0) - 5.0, stem_info.1 - 5.0 - SPACE_HALF, 10.0, 10.0);
                let nrect = Rc::new(RefCell::new(NRectExt::new(rect, NRectType::Dev(true, "Red".to_string()))));
                nrects.push(nrect);
            }
            DirUD::Down => {
                println!("Articulation :Down");
                dbg!(stem_info);
                let rect = NRect::new(stem_info.0 + (stem_info.2 / 2.0) - 5.0, stem_info.1 + stem_info.3 - 5.0 + SPACE_HALF, 10.0, 10.0);
                let nrect = Rc::new(RefCell::new(NRectExt::new(rect, NRectType::Dev(true, "Red".to_string()))));
                nrects.push(nrect);
            }
        }
    };
    nrects
}

// fn deal_with_articulation3(nid: &usize, item: RefMut<'_, RItem>, item2note: &BTreeMap<usize, Rc<RefCell<Note>>>) {}

// fn deal_with_articulation2(nid: &usize, item: &RefMut<'_, RItem>, nrects: &[Rc<RefCell<NRectExt>>], item2note: &BTreeMap<usize, Rc<RefCell<Note>>>) {}

// fn deal_with_articulation(nid: &usize, item: Ref<RItem>, item2note: &BTreeMap<usize, Rc<RefCell<Note>>>) {
//     let rect = item.note_beam_rect.expect("note_beam_rect should be calculated by now!");
//     let note = item2note.get(nid).expect(format!("could not get note id {} from item2note", nid).as_str()).borrow();
//     // let mut nrects = item.nrects.unwrap();

//     if let Some(direction) = note.direction {
//         match direction {
//             DirUD::Up => {
//                 println!("Articulation :Up");
//                 let rect = NRect::new(-5.0 + rect.0 + (rect.2 / 2.0), rect.1 - SPACE_HALF, 10.0, 10.0);
//                 let nrect = NRectExt::new(rect, NRectType::Dev(true, "Red".to_string()));
//                 // nrects.push(nrect);
//             }
//             DirUD::Down => {
//                 println!("Articulation :Down");
//             }
//         }
//     }
// }
