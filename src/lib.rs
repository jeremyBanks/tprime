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
            None => " ",
            Up => "⇑",
            UpRight => "⇗",
            Right => "⇒",
            DownRight => "⇘",
            Down => "⇓",
            DownLeft => "⇙",
            Left => "⇐",
            UpLeft => "⇖",
        })
    }
}

impl Default for Direction {
    fn default() -> Self {
        Direction::None
    }
}

type usize2 = (usize, usize);

/// The root application state.
#[wasm_bindgen]
#[derive(Debug)]
pub struct Application {
    t: u32,
    visited: Grid<bool>,
    path_direction: Grid<Direction>,
    path: Vec<usize2>,
    start_point: usize2,
    end_point: usize2,
    rng: rand_core::block::BlockRng<rand::prng::chacha::ChaChaCore>,
}

const WIDTH: usize = 32;
const HEIGHT: usize = 64;

use rand::SeedableRng;
use rand_core::block::BlockRng;
use rand::distributions::{Range, IndependentSample};
use rand::prng::chacha::ChaChaCore;

#[wasm_bindgen]
impl Application {
    /// Instantitates everything.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {

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
            Range::new(0, WIDTH).ind_sample(&mut rng),
            Range::new(HEIGHT - HEIGHT / 4, HEIGHT).ind_sample(&mut rng));

        let end_point = (
            Range::new(0, WIDTH).ind_sample(&mut rng),
            Range::new(0, HEIGHT / 4).ind_sample(&mut rng)
        );

        Self {
            t: 0,
            visited: Grid::new(WIDTH, HEIGHT),
            path_direction: Grid::new(WIDTH, HEIGHT),
            path: vec![start_point],
            start_point,
            end_point,
            rng
        }
    }

    #[wasm_bindgen]
    /// A new frame!
    pub fn tick(&mut self) {
        // explore the path
        let current_point = self.path[self.path.len() - 1];
        let (x, y) = current_point;
        let mut adjacent_points = Vec::new();

        if current_point == self.end_point {
            return;
        }

        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let xp = (x as isize) + dx;
                let yp = (y as isize) + dy;
                if xp < 0 || xp >= (WIDTH as isize) || yp < 0 || yp >= (HEIGHT as isize) {
                    continue;
                }
                adjacent_points.push(((xp as usize, yp as usize), match (dx, dy) {
                    (-1, -1) => Direction::UpLeft,
                    (-1, 0) => Direction::Left,
                    (-1, 1) => Direction::DownLeft,
                    (1, -1) => Direction::UpRight,
                    (1, 0) => Direction::Right,
                    (1, 1) => Direction::DownRight,
                    (0, -1) => Direction::Up,
                    (0, 1) => Direction::Down,
                    _ => unreachable!(),
                }));
            }
        }
        let a = adjacent_points.len();
        let free_points: Vec<(usize2, Direction)> = adjacent_points.into_iter().filter(|(point, direction)| {
            !self.visited[*point]
        }).collect();
        // set_text(&format!("{} adjacent, {} free", a, free_points.len()));

        if free_points.len() == 0 {
            // scratch this direction
            self.path_direction[self.path[self.path.len() - 2]] = Direction::None;

            // this point has nothing to offer
            self.path.pop();
        } else {
            let index = Range::new(0, free_points.len()).ind_sample(&mut self.rng);
            let (point, direction) = free_points[index];

            self.path.push(point);
            self.visited[point] = true;
            self.path_direction[current_point] = direction;
        }

        let mut text = String::new();
        // text.push_str(&format!("{:#?}\n\n", self));
        // text.push_str(&format!("{}\n\n", self.visited));
        text.push_str(&format!("{}\n\n", self.path_direction));
        set_text(&text);
        self.t += 1;
    }
}
