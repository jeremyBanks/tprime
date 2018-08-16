//! tprime binary
#![feature(rust_2018_preview, rust_2018_idioms, try_from)]
#![warn(missing_docs, missing_debug_implementations)]

use env_logger;

use serde_derive::Serialize;
use serdebug::SerDebug;

mod mods;
use self::mods::square_grid::SquareGrid;

fn main() {
    env_logger::init();

    let x = SquareGrid::<bool>::new(2, 2);
    println!("hello world {}", x);
}
