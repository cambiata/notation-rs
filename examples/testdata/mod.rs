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

//----------------------------------------------------------------

// use graphics::{glyphs::ebgaramond::*, prelude::*};
use notation_rs::{
    prelude::*, render::fonts::ebgaramond::GLYPH_HEIGHT, types::some_cloneables::SomeCloneablePairs,
};
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
    pub rowitems: Vec<Option<Rc<RefCell<RItem>>>>,
    pub spacing: f32,
    pub col_idx: usize,
}

impl RCol {
    pub fn new(rowitems: Vec<Option<Rc<RefCell<RItem>>>>, duration: Option<Duration>) -> Self {
        Self {
            rowitems,
            duration: duration.unwrap_or(0),
            spacing: 0.0,
            col_idx: 0,
        }
    }
}

#[derive(Debug)]
pub struct RMatrix {
    pub colitems: Vec<Rc<RefCell<RCol>>>,
    pub rowitems: Vec<Vec<Option<Rc<RefCell<RItem>>>>>,
}

impl RMatrix {
    pub fn new(colitems: Vec<Rc<RefCell<RCol>>>) -> Self {
        let row_count = &colitems[0].borrow().rowitems.len();

        let mut rowitems: Vec<Vec<Option<Rc<RefCell<RItem>>>>> = vec![vec![]; *row_count];

        let firstcol = &colitems[0];
        for item in firstcol.borrow().rowitems.iter() {
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
            if col.borrow().rowitems.len() != *row_count {
                panic!("part_count mismatch");
            }

            let mut rowidx = 0;
            for row in col.borrow_mut().rowitems.iter_mut() {
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
        Self { colitems, rowitems }
    }
}

fn main() {}
