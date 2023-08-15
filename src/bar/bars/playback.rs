use std::collections::BTreeMap;

use crate::prelude::*;

impl Bars {
    pub fn calc_playback() {}
}

fn setup_level_to_midinote_maps(clef: Clef, key: Key) -> (BTreeMap<i8, i8>, BTreeMap<i8, KeymapSign>) {
    const LEVEL_LIMIT: i8 = 10;
    let mut levels_notes: BTreeMap<i8, i8> = BTreeMap::new();
    let mut levels_keymapsign: BTreeMap<i8, KeymapSign> = BTreeMap::new();
    let dia: Vec<i8> = vec![0, 2, 4, 5, 7, 9, 11];

    let clef_adjust = match clef {
        Clef::G => 6,
        Clef::F => -6,
        Clef::C => 0,
    };

    for lev in -LEVEL_LIMIT..=LEVEL_LIMIT {
        let inv = -lev + clef_adjust;
        let oct = (inv + 35) / 7;
        let stam = (inv + 49) % 7;
        let note = oct * 12 + dia[stam as usize];

        println!("{lev} {note} {stam}");
        levels_notes.insert(lev, note);

        let mut sign = KeymapSign::Natural;
        match &key {
            Key::Sharps(sharp_count) => {
                if (*sharp_count as i8 >= 1) && (stam == 3) {
                    sign = KeymapSign::Sharp
                };
            }
            Key::Open => {}
            Key::Flats(flat_count) => {
                if (*flat_count as i8 >= 1) && (stam == 6) {
                    sign = KeymapSign::Flat
                };
                if (*flat_count as i8 >= 2) && (stam == 2) {
                    sign = KeymapSign::Flat
                };
            }
            Key::Naturals(_) => todo!(),
        }
        // dbg!(lev, sign);
        levels_keymapsign.insert(lev, sign.clone());
    }

    for lev in -LEVEL_LIMIT..=LEVEL_LIMIT {
        println!("{lev} {:?} {:?}", levels_notes[&lev], levels_keymapsign[&lev]);
    }

    (levels_notes, levels_keymapsign)
}

#[cfg(test)]
mod tests2 {
    use super::setup_level_to_midinote_maps;
    use crate::prelude::*;
    use std::collections::BTreeMap;

    #[test]
    fn example() {
        setup_level_to_midinote_maps(Clef::F, Key::Flats(2));
    }
}

#[derive(Debug, Clone, Copy)]
enum KeymapSign {
    Sharp = 1,
    Natural = 0,
    Flat = -1,
}
