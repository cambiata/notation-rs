#![allow(unused_imports)]

use notation_rs::prelude::*;
use notation_rs::quick::QCode;

fn main() {
    println!("Hello, Example 1!");
    let notes = QCode::notes("0 1,2 nv2 -4 p ").unwrap();
    dbg!(notes);
}
