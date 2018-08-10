//! tprime binary
#![feature(rust_2018_preview, rust_2018_idioms, try_from)]
#![warn(missing_docs, missing_debug_implementations)]

use env_logger;

mod mods;
use self::mods::*;

fn main() {
    env_logger::init();

    let x = grid::Grid::<bool>::new(2, 2);
    println!("hello world {}", x);
}