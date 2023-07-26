#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(clippy::uninlined_format_args)]
#![allow(clippy::useless_format)]

use crate::prelude::*;
pub mod bar;
pub mod beamgroup;
pub mod calc;
pub mod complex;
pub mod core;
pub mod error;
pub mod head;
pub mod note;
pub mod part;
pub mod prelude;
pub mod qcode;
pub mod render;
pub mod types;
pub mod utils;
pub mod voice;

#[cfg(test)]
mod tests2 {
    use crate::prelude::*;
    #[test]
    fn test() {
        //
        let v = vec![24, 24];

        match v.as_slice() {
            [1, 2] => println!("1,2"),
            [1, 2, 3] => println!("1,2,3"),
            [NV4, NV4] => println!("NV4,NV4"),
            _ => println!("not ok"),
        }
    }
}

//-------------------------------------------------
