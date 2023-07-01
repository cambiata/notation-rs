#![allow(unused_imports)]

use std::collections::HashMap;

use notation_rs::prelude::*;
use notation_rs::quick::QCode;

use graphics::item::Fill::Fillstyle;
use graphics::prelude::*;

fn main() -> notation_rs::prelude::Result<()> {
    let voices = QCode::voices("nv1 0 / nv4 1").unwrap();

    let voices_beamings = beamings_from_voices(
        &voices,
        &BeamingPattern::NValues(vec![NV4]),
        &DirUAD::Auto,
        &DirUAD::Auto,
    )?;

    let note_beamings_map: HashMap<&Note, &BeamingItem<'_>> =
        get_map_note_beamings(&voices_beamings)?;
    let complexes = complexes_from_voices(&voices, &note_beamings_map)?;

    // let items = GraphicItems::new();

    let clef = PathSegments(CADENZA_8.to_vec()).inv01();
    let lines = PathSegments(CADENZA_31.to_vec())
        .inv01()
        .move_path(0., 25.1);

    // items.push(Rect(0.0, -5.0, 10.0, 30.0, NoStroke, Fillstyle(Red)));

    let mut items = GraphicItems::new();
    items.extend(five_lines(200.));

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
        let x = (idx + 1) as f32 * 80.;
        println!("x:{}", x);
        complex_items = complex_items.move_items(x, 0.);
        items.extend(complex_items);
    }

    let svg = SvgBuilder::new().build(items).unwrap();
    std::fs::write(".test.svg", svg)?;

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
