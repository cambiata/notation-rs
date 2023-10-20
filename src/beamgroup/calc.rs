use crate::prelude::*;

pub fn get_beamgroups_nobeams(notes: &Notes) -> Result<Vec<Beamgroup>> {
    let mut result = vec![];

    for note in notes.items.iter() {
        let beamgroup = Beamgroup::new(vec![note.clone()]);
        result.push(beamgroup);
    }

    Ok(result)
}

pub fn get_beamgroups_nvalues(notes: &Notes, values: &[usize]) -> Result<Vec<Beamgroup>> {
    let mut result = vec![];

    let mut value_cycle: Vec<(usize, usize)> = vec![];
    let mut vpos_start: usize = 0;
    let mut vpos_end: usize = 0;
    let mut idx: usize = 0;

    // create value cycle of sufficient length
    while vpos_end <= notes.duration {
        vpos_start = vpos_end;
        let value = values[idx % values.len()];
        // let value_to_push = values[(idx % values.len()) as usize];
        vpos_end = vpos_start + value;
        value_cycle.push((vpos_start, vpos_end));
        idx += 1;
    }

    let mut note_group: Vec<Rc<RefCell<Note>>> = vec![];

    let mut cycle_iter = value_cycle.iter();
    let mut cycle = cycle_iter.next().unwrap();
    let mut cycle_start = cycle.0;
    let mut cycle_end = cycle.1;

    for (idx, item) in notes.items.iter().enumerate() {
        let note = item.borrow();

        while note.position >= cycle_end {
            // println!("note.end > *cycle_end - next value cycle");
            cycle = cycle_iter.next().unwrap();
            cycle_start = cycle.0;
            cycle_end = cycle.1;
        }

        match [note.end_position <= cycle_end, note.is_beamable()] {
            [true, true] => {
                note_group.push(item.clone());
                // println!("Ryms, Beamabale {}", note_group.len());
                if note.end_position == cycle_end {
                    // println!("Avsluta denna grupp");
                    let beamgroup = Beamgroup::new(note_group);
                    result.push(beamgroup);
                    note_group = vec![];
                }
            }

            [true, false] | [false, true] | [false, false] => {
                // println!("Ryms, Not Beamabale {} {}", note.end, cycle_end);
                if note_group.len() > 0 {
                    // println!("Avsluta förra grupp");
                    let beamgroup = Beamgroup::new(note_group);
                    result.push(beamgroup);
                    note_group = vec![];
                }
                let beamgroup = Beamgroup::new(vec![item.clone()]);
                result.push(beamgroup);
            }
        }

        if idx == notes.items.len() - 1 && note_group.len() > 0 {
            // println!("FINALISERA denna grupp");
            let beamgroup = Beamgroup::new(note_group);
            result.push(beamgroup);
            note_group = vec![];
        }
    }

    Ok(result)
}
