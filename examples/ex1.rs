#![allow(unused_imports)]

use notation_rs::prelude::*;
use notation_rs::quick::QCode;

use graphics::prelude::*;

fn main() -> notation_rs::prelude::Result<()> {
    let notes = QCode::notes("0 1,2 nv2 -4 p ").unwrap();
    dbg!(notes);

    let clef = PathSegments(CADENZA_8.to_vec()).scale_path(0.1, -0.1);
    let lines = PathSegments(CADENZA_31.to_vec())
        .scale_path(0.1, -0.1)
        .move_path(0.0, 25.0);
    let items = GraphicItems(vec![
        // Rect(0., 0., 50., 50., Strokestyle(5., Lime), Fillstyle(Blue)),
        // Ellipse(50., 0., 50., 50., Strokestyle(10., Purple), Fillstyle(Red)),
        // Line(0., 50., 100., 0., Strokestyle(5., Red)),
        Path(lines, NoStroke, Fillstyle(Black)),
        Path(clef, NoStroke, Fillstyle(Black)),
    ]);
    let svg = SvgBuilder::new().build(items).unwrap();
    std::fs::write(".test.svg", svg)?;
    Ok(())
}
