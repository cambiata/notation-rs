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
        let v = vec![1, 2, 3];

        match v {
            [1, 2, 3] => println!("ok"),
            _ => println!("not ok"),
        }
    }
}

//-------------------------------------------------
