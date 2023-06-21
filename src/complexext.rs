use crate::complex::Complex;
use crate::core::DurationTools;
use crate::note::NoteType;
use crate::prelude::*;

#[derive(Debug)]
pub struct ComplexExt<'a> {
    pub complex: &'a Complex<'a>,
    pub notes_overlap: ComplexNotesOverlap,
}

impl<'a> ComplexExt<'a> {
    pub fn new(complex: &'a Complex<'a>) -> Self {
        Self {
            complex,
            notes_overlap: Self::get_complex_note_overlap_type(complex),
        }
    }

    pub fn get_complex_note_overlap_type(complex: &'a Complex) -> ComplexNotesOverlap {
        match complex.ctype {
            crate::complex::ComplexType::OneBarpause(_) => ComplexNotesOverlap::None,
            crate::complex::ComplexType::TwoBarpauses(_, _) => ComplexNotesOverlap::None,
            crate::complex::ComplexType::OneNote(_, _) => ComplexNotesOverlap::None,
            crate::complex::ComplexType::BarpauseNote(_, _) => ComplexNotesOverlap::None,
            crate::complex::ComplexType::NoteBarpause(_, _) => ComplexNotesOverlap::None,
            crate::complex::ComplexType::TwoNotes(upper, lower) => {
                let overlap = match [&upper.ntype, &lower.ntype] {
                    [NoteType::Heads(upper_heads), NoteType::Heads(lower_heads)] => {
                        let level_diff =
                            lower_heads.get_level_top() - upper_heads.get_level_bottom();

                        let upper_head_width = match DurationTools::get_headtype(upper.duration) {
                            crate::head::HeadType::NormalHead => OVERLAP_NORMAL_HEAD,
                            crate::head::HeadType::WideHead => OVERLAP_WIDE_HEAD,
                        };

                        if level_diff < 0 {
                            ComplexNotesOverlap::UnderRight(upper_head_width + OVERLAP_SPACE)
                        } else if level_diff == 0 {
                            ComplexNotesOverlap::UnderRight(upper_head_width)
                        } else if level_diff == 1 {
                            ComplexNotesOverlap::UnderRight(
                                upper_head_width + OVERLAP_DIAGONAL_SPACE,
                            )
                        } else {
                            ComplexNotesOverlap::None
                        }
                    }
                    _ => ComplexNotesOverlap::None,
                };
                overlap
            }
        }
    }
}

#[derive(Debug)]
pub enum ComplexNotesOverlap {
    None,
    UnderRight(f32),
    LowerRight(f32),
}

#[cfg(test)]
mod tests {
    use crate::complex::complexes_from_voices;
    use crate::complex::Complex;
    use crate::complex::{self};
    use crate::complexext::ComplexExt;
    use crate::prelude::*;
    use crate::quick::QCode;

    #[test]
    fn example() {
        let voices = QCode::voices("0 / 1").unwrap();
        let complexes = complexes_from_voices(&voices).unwrap();
        let complex_ext = ComplexExt::new(&complexes[0]);

        let notes = QCode::notes("0 -1").unwrap();
        let complex = Complex::new(
            0,
            0,
            complex::ComplexType::TwoNotes(&notes.items[0], &notes.items[1]),
        );
        let ext = ComplexExt::new(&complex);
        let ext_type = ext.notes_overlap;
        println!("ext_type:{:?}", ext_type);
        // dbg!(complex.ge);
    }
}

pub const OVERLAP_NORMAL_HEAD: f32 = 1.0;
pub const OVERLAP_WIDE_HEAD: f32 = 1.5;
pub const OVERLAP_SPACE: f32 = 0.2;
pub const OVERLAP_DIAGONAL_SPACE: f32 = -0.5;
