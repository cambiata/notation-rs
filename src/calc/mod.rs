use crate::prelude::*;

pub fn complex_calculate_x_adjustment(upper: &RefCell<Note>, lower: &RefCell<Note>) -> Option<ComplexXAdjustment> {
    let upper = upper.borrow();
    let lower = lower.borrow();

    match [&upper.ntype, &lower.ntype] {
        [NoteType::Heads(upper_heads), NoteType::Heads(lower_heads)] => {
            let level_diff = lower_heads.top - upper_heads.bottom;

            let upper_head_width = match duration_get_headshape(&upper.duration) {
                HeadShape::BlackHead => HEAD_WIDTH_BLACK,
                HeadShape::WhiteHead => HEAD_WIDTH_WHITE,
                HeadShape::WholeHead => HEAD_WIDTH_WIDE,
            };

            let upper_dots_nr = duration_get_dots(&upper.duration);
            let upper_dots_width = upper_dots_nr as f32 * DOT_WIDTH;

            let lower_head_width = match duration_get_headshape(&lower.duration) {
                HeadShape::BlackHead => HEAD_WIDTH_BLACK,
                HeadShape::WhiteHead => HEAD_WIDTH_WHITE,
                HeadShape::WholeHead => HEAD_WIDTH_WIDE,
            };

            let lower_dots_nr = duration_get_dots(&lower.duration);
            let lower_dots_width = lower_dots_nr as f32 * DOT_WIDTH;

            if level_diff < 0 {
                // upper is lower than lower
                Some(ComplexXAdjustment::UpperRight(lower_head_width + lower_dots_width))
            } else if level_diff == 0 {
                // same level
                let same_duration = upper.duration == lower.duration;
                if same_duration {
                    None
                } else {
                    Some(ComplexXAdjustment::UpperRight(lower_head_width))
                }
            } else if level_diff == 1 {
                // lower is one lower than upper
                Some(ComplexXAdjustment::LowerRight(upper_head_width + upper_dots_width - HEADS_DIAGONAL_ADJUST))
            } else {
                // level_diff > 1
                None
            }
        }
        _ => None,
    }
}

pub fn note_get_heads_placements(note: &Note) -> Result<HeadsPlacement> {
    match note.ntype {
        NoteType::Heads(ref heads) => {
            if note.direction.is_none() {
                return Err(Generic(format!("Note {:?} has no direction", note)).into());
            }
            let dir = note.direction.unwrap();

            let levels_heads = heads.levels();
            if levels_heads.len() == 1 {
                return Ok(vec![(levels_heads[0], HeadPlacement::Center, heads.heads[0].clone())]);
            }

            //------------------------------------------------------------
            let mut result: HeadsPlacement = Vec::new();
            return match dir {
                DirUD::Up => {
                    for (idx, level_pair) in levels_heads.into_iter().rev().collect::<Vec<i8>>().windows(2).enumerate() {
                        let lower_level = level_pair[0];
                        let upper_level = level_pair[1];
                        let diff = lower_level - upper_level;
                        let head = &heads.heads[idx];
                        let upper_head = &heads.heads[idx + 1];

                        if idx == 0 {
                            result.push((lower_level, HeadPlacement::Center, head.clone()));
                            if diff < 2 {
                                result.push((upper_level, HeadPlacement::Right, upper_head.clone()));
                            } else {
                                result.push((upper_level, HeadPlacement::Center, upper_head.clone()));
                            }
                        } else {
                            let (current_level, current_placement, current_head) = &result[idx];
                            match diff {
                                0 | 1 => {
                                    if let HeadPlacement::Center = current_placement {
                                        result.push((upper_level, HeadPlacement::Right, upper_head.clone()));
                                    } else {
                                        result.push((upper_level, HeadPlacement::Center, upper_head.clone()));
                                    }
                                }
                                _ => {
                                    result.push((upper_level, HeadPlacement::Center, upper_head.clone()));
                                }
                            }
                        }
                    }
                    Ok(result)
                }
                DirUD::Down => {
                    for (idx, level_pair) in levels_heads.windows(2).enumerate() {
                        let upper_level = level_pair[0];
                        let lower_level = level_pair[1];
                        let diff = lower_level - upper_level;
                        let head = &heads.heads[idx];
                        let lower_head = &heads.heads[idx + 1];

                        if idx == 0 {
                            result.push((upper_level, HeadPlacement::Center, head.clone()));
                            if diff < 2 {
                                result.push((lower_level, HeadPlacement::Left, lower_head.clone()));
                            } else {
                                result.push((lower_level, HeadPlacement::Center, lower_head.clone()));
                            }
                        } else {
                            let (current_level, current_placement, current_head) = &result[idx];
                            match diff {
                                0 | 1 => {
                                    if let HeadPlacement::Center = current_placement {
                                        result.push((lower_level, HeadPlacement::Left, lower_head.clone()));
                                    } else {
                                        result.push((lower_level, HeadPlacement::Center, lower_head.clone()));
                                    }
                                }
                                _ => {
                                    result.push((lower_level, HeadPlacement::Center, lower_head.clone()));
                                }
                            }
                        }
                    }
                    Ok(result)
                }
            };
        }
        NoteType::Pause => Ok(Vec::new()),
        NoteType::Lyric(_) => Ok(Vec::new()),
        NoteType::Spacer(_) => Ok(Vec::new()),
        NoteType::Tpl(_, _, _, _) => Ok(Vec::new()),
        NoteType::Function(_, _, _, _, _) => Ok(Vec::new()),
        NoteType::Symbol(_) => Ok(Vec::new()),
        NoteType::ChordSymbol(_, _, _, _) => Ok(Vec::new()),
    }
}
