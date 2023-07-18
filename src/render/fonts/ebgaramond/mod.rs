use std::collections::HashMap;

pub const GLYPH_HEIGHT: f32 = 705.0;

pub fn glyph_widths(c: char) -> f32 {
    match c {
        'a' => 398 as f32,
        'b' => 480 as f32,
        'c' => 372 as f32,
        'A' => 697 as f32,
        'B' => 547 as f32,
        'C' => 655 as f32,
        _ => 100.0,
    }
}
