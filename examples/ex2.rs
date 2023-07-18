#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(clippy::uninlined_format_args)]
#![allow(clippy::useless_format)]

mod testdata;
use crate::testdata::*;

use graphics::{glyphs::ebgaramond::*, prelude::*};
use notation_rs::{
    prelude::*, render::fonts::ebgaramond::GLYPH_HEIGHT, types::some_cloneables::SomeCloneables,
};
use render_notation::render::dev::*;

fn main() {
    let matrix = matrix_test1();

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
                let nrect = NRectExt::new(rect, NRectType::Dev(true, "gray".to_string()));
                let graphic_item = next2graphic(&nrect).unwrap();
                items.push(graphic_item);
            }
            y += 30.0;
        }
        x += 30.0;
    }

    let svg = SvgBuilder::new().build(items).unwrap();
    std::fs::write("./examples/ex2.svg", svg).unwrap();

    //-----------------------------------------------------------------------

    // let row0 = &matrix.rowitems[0];
    // let pairs = SomeCloneables {
    //     items: row0.clone(),
    // };

    // for (left, left_idx, right, right_idx) in pairs.into_iter() {
    //     dbg!(left, right);
    // }
}
