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

fn main() -> notation_rs::prelude::Result<()> {
    // let bar_data = QCode::bars(" 0  0 / 1 | bp / bp | 0 / 1 1 ").unwrap();
    let bar_data = QCode::bars(" 0 0 0 % 1 1 1 / 2 2 2 | 0 % 1 / 2 |clef G F | 0 % 1 / 2").unwrap();
    // let bar_data = QCode::bars(" 0 0 p 1 1 % 2 p 3 | 1 % 3").unwrap();

    // let bar_data = QCode::bars(" 0 % 1 | bp | 0 % 1 ").unwrap();
    let bar_data = QCode::bars(" 0  % 1 / 2 |mul | 0 % 1  / 2  % 3 | 0 / 2 % 3 ").unwrap();

    let (bartemplate, bars) = bar_data;
    bars.resolve_ties();
    Ok(())
}

// fn mainx() -> notation_rs::prelude::Result<()> {
//     const COMPLEX_WIDTH: f32 = 130.;
//     // let voices = QCode::voices(" nv2 0,-5 % nv4 -1,0 0").unwrap();
//     // let voices = QCode::voices("nv4 0n,-3#,6b % nv2 0b").unwrap();
//     // let voices =
//     //     QCode::voices("nv2 p p p p p p p -3# -2 -1 0 1 2 3 % nv2 -3 -2 -1 0 1 2 3 p p p p p p p")
//     //         .unwrap();

//     let voices = QCode::voices("#3,0n $lyr:abc").unwrap();
//     let mut part = Part::new(PartType::Voices(voices));
//     part.set_voice_notes_references();
//     part.create_beamgroups(BeamingPattern::NValues(vec![NV4]));
//     part.create_complexes();
//     part.set_complex_durations();
//     part.set_beamgroups_directions(DirUAD::Auto);
//     part.set_note_directions();
//     part.create_complex_rects()?;

//     let complexes = part.complexes.as_ref().unwrap();

//     let clef = PathSegments(CADENZA_8.to_vec()).inv01();
//     let lines = PathSegments(CADENZA_31.to_vec()).inv01().move_path(0., 25.1);

//     let mut items = GraphicItems::new();
//     items.extend(five_lines((complexes.len() as f32 + 1.) * COMPLEX_WIDTH));

//     let clef_items = GraphicItems(vec![Path(clef, NoStroke, Fillstyle(Black))]).move_items(0.0, 25.1);
//     items.extend(clef_items);

//     for (idx, complex) in complexes.iter().enumerate() {
//         dbg!(idx);
//         let mut complex_items = GraphicItems::new();
//         for nrect in complex.borrow().rects.borrow().iter() {
//             let graphic_rect = nrect2graphic(nrect.0, Strokestyle(1., Blue), NoFill);
//             complex_items.push(graphic_rect);

//             let graphic_item = nrectext2graphic(nrect);
//             if let Some(item) = graphic_item {
//                 complex_items.push(item);
//             }
//         }
//         let x = (idx + 1) as f32 * COMPLEX_WIDTH;
//         complex_items = complex_items.move_items(x, 0.);
//         items.extend(complex_items);
//     }

//     let svg = SvgBuilder::new().build(items).unwrap();
//     std::fs::write("./examples/ex1.svg", svg)?;

//     Ok(())
// }
