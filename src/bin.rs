//! tprime binary
#![feature(rust_2018_preview, try_from)]
#![warn(missing_docs)]

use env_logger;

mod mods;
use self::mods::pathfinding;

fn main() {
    env_logger::init();

    let mut pathfinder = pathfinding::AStarPathfinder::default();
    let path = pathfinder.get_path();

    println!("AStarPathfinder::default().find_path == {:?}", path);
}

#[test]
fn test_main() {
    main();
}
