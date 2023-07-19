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
    prelude::*, render::fonts::ebgaramond::GLYPH_HEIGHT, types::some_cloneables::SomeCloneablePairs,
};
use render_notation::render::dev::*;

fn main() {
    let matrix = matrix_test2();
    matrix.calculate_col_spacing(SPACING_RELATIVE);

    //---------------------------------------------------------------------

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
        // for row in &matrix.rowitems {
        //     // dbg!(row);
        //     let pairs = SomeCloneablePairs { items: row.clone() };
        //     for (left, left_idx, right, right_idx) in pairs.into_iter() {
        //         //println!("==========================");
        //         match [&left, &right] {
        //             [Some(left), Some(right)] => {
        //                 let left = left.borrow_mut();
        //                 let right = right.borrow_mut();
        //                 // let right_col = &matrix.get_column_clone(right.col_idx);
        //                 let spacing_overlap: f32 =
        //                     nrects_overlap_x(&left.rects, &right.rects).unwrap_or(0.0);
        //                 let spacing_duration = SPACING_LINEAR(&left.duration);

        //                 let mut left_col =
        //                     matrix.get_column_clone(left.col_idx).unwrap().borrow_mut();
        //                 let max: f32 = f32::max(spacing_overlap, spacing_duration);
        //                 left_col.spacing = max;
        //             }

        //             [Some(left), None] => {
        //                 panic!("Should not happen - right should always be Some(T)");
        //             }
        //             [None, Some(right)] => {
        //                 let right = right.borrow();
        //                 let right_col = &matrix.get_column_clone(right.col_idx);
        //                 if let Some(right_col) = right_col {
        //                     let right_col_mut = right_col.borrow_mut();
        //                 }
        //             }

        //             [None, None] => {
        //                 panic!("Should not happen - right should always be Some(T)");
        //             }
        //         }
        //     }
        // }
    }

    // fn calculate_col_spacing(matrix: &RMatrix, spacing_fn: SpacingFn) {
    //     for row in &matrix.rowitems {
    //         // dbg!(row);
    //         let pairs = SomeCloneablePairs { items: row.clone() };
    //         for (left, left_idx, right, right_idx) in pairs.into_iter() {
    //             //println!("==========================");
    //             match [&left, &right] {
    //                 [Some(left), Some(right)] => {
    //                     let left = left.borrow_mut();
    //                     let right = right.borrow_mut();
    //                     // let left_col = &matrix.get_column_clone(left.col_idx);
    //                     // let right_col = &matrix.get_column_clone(right.col_idx);
    //                     let spacing_overlap: f32 =
    //                         nrects_overlap_x(&left.rects, &right.rects).unwrap_or(0.0);
    //                     let spacing_duration = SPACING_LINEAR(&left.duration);

    //                     let mut left_col =
    //                         matrix.get_column_clone(left.col_idx).unwrap().borrow_mut();
    //                     let max: f32 = f32::max(spacing_overlap, spacing_duration);
    //                     left_col.spacing = max;
    //                 }

    //                 [Some(left), None] => {
    //                     panic!("Should not happen - right should always be Some(T)");
    //                 }
    //                 [None, Some(right)] => {
    //                     let right = right.borrow();
    //                     let right_col = &matrix.get_column_clone(right.col_idx);
    //                     if let Some(right_col) = right_col {
    //                         let right_col_mut = right_col.borrow_mut();
    //                     }
    //                 }

    //                 [None, None] => {
    //                     panic!("Should not happen - right should always be Some(T)");
    //                 }
    //             }
    //         }
    //     }
    // }

    #[test]
    fn signature() {
        fn test(dur: usize) -> f32 {
            10.0
        }

        let t: fn(usize) -> f32 = test;
    }
}
