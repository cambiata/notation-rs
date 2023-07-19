#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(clippy::uninlined_format_args)]
#![allow(clippy::useless_format)]

use graphics::{glyphs::ebgaramond::*, prelude::*};
use notation_rs::{
    //
    prelude::*,
    render::fonts::ebgaramond::GLYPH_HEIGHT,
    render::render_items::*,
    types::some_cloneables::SomeCloneablePairs,
};
use render_notation::render::dev::*;

fn main() {
    let matrix = matrix_test2();
    matrix.calculate_col_spacing(SPACING_RELATIVE);
    matrix.calculate_row_spacing();

    //---------------------------------------------------------------------

    let mut items = GraphicItems::new();

    let mut x = 0.0;
    for col in &matrix.cols {
        let col = col.borrow();
        dbg!(&col.duration, &col.spacing);

        let mut y = 0.0;
        let mut rowidx = 0;
        for item in &col.items {
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
            let row = &matrix.get_row(rowidx).unwrap().borrow();
            y += row.spacing_y;
            rowidx += 1;
        }
        x += col.spacing;
    }

    let svg = SvgBuilder::new().build(items).unwrap();
    std::fs::write("./examples/ex2.svg", svg).unwrap();

    //-----------------------------------------------------------------------
}

#[cfg(test)]
mod tests {
    use core::panic;

    use crate::testdata::*;
    use graphics::{glyphs::ebgaramond::*, prelude::*};
    use notation_rs::{
        prelude::*, render::fonts::ebgaramond::GLYPH_HEIGHT,
        types::some_cloneables::SomeCloneablePairs,
    };
    use render_notation::render::dev::*;

    #[test]
    fn example() {
        let matrix = matrix_test2();
        matrix.calculate_col_spacing(SPACING_LINEAR);
    }
}
