//----------------------------------------------------------------

use std::cell::{Ref, RefMut};

use crate::{
    prelude::*, render::fonts::ebgaramond::GLYPH_HEIGHT, types::some_cloneables::SomeCloneablePairs,
};

use crate::prelude::NRect;

pub fn qitem(x: f32, w: f32, dur: Duration) -> Option<Rc<RefCell<RItem>>> {
    Some(Rc::new(RefCell::new(RItem::new(
        vec![NRect::new(x, 0.0, w, 10.0)],
        dur,
    ))))
}

pub fn xitem(x: f32, w: f32, h: f32, dur: Duration) -> Option<Rc<RefCell<RItem>>> {
    Some(Rc::new(RefCell::new(RItem::new(
        vec![NRect::new(x, 0.0, w, h)],
        dur,
    ))))
}

pub fn r10() -> Vec<NRect> {
    vec![NRect::new(0.0, 0.0, 10.0, 10.0)]
}

pub fn r20() -> Vec<NRect> {
    vec![NRect::new(0.0, 0.0, 10.0, 20.0)]
}

// trait Col {}

#[derive(Debug)]
pub struct RItem {
    pub rects: Vec<NRect>,
    pub duration: Duration,
    pub col_idx: usize,
    pub row_idx: usize,
    pub coords: Option<NPoint>,
}

impl RItem {
    pub fn new(rects: Vec<NRect>, dur: Duration) -> Self {
        Self {
            rects,
            duration: dur,
            col_idx: 0,
            row_idx: 0,
            coords: None,
        }
    }
}

#[derive(Debug)]
pub struct RCol {
    pub duration: Duration,
    pub items: Vec<Option<Rc<RefCell<RItem>>>>,
    pub distance_x: f32,
    pub x: f32,

    pub spacing_duration: f32,
    pub spacing_overlap: f32,
    pub overlap_overshoot: f32,
    pub alloted_duration: f32,
    // pub distance_x_after_allot: f32,
}

impl RCol {
    pub fn new(items: Vec<Option<Rc<RefCell<RItem>>>>, duration: Option<Duration>) -> Self {
        Self {
            items,
            duration: duration.unwrap_or(0),
            distance_x: 0.0,
            x: 0.0,

            spacing_duration: 0.0,
            spacing_overlap: 0.0,
            overlap_overshoot: 0.0,
            alloted_duration: 0.0,
            // distance_x_after_allot: 0.0,
        }
    }
}

#[derive(Debug)]
pub struct RRow {
    pub items: Vec<Option<Rc<RefCell<RItem>>>>,
    pub distance_y: f32,
    pub y: f32,
}
impl RRow {
    fn new(items: Vec<Option<Rc<RefCell<RItem>>>>, distance_y: f32) -> Self {
        Self {
            items,
            distance_y,
            y: 0.0,
        }
    }
}

#[derive(Debug)]
pub struct RMatrix {
    pub cols: Vec<Rc<RefCell<RCol>>>,
    pub rows: Vec<Rc<RefCell<RRow>>>,
    pub width: f32,
    pub height: f32,
}

impl RMatrix {
    pub fn new(colitems: Vec<Rc<RefCell<RCol>>>) -> Self {
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

            let pairs = SomeCloneablePairs {
                items: row.items.clone(),
            };
            for (left, left_idx, right, right_idx) in pairs.into_iter() {
                //println!("==========================");
                match [&left, &right] {
                    [Some(left), Some(right)] => {
                        let left = left.borrow_mut();
                        let right = right.borrow_mut();
                        let mut left_col = self.get_column(left.col_idx).unwrap().borrow_mut();
                        let mut right_col = self.get_column(right.col_idx).unwrap().borrow_mut();

                        // calculate spacings...
                        let overlap_spacing: f32 =
                            nrects_overlap_x(&left.rects, &right.rects).unwrap_or(0.0);

                        let spacing = if (right_idx - 1) != left_idx.unwrap() {
                            let mut prev_col = self.get_column(right.col_idx - 1).unwrap().borrow();
                            overlap_spacing - prev_col.distance_x
                        } else {
                            overlap_spacing
                        };
                        left_col.distance_x = left_col.distance_x.max(spacing);
                        //

                        left_col.spacing_overlap = left_col.spacing_overlap.max(overlap_spacing);
                        left_col.overlap_overshoot = left_col
                            .overlap_overshoot
                            .max(left_col.spacing_overlap - left_col.spacing_duration);
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
                    for rect in item.rects.iter() {
                        let rect = rect.move_rect(colx, 0.0);
                        itemrects.push(rect);
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

    pub fn calculate_measurements(&mut self) {
        // cols, rows, items
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

        // matrix size
        let last_col: Ref<RCol> = self.cols.last().unwrap().borrow();
        let mut item_w: f32 = -1000.0;
        for item in &last_col.items {
            if let Some(item) = item {
                let item: Ref<RItem> = item.borrow();
                for rect in item.rects.iter() {
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
                for rect in item.rects.iter() {
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
}
