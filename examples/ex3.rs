#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(clippy::uninlined_format_args)]
#![allow(clippy::useless_format)]

use graphics::{glyphs::ebgaramond::*, prelude::*};
use notation_rs::prelude::*;
use render_notation::render::dev::*;

fn main() {
    // let bar_data = QCode::bars("|clef G F - | 0,1,4 1 / 0 0 /lyr $lyr:aa $lyr:c |bl | #0 nv8 1 b2 / 0 0 /lyr nv4dot $lyr:bbb nv8 $lyr:c").unwrap();
    // let bar_data = QCode::bars("|clef G - | nv2 #0 nv8 1 nv16 0 -3 nv8 -2,b-1 1#  /lyr nv2 $lyr:aaa $lyr:b").unwrap();
    // let bar_data = QCode::bars("|clef F G C | 0 / 0 / 0 ").unwrap();
    // let bar_data = QCode::bars("|clef G | nv4 0 nv8 0 0 % nv8 2 nv16 2 2 nv4 2 ").unwrap();
    // let bar_data = QCode::bars("|clef G | nv1 0 % nv4 1# 2 2 2 2 ").unwrap();
    // let bar_data = QCode::bars("nv1 0 nv2 0").unwrap();
    // let bar_data = QCode::bars("nv16 -1 -2 -3 -3 ").unwrap();
    // let bar_data = QCode::bars("nv16 -1 -2 -2 -3 ").unwrap();
    // let bar_data = QCode::bars("nv16 0 -1 -4 -5 -4 -2 -2 -1 % nv16 5 3 3 2 2 3 3 5").unwrap();
    // let bar_data = QCode::bars("nv8 0 1 nv16 0 0 0 0 nv8 0 0 ").unwrap();
    // let bar_data = QCode::bars("nv4 0 % nv8 2 0").unwrap();
    // let bar_data = QCode::bars("|clef G F | 0 -2 % 2 2  / 0,3 0,-3 | nv8 -3 3 nv16 -3 -1 1 3 nv8 -2 4 nv16 -2 0 2 4 / nv8 3 -3 nv16 3 1 -1 -3 nv8 4 -2 nv16 4 2 0 -2 ").unwrap();
    // let bar_data = QCode::bars("|clef G |  0,-5 -1,-6 -4,4 1,-1 1,-3  0 1 3 5 7 0,2 0,4 0,6 0,8 ").unwrap();
    // let bar_data = QCode::bars("|clef G |  0 1 nv8 0 0 1 1 0 2 2 0 -2 0 0 -2 ").unwrap();
    // let bar_data = QCode::bars("|clef G - |  nv4 -1 nv16 3 2 1 0 nv4 -2 nv8 -3 2 % nv4 p nv2 5 nv4 3 /lyr $lyr:aaaa nv2 $lyr:bbb nv4 $lyr:abc ").unwrap();
    // let bar_data = QCode::bars("nv8 -5,-7 5,3 5,3 -5,-7 -4,-2 6,8 6,8 -4,-2").unwrap();
    // let bar_data = QCode::bars("|clef G - | nv8 0 1 /lyr $lyr:aa  ").unwrap();
    // let bar_data = QCode::bars("|clef G | -3 nv8 0 1 nv4 1 % nv8 #-2 nv4 7,9 nv8 7 nv4 2").unwrap();
    // let bar_data = QCode::bars("|clef G | nv16 0 -1 -2 -3 -4 -5 -6 -7 -8 -9 -10 -11 -12 -13 -14 -15 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16").unwrap();
    // let bar_data = QCode::bars("|clef G - |  nv4 #3 nv16 3 b2 p 0 nv4 -2 nv8 -3 2 % nv4 p nv2dot 4 /lyr $lyr:aaa nv2 $lyr:bbb nv4 $lyr:abc ").unwrap();
    // let bar_data = QCode::bars("nv8 -2 nv16 -3 -4 % nv16 3 2 nv8 4").unwrap();
    // let bar_data = QCode::bars("nv8 -2 nv16 -3 -4 ").unwrap();
    // let bar_data = QCode::bars("0 0 % nv8 3 5 5 3").unwrap();
    // let bar_data = QCode::bars("nv8dot 4# nv16 0 nv16 -4 nv8 1 nv16 2 nv16 4 3 5,3 1 nv8 -3 nv16 -4,-6 -5 ").unwrap();
    // let bar_data = QCode::bars("-2 -2 % nv16 1 #3 5 7 nv8 3 nv16 2 1 / nv8 4 -3 2 -6 |bl | nv16 -1 nv8 -2 nv16 -3 nv16 -4 nv8dot -2 % nv4dot 2 nv8 3 / nv8 0 2  4 1 |bl | nv8 3 p -4 -6 / 0 p ").unwrap();
    // let bar_data = QCode::bars("nv8 -1 nv16 -2 -3 nv8 -3 nv16 -4 -5 % nv16 1 2 nv8 3 / nv16 0 1 nv8 3  nv16 -2 -2 -3 -4 / nv16 0 1 2 3  nv16 -2 -2 -3 -4").unwrap();
    // let bar_data = QCode::bars("nv16 3 2 1 / nv16 3 2 1").unwrap();
    let bar_data = QCode::bars("nv8 0 p 2 p nv16 0 p 2 p % nv8 1 p 1 p / nv8 0 p 2 p nv16 0 p 2 p").unwrap();
    // let bar_data = QCode::bars("0 % 1 ").unwrap();

    let (bartemplate, bars) = bar_data;
    let mut matrix = bars.to_matrix(&bartemplate).unwrap();
    bars.add_beamgroups_to_matrix_items();

    matrix.calculate_col_spacing(ALLOTMENT_RELATIVE_FN);
    // matrix.calculate_col_row_item_measurements();
    matrix.calculate_beamgroups();
    matrix.calculate_row_spacing();
    matrix.calculate_col_row_item_measurements();
    matrix.calculate_matrix_size();

    matrix_to_svg(&matrix, "./examples/ex3A.svg");

    matrix.add_horizontal_space(100.0);
    matrix.add_vertical_space(50.0);
    matrix.calculate_col_row_item_measurements();
    matrix.calculate_matrix_size();

    // matrix_to_svg(&matrix, "./examples/ex3B.svg");
}
