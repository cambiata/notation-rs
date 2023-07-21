#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(clippy::uninlined_format_args)]
#![allow(clippy::useless_format)]

use std::cell::RefMut;

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
    let mut matrix = matrix_test2();
    matrix.calculate_col_spacing(SPACING_RELATIVE);
    matrix.calculate_row_spacing();
    matrix.calculate_measurements();

    matrix_to_svg(&matrix, "./examples/ex2.svg");

    matrix.add_horizontal_space(50.0);
    // matrix.add_vertical_space(10.0);
    matrix.calculate_measurements();
    matrix_to_svg(&matrix, "./examples/ex2b.svg");
}

fn matrix_to_svg(matrix: &RMatrix, svg_filename: &str) {
    let mut items = GraphicItems::new();
    for col in matrix.cols.iter() {
        let col = col.borrow();
        let mut rowidx = 0;
        for item in &col.items {
            if let Some(item) = item {
                let item = item.borrow();
                let coords = item
                    .coords
                    .expect("RItem coords should always be calculated!");
                let rects = &item.rects;
                for rect in rects {
                    let color = if col.duration == 0 { "orange" } else { "blue" };
                    let nrect = NRectExt::new(
                        rect.move_rect(coords.0, coords.1),
                        NRectType::Dev(false, color.to_string()),
                    );
                    let graphic_item = next2graphic(&nrect).unwrap();
                    items.push(graphic_item);
                }
            } else {
                let y = matrix.get_row(rowidx).unwrap().borrow().y;
                let x = col.x;
                let rect = NRect::new(x, y, 10.0, 10.0);
                let nrect = NRectExt::new(rect, NRectType::Dev(true, "gray".to_string()));
                let graphic_item = next2graphic(&nrect).unwrap();
                items.push(graphic_item);
            }
            rowidx += 1;
        }
    }
    dbg!(matrix.width, matrix.height);
    let svg = SvgBuilder::new().build(items).unwrap();
    std::fs::write(svg_filename, svg).unwrap();
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
