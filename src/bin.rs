//! tprime binary
#![feature(rust_2018_preview, try_from)]
#![warn(missing_docs, missing_debug_implementations)]

use env_logger;

use serde_derive::Serialize;
use serdebug::SerDebug;

mod mods;
use self::mods::pathfinding;

fn main() {
    env_logger::init();
    println!("hello world");
}
