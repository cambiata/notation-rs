use crate::prelude::NRect;
use crate::{prelude::*, render::fonts::ebgaramond::GLYPH_HEIGHT, types::some_cloneables::SomeCloneablePairs};
use std::cell::{Ref, RefMut};

pub fn get_head_x_adjustment(data: &RItemBeamData) -> f32 {
    let adjustment_x: f32 = if let Some(adjustment_x) = data.adjustment_x {
        match data.lower_voice {
            false => match adjustment_x {
                ComplexXAdjustment::UpperRight(upper_right) => upper_right,
                _ => 0.0,
            },
            true => match adjustment_x {
                ComplexXAdjustment::LowerRight(lower_right) => lower_right,
                _ => 0.0,
            },
        }
    } else {
        0.0
    };

    let head_x = match data.direction {
        DirUD::Down => 0.0 + STEM_WIDTH / 2.0,
        DirUD::Up => data.head_width - STEM_WIDTH / 2.0,
    };
    adjustment_x + head_x
}

pub fn add_flag(data: &RItemBeamData) -> Option<NRectExt> {
    // Add actual flag rectangle
    // The spcing rectangle is added
    if let BeamType::None = duration_to_beamtype(&data.duration) {
        return None;
    }

    let first_tip_y = (data.tip_level * SPACE_HALF) + (STEM_LENGTH * SPACE_HALF) * data.direction.sign();
    let rect_x = match data.direction {
        DirUD::Up => get_head_x_adjustment(data) + STEM_WIDTH_HALF,
        DirUD::Down => get_head_x_adjustment(data) + STEM_WIDTH - 2.0,
    };
    let rect_y = match data.direction {
        DirUD::Up => first_tip_y,
        DirUD::Down => first_tip_y - FLAG_RECT_HEIGHT,
    };

    let rect = NRect::new(rect_x, rect_y, FLAG_RECT_WIDTH, FLAG_RECT_HEIGHT);
    let nrect = NRectExt::new(rect, NRectType::Flag(duration_to_beamtype(&data.duration), data.direction));
    Some(nrect)
}

pub fn qitem(x: f32, w: f32, dur: Duration) -> Option<Rc<RefCell<RItem>>> {
    // Some(Rc::new(RefCell::new(RItem::new(
    //     vec![NRect::new(x, 0.0, w, 10.0)],
    //     dur,
    // ))))

    Some(Rc::new(RefCell::new(RItem::new_with_nrectsext(vec![NRect::new(x, 0.0, w, 10.0)], dur))))
}

pub fn xitem(x: f32, w: f32, h: f32, dur: Duration) -> Option<Rc<RefCell<RItem>>> {
    // Some(Rc::new(RefCell::new(RItem::new(
    //     vec![NRect::new(x, 0.0, w, h)],
    //     dur,
    // ))))
    Some(Rc::new(RefCell::new(RItem::new_with_nrectsext(vec![NRect::new(x, 0.0, w, h)], dur))))
}

pub fn r10() -> Vec<NRect> {
    vec![NRect::new(0.0, 0.0, 10.0, 10.0)]
}

pub fn r20() -> Vec<NRect> {
    vec![NRect::new(0.0, 0.0, 10.0, 20.0)]
}
