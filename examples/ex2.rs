#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(clippy::uninlined_format_args)]
#![allow(clippy::useless_format)]

use graphics::{glyphs::ebgaramond::*, prelude::*};
use notation_rs::{prelude::*, render::fonts::ebgaramond::GLYPH_HEIGHT};
use render_notation::render::dev::*;

fn main() {
    let col0 = MCol::new(
        vec![
            Some(Rc::new(RefCell::new(MItem::new(r20(), 0)))),
            Some(Rc::new(RefCell::new(MItem::new(r20(), 0)))),
            Some(Rc::new(RefCell::new(MItem::new(r20(), 0)))),
            //
        ],
        None,
    );
    let col1 = MCol::new(
        vec![
            Some(Rc::new(RefCell::new(MItem::new(r10(), NV1)))),
            Some(Rc::new(RefCell::new(MItem::new(r10(), NV4)))),
            Some(Rc::new(RefCell::new(MItem::new(r10(), NV2)))),
            //
        ],
        Some(NV4),
    );
    let col2 = MCol::new(
        vec![
            None, //
            Some(Rc::new(RefCell::new(MItem::new(r10(), NV2DOT)))),
            None, //
        ],
        Some(NV4),
    );
    let col3 = MCol::new(
        vec![
            None, //
            None, //
            Some(Rc::new(RefCell::new(MItem::new(r10(), NV2)))),
        ],
        Some(NV2),
    );

    let matrix = MMatrix::new(vec![
        Rc::new(RefCell::new(col0)),
        Rc::new(RefCell::new(col1)),
        Rc::new(RefCell::new(col2)),
        Rc::new(RefCell::new(col3)),
    ]);

    let mut items = GraphicItems::new();

    let mut x = 0.0;
    for col in &matrix.colitems {
        let col = col.borrow();
        dbg!(&col.duration, &col.spacing);

        let mut y = 0.0;
        for item in &col.rowitems {
            if let Some(row) = item {
                let item = row.borrow();
                let rects = &item.rects;
                for rect in rects {
                    // let graphic_rect = nrect2rect(*rect, Strokestyle(1., Blue), NoFill);
                    let color = if col.duration == 0 { "orange" } else { "blue" };
                    let nrect = NRectExt::new(
                        rect.move_rect(x, y),
                        NRectType::Dev(false, color.to_string()),
                    );
                    let graphic_item = next2graphic(&nrect).unwrap();
                    items.push(graphic_item);
                }
            } else {
                let rect = NRect::new(x, y, 10.0, 10.0);
                let nrect = NRectExt::new(rect, NRectType::Dev(true, "red".to_string()));
                let graphic_item = next2graphic(&nrect).unwrap();
                items.push(graphic_item);
            }
            y += 50.0;
        }
        x += 50.0;

        // dbg!(&col.borrow().dur, &col.borrow().spacing);
    }

    let svg = SvgBuilder::new().build(items).unwrap();
    std::fs::write("./examples/ex2.svg", svg).unwrap();
}

fn r10() -> Vec<NRect> {
    vec![NRect::new(0.0, 0.0, 10.0, 10.0)]
}

fn r20() -> Vec<NRect> {
    vec![NRect::new(0.0, 0.0, 10.0, 20.0)]
}

// trait Col {}

#[derive(Debug)]
pub struct MItem {
    pub rects: Vec<NRect>,
    pub duration: Duration,
    pub position: Position,
    pub col_idx: usize,
    pub row_idx: usize,
}

impl MItem {
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
pub struct MCol {
    pub duration: Duration,
    pub rowitems: Vec<Option<Rc<RefCell<MItem>>>>,
    pub spacing: f32,
    pub col_idx: usize,
}

impl MCol {
    pub fn new(rowitems: Vec<Option<Rc<RefCell<MItem>>>>, duration: Option<Duration>) -> Self {
        Self {
            rowitems,
            duration: duration.unwrap_or(0),
            spacing: 0.0,
            col_idx: 0,
        }
    }
}

// impl Col for MCol {}

#[derive(Debug)]
pub struct MMatrix {
    pub colitems: Vec<Rc<RefCell<MCol>>>,
    pub rowitems: Vec<Vec<Option<Rc<RefCell<MItem>>>>>,
}

impl MMatrix {
    pub fn new(colitems: Vec<Rc<RefCell<MCol>>>) -> Self {
        let row_count = &colitems[0].borrow().rowitems.len();

        let mut rowitems: Vec<Vec<Option<Rc<RefCell<MItem>>>>> = vec![vec![]; *row_count];

        let firstcol = &colitems[0];
        for item in firstcol.borrow().rowitems.iter() {
            if item.is_none() {
                panic!("firstcol has None - shouldn't have!");
            }
        }

        let mut colidx = 0;
        for col in &colitems {
            let mut col = col.borrow_mut();

            if col.rowitems.len() != *row_count {
                panic!("part_count mismatch");
            }

            col.col_idx = colidx;

            let mut rowidx = 0;
            for row in col.rowitems.iter_mut() {
                if let Some(row) = row {
                    let mut row = row.borrow_mut();
                    row.row_idx = rowidx;
                    row.col_idx = colidx;
                }

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
