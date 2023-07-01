#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(clippy::uninlined_format_args)]
#![allow(clippy::useless_format)]

use std::collections::HashMap;

use notation_rs::prelude::*;
use notation_rs::quick::QCode;

use graphics::item::Fill::Fillstyle;
use graphics::prelude::*;

fn main() -> notation_rs::prelude::Result<()> {
    const COMPLEX_WIDTH: f32 = 100.;

    let voices = QCode::voices("nv4 -1 p % 2 3 ").unwrap();

    let voices_beamings = beamings_from_voices(
        &voices,
        &BeamingPattern::NValues(vec![NV4]),
        &DirUAD::Auto,
        &DirUAD::Auto,
    )?;

    let note_beamings_map: HashMap<&Note, &BeamingItem<'_>> =
        get_map_note_beamings(&voices_beamings)?;
    let complexes = complexes_from_voices(&voices, &note_beamings_map)?;

    let clef = PathSegments(CADENZA_8.to_vec()).inv01();
    let lines = PathSegments(CADENZA_31.to_vec())
        .inv01()
        .move_path(0., 25.1);

    let mut items = GraphicItems::new();
    items.extend(five_lines((complexes.len() as f32 + 1.) * COMPLEX_WIDTH));

    let clef_items =
        GraphicItems(vec![Path(clef, NoStroke, Fillstyle(Black))]).move_items(0.0, 25.1);
    items.extend(clef_items);

    for (idx, complex) in complexes.into_iter().enumerate() {
        let mut complex_items = GraphicItems::new();

        let nrects = complex.get_rectangles().unwrap();
        for nrect in nrects {
            let graphic_rect = nrect2rect(nrect.0, Strokestyle(1., Blue), NoFill);
            complex_items.push(graphic_rect);
            let graphic_item = next2graphic(nrect);
            complex_items.push(graphic_item);
        }
        let x = (idx + 1) as f32 * COMPLEX_WIDTH;
        complex_items = complex_items.move_items(x, 0.);
        items.extend(complex_items);
    }

    let svg = SvgBuilder::new().build(items).unwrap();
    std::fs::write("./examples/ex1.svg", svg)?;

    Ok(())
}

fn nrect2rect(n: NRect, s: Stroke, f: graphics::item::Fill) -> GraphicItem {
    Rect(n.0, n.1, n.2, n.3, s, f)
}

fn next2graphic(n: NRectExt) -> GraphicItem {
    let r = n.0;
    match n.1 {
        NRectType::Head(head_type, head_shape) => {
            //
            let p = match head_shape {
                HeadShape::BlackHead => CADENZA_148.to_vec(),
                HeadShape::WhiteHead => CADENZA_153.to_vec(),
                HeadShape::WholeHead => CADENZA_83.to_vec(),
            };
            Path(
                PathSegments(p).inv01().move_path(r.0, SPACE_HALF + r.1),
                NoStroke,
                Fillstyle(Black),
            )
        }
        NRectType::Pause(pause_type) => {
            //
            let p = match pause_type {
                PauseShape::Whole => CADENZA_122.to_vec(),
                PauseShape::Half => CADENZA_172.to_vec(),
                PauseShape::Quarter => CADENZA_147.to_vec(),
                PauseShape::Eighth => CADENZA_165.to_vec(),
                PauseShape::Sixteenth => CADENZA_176.to_vec(),
                PauseShape::ThirtySecond => CADENZA_3.to_vec(),
            };
            let y: f32 = match pause_type {
                PauseShape::Whole => SPACE_HALF,
                PauseShape::Half => SPACE,
                PauseShape::Quarter => 3. * SPACE_HALF,
                PauseShape::Eighth => SPACE,
                PauseShape::Sixteenth => SPACE,
                PauseShape::ThirtySecond => 0.,
            };
            Path(
                PathSegments(p).inv01().move_path(r.0, r.1 + y),
                NoStroke,
                Fillstyle(Black),
            )
        }
        NRectType::Clef => {
            //
            Path(
                PathSegments(CADENZA_8.to_vec()).inv01(),
                NoStroke,
                Fillstyle(Black),
            )
        }
        NRectType::Accidental(_) => {
            //
            Path(
                PathSegments(CADENZA_64.to_vec()).inv01(),
                NoStroke,
                Fillstyle(Black),
            )
        }
        NRectType::WIP(msg) => {
            //
            println!("WIP graphic:{}", msg);
            Path(
                PathSegments(CADENZA_3.to_vec()).inv01(),
                NoStroke,
                Fillstyle(Black),
            )
        }
    }
}

fn five_lines(w: f32) -> GraphicItems {
    let mut items = GraphicItems::new();
    for i in 0..5 {
        let y = (i - 2) as f32 * SPACE;
        let line = Line(0., y, w, y, Strokestyle(NOTELINES_WIDTH, Black));
        items.push(line);
    }
    items
}
