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
    let bar_data = QCode::bars("nv4 0 % nv8 2 0").unwrap();
    let (bartemplate, bars) = bar_data;
    let mut matrix = bars.to_matrix(&bartemplate).unwrap();
    bars.add_beamgroups_to_matrix_items();

    matrix.calculate_col_spacing(ALLOTMENT_RELATIVE_FN);
    matrix.calculate_beamgroups();
    matrix.calculate_row_spacing();
    matrix.calculate_measurements();

    matrix_to_svg(&matrix, "./examples/ex3AX.svg");

    matrix.add_horizontal_space(100.0);
    matrix.add_vertical_space(50.0);
    matrix.calculate_measurements();

    // matrix_to_svg(&matrix, "./examples/ex3B.svg");
}
