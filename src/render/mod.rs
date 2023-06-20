#[cfg(test)]
mod tests {

    use crate::complex::Complexes;
    use crate::prelude::*;
    use crate::quick::QCode;

    #[test]
    fn example() {
        assert_eq!(3, 2 + 1);

        let voices = QCode::voices("nv4 0 0  / nv8 2 2 2 2").unwrap();


        
        let complexes = Complexes::from_voices(&voices).unwrap();
    }
}
