use crate::{
    core::{DirUD, Rect},
    head::Head,
    heads::Heads,
    note::NoteType,
};

struct NoteRects;

impl NoteRects {
    fn get_heads_positions(heads: Heads, direction: DirUD) {
        println!("heads.level_bottom():{:?}", heads.level_bottom());

        for head in heads.items.iter().rev() {
            println!("head:{:?}", head);
        }

        // let mut result: &Vec<(&Head, i32)> =
        //     &heads.items.iter().map(|head| (head, 0)).rev().collect();

        // for res in result {
        //     println!("res:{:?}", res);
        // }

        // for pair in result.windows(2) {
        //     // println!("pair:{:?}", pair);
        //     match pair {
        //         [mut lower, mut upper] => {
        //             println!(
        //                 "lower:{:?} \tupper:{:?} \tdiff:{:?}",
        //                 lower.0.level,
        //                 upper.0.level,
        //                 lower.0.level - upper.0.level
        //             );

        //             lower.1 = 123;
        //         }
        //         _ => {}
        //     }
        // }

        // for res in result {
        //     println!("res:{:?}", res);
        // }
    }

    fn get_rects(note_type: NoteType, direction: DirUD) -> Vec<Rect> {
        match note_type {
            NoteType::Heads(heads) => Vec::new(),
            _ => Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{core::DirUD, head::HeadAttributes};

    #[test]
    fn test_get_heads_positions() {
        let heads = Heads::new(vec![
            Head::new(1, HeadAttributes { accidental: None }),
            Head::new(0, HeadAttributes { accidental: None }),
            Head::new(-2, HeadAttributes { accidental: None }),
            Head::new(4, HeadAttributes { accidental: None }),
        ]);

        let heads_posititions = NoteRects::get_heads_positions(heads, DirUD::Up);
    }
}
