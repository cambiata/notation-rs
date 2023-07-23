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
    // let bar_data =
    //     QCode::bars("|clef G F - | 0,1,4 1 / 0 0 /lyr $lyr:aa $lyr:c |bl | #0 nv8 1 b2 / 0 0 /lyr nv4dot $lyr:bbb nv8 $lyr:c").unwrap();

    let bar_data =
        QCode::bars("|clef G - | nv2 2 nv8 1 nv16 0 -3 nv8 -2,b-1 1#  /lyr nv2 $lyr:aaa $lyr:b")
            .unwrap();
    let (bartemplate, bars) = bar_data;

    let mut matrix = bars.to_matrix(&bartemplate).unwrap();

    matrix.calculate_col_spacing(ALLOTMENT_RELATIVE_FN);
    matrix.calculate_row_spacing();
    matrix.calculate_measurements();

    matrix_to_svg(&matrix, "./examples/ex3A.svg");

    matrix.add_horizontal_space(100.0);
    matrix.add_vertical_space(50.0);
    matrix.calculate_measurements();

    matrix_to_svg(&matrix, "./examples/ex3B.svg");
}
