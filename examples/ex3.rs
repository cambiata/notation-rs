#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(clippy::uninlined_format_args)]
#![allow(clippy::useless_format)]

use graphics::{
    builder::{BuilderOptions, SizeUnit},
    glyphs::ebgaramond::*,
    prelude::*,
};
use notation_rs::prelude::*;

use render_notation::prelude::*;
// use render_notation::render::elements::*;
// use render_notation::render::output::*;

fn main() {
    // let bar_data = QCode::bars("|clef G F - | 0,1,4 1 / 0 0 /lyr lyr:aa lyr:c |bl | #0 nv8 1 b2 / 0 0 /lyr nv4dot lyr:bbb nv8 lyr:c").unwrap();
    // let bar_data = QCode::bars("|clef G - | nv2 #0 nv8 1 nv16 0 -3 nv8 -2,b-1 1#  /lyr nv2 $lyr:aaa $lyr:b").unwrap();
    // let bar_data = QCode::bars("|clef F G C | 0 / 0 / 0 ").unwrap();
    // let bar_data = QCode::bars("|clef G | nv4 0 nv8 0 0 % nv8 2 nv16 2 2 nv4 2 ").unwrap();
    // let bar_data = QCode::bars("|clef G | nv1 0 % nv4 1# 2 2 2 2 ").unwrap();
    // let bar_data = QCode::bars("nv1 0 nv2 0").unwrap();
    // let bar_data = QCode::bars("nv16 -1 -2 -3 -3 ").unwrap();
    // let bar_data = QCode::bars("nv16 0 -1 -4 -5 -4 -2 -2 -1 % nv16 5 3 3 2 2 3 3 5").unwrap();
    // let bar_data = QCode::bars("nv8 0 1 nv16 0 0 0 0 nv8 0 0 ").unwrap();
    // let bar_data = QCode::bars("nv4 0 % nv8 2 0").unwrap();
    // let bar_data = QCode::bars("|clef G F | 0 -2 % 2 2  / 0,3 0,-3 | nv8 -3 3 nv16 -3 -1 1 3 nv8 -2 4 nv16 -2 0 2 4 / nv8 3 -3 nv16 3 1 -1 -3 nv8 4 -2 nv16 4 2 0 -2 ").unwrap();
    // let bar_data = QCode::bars("|clef G |  0,-5 -1,-6 -4,4 1,-1 1,-3  0 1 3 5 7 0,2 0,4 0,6 0,8 ").unwrap();
    // let bar_data = QCode::bars("|clef G |  0 1 nv8 0 0 1 1 0 2 2 0 -2 0 0 -2 ").unwrap();
    // let bar_data = QCode::bars("|clef G - |  nv4 -1 nv16 3 2 1 0 nv4 -2 nv8 -3 2 % nv4 p nv2 5 nv4 3 /lyr lyr:aaaa nv2 lyr:bbb nv4 lyr:abc ").unwrap();
    // let bar_data = QCode::bars("nv8 -5,-7 5,3 5,3 -5,-7 -4,-2 6,8 6,8 -4,-2").unwrap();
    // let bar_data = QCode::bars("|clef G - | nv8 0 1 /lyr nv8 lyr:aa lyr:bbb ").unwrap();
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
    // let bar_data = QCode::bars("nv8 0 p 2 p nv16 0 p 2 p % nv8 1 p 1 p / nv8 0 p 2 p nv16 0 p 2 p").unwrap();
    // let bar_data = QCode::bars("0 % 1 ").unwrap();
    // let bar_data = QCode::bars("5 4 3 2 1 0 -1 -2 -3 -4 -5 ").unwrap();
    // let bar_data = QCode::bars("5 nv8 5 5 nv16 5 5 5 5 nv4 4 nv8 4 4 nv16 4 4 4 4").unwrparttemplatesap();
    // let bar_data = QCode::bars(" -2_,-3 -2 p ~1,~2 -2_ |bl | -2 p -2_ -1").unwrap();
    // let bar_data = QCode::bars(" nv2 ~-2_ nv4 -3 % nv4 0_ nv2 1").unwrap();
    // let bar_data = QCode::bars(" -4#_,0_,2_ -4,0,2  ").unwrap();
    // let bar_data = QCode::bars(" 0 1_,3_,5_ 1,3,5 -1_,-3_,-5_ -1,-3,-5 ").unwrap();
    // let bar_data = QCode::bars(" -1_ -1 -1_,-3_ -1,-3 1_ 1 1_,3_ 1,3 | -2_ -2 -2_,-4_ -2,-4 % 2_ 2 2_,4_ 2,4 ").unwrap();
    // let bar_data = QCode::bars("|clef G | nv4 0_ 0 2_ 2 nv16 3_,5_ 3,5 3 3_ |bl | 3 ").unwrap();
    // let bar_data = QCode::bars(" 0 3 % 1_ 1  | bp % 0 0 | 0 0 nv16 0_,2_ 0,2 ").unwrap();
    // let bar_data = QCode::bars("|clef G | nv4 2 0 % nv16 #3 n3 3_ 3 ").unwrap();

    // let bar_data = QCode::bars("2,4,6,7,9").unwrap();
    // let bar_data = QCode::bars("nv1 #7,n8 -7,-8").unwrap();
    // let bar_data = QCode::bars(" 0 0 /lyr $lyr:aaa  ").unwrap();

    // let bar_data = QCode::bars("|clef G - | nv8 6 5 4 3  nv4 2 2 /lyr nv8 lyr:Hej,  lyr:sa lyr:Pet lyr:ro nv4 lyr:nel lyr:la").unwrap();

    // let bar_data = QCode::bars("|clef G F - |key ## ## - | 0 0 / 0 0 /lyr lyr:123 lyr:eleison").unwrap();
    // let bar_data = QCode::bars("|clef - G - |/lyr lyr:aa lyr:bb / 0  0 /lyr tpl:3:1 tpl:-2:2 tpl:0:3 tpl:0:4").unwrap();
    // let bar_data = QCode::bars("|sp3 |clef G - - | 0 0 0 0 /lyr lyr:Hej lyr:och lyr:hopp /lyr lyr:Ky lyr:ri lyr:e").unwrap();

    // let bar_data = QCode::bars(" nv16 0 -1 -2 -3 % 2 ").unwrap();
    // let bar_data = QCode::bars(" -5 nv16 -1 -3 -1 -3  % nv16 4 1 4 1 nv4 1").unwrap();
    // let bar_data = QCode::bars(" -3 nv8 0 nv4 1 nv8 1 nv1 -2 2 % nv2 3 3 3").unwrap();
    // let bar_data = QCode::bars("-3 -1 1 3 nv1 -3 -1 1 3 % nv1 3 4 nv4 5 4 3").unwrap();
    // let bar_data = QCode::bars("2 nv16 2 1 0 -1  -3 -2 -1 0 nv4 -2 % nv16 7 4 5 6 nv4 4 nv16 7 6 5 4 nv4 2").unwrap();

    // let bar_data = QCode::bars(" p 0  % 4_ 4_ 4 ").unwrap(); // bindebågar för voice2?

    // let bar_data = QCode::bars("|sp1 x 80 |clef G |sp2 |key # |sp2 |time 2:4 |sp2 |time 3:4 |sp2 |time 6:4 |sp3 |   0 1 |bld |spc | 0 -1 |bl ").unwrap(); // bindebågar för voice2?

    // let bar_data = QCode::bars("sp 20 400 | 0 0 1 -2 |sp3 | 0 0 |bld ").unwrap();

    // let bar_data = QCode::bars("|clef - G |key - ## |sp3 | lyr tpl:0:1 tpl:-4:5 tpl:-5:6 tpl:-6:7 tpl:-7:1 / 5 1 0 -1 -2").unwrap();

    // let bar_data = QCode::bars("|sp2 |clef G - |sp3 | nv4 6 -1 -8 /lyr lyr:c¹  lyr:c² lyr:c³ lyr:c lyr:C lyr:Cº lyr:cµ lyr:c¶  |bl").unwrap();
    // let bar_data = QCode::bars("|sp2 |clef G F - |sp3 | nv4 6 -1 -8 / nv4 0 0 0 /lyr lyr:c¹  lyr:c² lyr:c³ lyr:c lyr:C lyr:Cº lyr:cµ lyr:c¶  |bl").unwrap();

    // SUPERSCRIPT
    // lyr:c¹
    // lyr:c²
    // lyr:c³

    // SUBSCRIPT
    // lyr:cº
    // lyr:cµ
    // lyr:c¶

    // let bar_data = QCode::bars("/lyr lyr:c¹  lyr:c² lyr:c³ lyr:cº lyr:Cµ lyr:Fiss¶ ").unwrap();

    // let bar_data = QCode::bars("|sp2 |clef G F |sp3 | 0LH,2LW -2,3 % 5LG 5 / #1L 0").unwrap();
    // let bar_data = QCode::bars("|sp2 |clef G |sp3 | 0 -1 -2 -3 -4 -5 -6 -7  3 2 1 0 % 0 1 2 3 4 5 6 7").unwrap();
    // let bar_data = QCode::bars("|sp2 |clef G |sp3 | 0 fun:T:64:3:()) ").unwrap();
    // let bar_data = QCode::bars("|sp2 |clef G |sp3 | 0 |bl").unwrap();
    // let bar_data = QCode::bars("|clef G |sp3 | 6LW 5LW 4LH 3LW 2LW 1LW 0LH -1 |bl").unwrap();
    // let bar_data = QCode::bars("|clef G |sp3 | nv1 6L 0 -1L -8 2LW 1LW 0LH -1 |bl").unwrap();

    // let bar_data = QCode::bars("|clef G |sp3 | 0 0 chd:Gbmb9:F# chd:A chd:A7 chd:Am7 chd:Abm7 chd:Em:G chd:A#:Gb chd:Amsus4 chd:Gmmaj7:B |bl").unwrap();
    // let bar_data = QCode::bars("|clef - G |sp3 |/lyr chd:Gbmb9:F# chd:A chd:Am chd:D:F# / nv8 0 0 0 0 0 0 0 0 |bl").unwrap();
    let bar_data = QCode::bars("|sp2 | nv8 0 nv16 1 2 nv2 3 |bl").unwrap();

    let (bartemplate, mut bars) = bar_data;

    bars.create_matrix(Some(bartemplate)).unwrap();
    bars.resolve_stuff();
    bars.matrix_add_beamgroups();
    bars.matrix_add_ties();
    bars.matrix_add_lines();
    let matrix = bars.matrix.as_mut().unwrap();
    matrix.calculate_col_spacing(ALLOTMENT_RELATIVE_FN);
    matrix.calculate_beamgroups();
    matrix.calculate_attachment_points(&bars.note_id_map);

    matrix.calculate_row_spacing();
    matrix.calculate_col_row_item_measurements();
    matrix.calculate_matrix_size();

    //------------------------------------------------------
    let svg = matrix_to_svg(&matrix, true, None);
    std::fs::write("./examples/ex3A.svg", svg).unwrap();

    // //---------------------------------------------------
    // let script_name = "TestSuperscript1";
    // let category_name = "MusicClips";
    // let fuse = matrix_to_fuse(
    //     matrix,
    //     false,
    //     Some(BuilderOptions {
    //         size_unit: SizeUnit::Pixel,
    //         size_scaling: 0.005,
    //     }),
    //     script_name,
    //     category_name,
    // );
    // // std::fs::write(format!("C:/Users/Cambiata MusikProd/AppData/Roaming/Blackmagic Design/Fusion/Fuses/{}.fuse", script_name), fuse).unwrap();
    // std::fs::write(format!("./examples/{}.fuse", script_name), fuse).unwrap();

    // let playdata = bars.calc_playback();
    // let playpositions = bars.calculate_playpositions();
    // std::fs::write("./examples/ex3A.playdata.json", playdata.to_json()).unwrap();
    // std::fs::write("./examples/ex3A.positions.json", playpositions.to_json()).unwrap();
    // matrix.add_horizontal_space(100.0);
    // matrix.add_vertical_space(50.0);
    // matrix.calculate_col_row_item_measurements();
    // matrix.calculate_matrix_size();
    // let svg = matrix_to_svg(&matrix, true, None);
    // std::fs::write("./examples/ex3B.svg", svg).unwrap();
}

#[cfg(test)]
mod tests2 {
    use std::collections::BTreeMap;

    const TEST_1: &[usize] = &[11, 22];
    const TEST_2: &[usize] = &[333, 444, 555];
    const A: &[&[usize]] = &[TEST_1, TEST_2];

    // pub const MERRIWEATHER_REGULAR_CHAR_45: &'static [PathSegment] = &[M(410.5012, -225.93475), L(89.10103, -225.93475), L(89.10103, -284.00955), L(410.5012, -284.00955), L(410.5012, -225.93475), Z];
    // pub const MERRIWEATHER_REGULAR_CHAR_45_SIZE: &'static (f32, f32) = &(321.40015, 58.0748);

    #[test]
    fn example() {
        dbg!(A);

        let s = BTreeMap::from([(1, 11), (3, 33)]);
        println!("{:?}", s);
    }
}
