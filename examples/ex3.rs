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
    println!("ex3");

    // let bar_data = QCode::bars("|clef G F | 0 0 / 0 0 0 ").unwrap();
    let bar_data =
        // QCode::bars("0 nv8 1 n2 nv16 3 #0,-1 -2 -1 Nv8 -3 -4 % 2 b2 2 2 / nv4 0 0 0 0").unwrap();
        QCode::bars("nv8 0 0 0 0 / 0 0").unwrap();

    // QCode::bars("|clefs G F - | 0 % 1 / 0 /lyr $lyr:aaa | 0 / 0 /lyr $lyr:bbb").unwrap();

    let (bartemplate, bars) = bar_data;
    let mut matrix = bars.to_matrix().unwrap();

    matrix.calculate_col_spacing(SPACING_RELATIVE);
    matrix.calculate_row_spacing();
    matrix.calculate_measurements();

    matrix_to_svg(&matrix, "./examples/ex3A.svg");
}
