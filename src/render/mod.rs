#[cfg(test)]
mod tests {

    use std::collections::HashMap;

    use crate::prelude::*;
    use crate::quick::QCode;

    #[test]
    fn example() {
        assert_eq!(3, 2 + 1);

        let voices = QCode::voices("nv4 0 0  / nv8 2 2 2 2").unwrap();
        let voices_beamings = beamings_from_voices(
            &voices,
            &BeamingPattern::NValues(vec![NV4]),
            &DirUAD::Auto,
            &DirUAD::Auto,
        )
        .unwrap();
        let map: HashMap<&Note, &BeamingItem<'_>> =
            get_map_note_beamings(&voices_beamings).unwrap();
        let complexes = complexes_from_voices(&voices, &map).unwrap();
    }
}
