pub fn matrix_test1() -> RMatrix {
    let col0 = RCol::new(
        vec![
            Some(Rc::new(RefCell::new(RItem::new(r20(), 0)))),
            Some(Rc::new(RefCell::new(RItem::new(r20(), 0)))),
            Some(Rc::new(RefCell::new(RItem::new(r20(), 0)))),
            //
        ],
        None,
    );
    let col1 = RCol::new(
        vec![
            Some(Rc::new(RefCell::new(RItem::new(r10(), NV1)))),
            Some(Rc::new(RefCell::new(RItem::new(r10(), NV4)))),
            Some(Rc::new(RefCell::new(RItem::new(r10(), NV2)))),
            //
        ],
        Some(NV4),
    );
    let col2 = RCol::new(
        vec![
            None, //
            Some(Rc::new(RefCell::new(RItem::new(r10(), NV2DOT)))),
            None, //
        ],
        Some(NV4),
    );
    let col3 = RCol::new(
        vec![
            None, //
            None, //
            Some(Rc::new(RefCell::new(RItem::new(r10(), NV2)))),
        ],
        Some(NV2),
    );

    let matrix = RMatrix::new(vec![
        Rc::new(RefCell::new(col0)),
        Rc::new(RefCell::new(col1)),
        Rc::new(RefCell::new(col2)),
        Rc::new(RefCell::new(col3)),
    ]);

    matrix
}

pub fn matrix_test2() -> RMatrix {
    let col0 = RCol::new(
        vec![
            Some(Rc::new(RefCell::new(RItem::new(r20(), 0)))),
            Some(Rc::new(RefCell::new(RItem::new(r20(), 0)))),
            //
        ],
        None,
    );
    let col1 = RCol::new(
        vec![
            Some(Rc::new(RefCell::new(RItem::new(
                vec![NRect::new(0.0, 0.0, 10.0, 30.0)],
                NV2,
            )))),
            Some(Rc::new(RefCell::new(RItem::new(r10(), NV4)))),
            //
        ],
        Some(NV4),
    );
    let col2 = RCol::new(
        vec![
            None, //
            Some(Rc::new(RefCell::new(RItem::new(
                vec![NRect::new(0.0, 0.0, 40.0, 5.0)],
                NV4,
            )))),
        ],
        Some(NV4),
    );
    let col3 = RCol::new(
        vec![
            Some(Rc::new(RefCell::new(RItem::new(
                vec![NRect::new(-0.0, 0.0, 20.0, 20.0)],
                0,
            )))),
            // Some(Rc::new(RefCell::new(RItem::new(r20(), 0)))),
            None, //
        ],
        None,
    );

    let col4 = RCol::new(
        vec![
            Some(Rc::new(RefCell::new(RItem::new(
                vec![NRect::new(0.0, 0.0, 10.0, 50.0)],
                NV2,
            )))),
            Some(Rc::new(RefCell::new(RItem::new(
                vec![NRect::new(-20.0, 0.0, 40.0, 5.0)],
                NV4,
            )))),
            //
        ],
        Some(NV2),
    );

    let col5 = RCol::new(
        vec![
            Some(Rc::new(RefCell::new(RItem::new(r20(), 0)))),
            // Some(Rc::new(RefCell::new(RItem::new(r20(), 0)))),
            None, //
        ],
        None,
    );

    let matrix = RMatrix::new(vec![
        Rc::new(RefCell::new(col0)),
        Rc::new(RefCell::new(col1)),
        Rc::new(RefCell::new(col2)),
        Rc::new(RefCell::new(col3)),
        Rc::new(RefCell::new(col4)),
        Rc::new(RefCell::new(col5)),
    ]);

    matrix
}

//----------------------------------------------------------------

// use graphics::item;

// use graphics::{glyphs::ebgaramond::*, prelude::*};
use crate::{
    prelude::*, render::fonts::ebgaramond::GLYPH_HEIGHT, types::some_cloneables::SomeCloneablePairs,
};

use crate::prelude::NRect;
// use render_notation::render::dev::*;

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
    pub position: Position,
    pub col_idx: usize,
    pub row_idx: usize,
}

impl RItem {
    pub fn new(rects: Vec<NRect>, dur: Duration) -> Self {
        Self {
            rects,
            duration: dur,
            position: 0,
            col_idx: 0,
            row_idx: 0,
        }
    }
}

#[derive(Debug)]
pub struct RCol {
    pub duration: Duration,
    pub items: Vec<Option<Rc<RefCell<RItem>>>>,
    pub spacing: f32,
    pub col_idx: usize,
}

impl RCol {
    pub fn new(items: Vec<Option<Rc<RefCell<RItem>>>>, duration: Option<Duration>) -> Self {
        Self {
            items,
            duration: duration.unwrap_or(0),
            spacing: 0.0,
            col_idx: 0,
        }
    }
}

#[derive(Debug)]
pub struct RRow {
    pub items: Vec<Option<Rc<RefCell<RItem>>>>,
    pub row_idx: usize,
    pub spacing_y: f32,
}
impl RRow {
    fn new(items: Vec<Option<Rc<RefCell<RItem>>>>, spacing_y: f32, row_idx: usize) -> Self {
        Self {
            items,
            spacing_y,
            row_idx,
        }
    }
}

#[derive(Debug)]
pub struct RMatrix {
    pub cols: Vec<Rc<RefCell<RCol>>>,
    // pub xrowitems: Vec<Vec<Option<Rc<RefCell<RItem>>>>>,
    pub rows: Vec<Rc<RefCell<RRow>>>,
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
            // set column index
            col.borrow_mut().col_idx = colidx;
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

        let mut rowidx = 0;
        for ritems in rowitems {
            let row = RRow::new(ritems, 0.0, rowidx);
            rows.push(Rc::new(RefCell::new(row)));
            rowidx += 1;
        }

        dbg!(rows.len());

        Self {
            cols: colitems,
            // xrowitems: Vec::new(),
            rows: rows,
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
            col.spacing = spacing_fn(&col.duration);
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
                            overlap_spacing - prev_col.spacing
                        } else {
                            overlap_spacing
                        };
                        left_col.spacing = left_col.spacing.max(spacing);
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
            println!("==========================");
            for (colidx, item) in row.items.iter().enumerate() {
                let col = self.get_column(colidx).unwrap().borrow();
                if let Some(item) = item {
                    let item = item.borrow();
                    for rect in item.rects.iter() {
                        let rect = rect.move_rect(colx, 0.0);
                        itemrects.push(rect);
                    }
                };
                colx += col.spacing;
            }
            dbg!(&itemrects);
            rowrects.push(itemrects);
        }

        let mut rowidx = 0;
        for pair in rowrects.windows(2) {
            let (uppers, lowers) = (&pair[0], &pair[1]);

            dbg!(uppers);
            dbg!(lowers);

            let overlap = nrects_overlap_y(uppers, lowers).unwrap_or(0.0);
            let mut row = self.get_row(rowidx).unwrap().borrow_mut();
            row.spacing_y = row.spacing_y.max(overlap);
            rowidx += 1;
        }
    }
}
