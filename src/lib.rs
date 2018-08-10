//! tprime
#![feature(rust_2018_preview, rust_2018_idioms, try_from)]
#![warn(missing_docs, missing_debug_implementations)]

use std::fmt;

use rand;

use wasm_bindgen::prelude::wasm_bindgen;
use js_sys;
use web_sys;

use serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serdebug;

use rand;

mod grid;
mod history;

use self::grid::Grid;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name=setText)]
    pub fn set_text(s: &str);
}

/// The eight directions between neighbouring cells.
#[derive(Debug, Clone, Copy)]
pub enum Direction {
    None,
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Direction::*;
        write!(f, "{}", match self {
            None => "·",
            Up => "↑",
            UpRight => "↗",
            Right => "→",
            DownRight => "↘",
            Down => "↓",
            DownLeft => "↙",
            Left => "←",
            UpLeft => "↖",
        })
    }
}

impl Default for Direction {
    fn default() -> Self {
        Direction::None
    }
}

/// The root application state.
#[wasm_bindgen]
#[derive(Debug)]
pub struct Application {
    t: u32,
    final_path: Grid<Direction>,
}

const WIDTH: usize = 16;
const HEIGHT: usize = 16;

#[wasm_bindgen]
impl Application {
    /// Instantitates everything.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            t: 0,
            final_path: Grid::new(16, 16),
        }
    }

    #[wasm_bindgen]
    /// A new frame!
    pub fn tick(&mut self) {
        use rand::SeedableRng;
        use rand_core::block::BlockRng;
        use rand::distributions::{Range, IndependentSample};
        use rand::prng::chacha::ChaChaCore;

        let b = js_sys::Date::now() as u64;
        let bs = {
            let mut bs = [0; 32];
            bs[0] = b as u8;
            bs[1] = (b >> 8) as u8;
            bs[2] = (b >> 12) as u8;
            bs[3] = (b >> 16) as u8;
            bs
        };
        let mut rng = BlockRng::new(ChaChaCore::from_seed(bs));

        let start_point = (
            Range::new(1, WIDTH / 2 - 1).ind_sample(&mut rng),
            Range::new(1, HEIGHT / 2 - 1).ind_sample(&mut rng));
        let end_point = (
            Range::new(1 + WIDTH - WIDTH / 2, WIDTH - 1).ind_sample(&mut rng),
            Range::new(1 + HEIGHT - HEIGHT / 2, HEIGHT - 1).ind_sample(&mut rng));        

        self.final_path[start_point] = Direction::Up;
        self.final_path[end_point] = Direction::Down;

        let mut text = String::new();
        text.push_str(&format!("{:#?}\n\n", self));
        text.push_str(&format!("{}\n\n", self.final_path));
        set_text(&text);
        self.t += 1;
    }
}
