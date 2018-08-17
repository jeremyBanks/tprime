//! tprime
#![feature(rust_2018_preview, rust_2018_idioms, try_from)]
#![warn(missing_docs, missing_debug_implementations)]

#[macro_use] extern crate log;
extern crate env_logger;

use std::fmt;

use rand;

use std::cmp;

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

    #[wasm_bindgen(js_name=clearCanvas)]
    pub fn clear_canvas();

    #[wasm_bindgen(js_name=drawLine)]
    pub fn draw_line(color: String, x0: usize, y0: usize, x1: usize, y1: usize);
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

mod ellipsis {
    pub fn serialize<S>(_: impl std::any::Any, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_unit_struct("…")
    }
}

/// The root application state.
#[wasm_bindgen]
#[derive(Serialize, SerDebug)]
pub struct Application {
    t: u32,
    iteration: u32,
    visited: Grid<bool>,
    #[serde(with="ellipsis")]
    path: Vec<usize2>,
    start_point: usize2,
    end_point: usize2,
    #[serde(with="ellipsis")]
    rng: rand_core::block::BlockRng<rand::prng::chacha::ChaChaCore>,
}

const WIDTH: usize = 128;
const HEIGHT: usize = 128;

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
            Range::new(WIDTH - WIDTH / 16, WIDTH).ind_sample(&mut rng),
            Range::new(0, HEIGHT / 16).ind_sample(&mut rng));

        let end_point = (
            Range::new(0, WIDTH / 16).ind_sample(&mut rng),
            Range::new(HEIGHT - HEIGHT / 16, HEIGHT).ind_sample(&mut rng)
        );

        set_text(r#"
Welcome to my blog!

It has a funny animated background.

Tprime feature rust idioms try from seed bs bs bs let. Feature rust preview rust preview rust preview rust preview rust preview.

Rust idioms try from seed bs b as u visited grid.

Preview rust idioms try from seed bs bs b as isize. Rust idioms try from seed bs bs bs b as isize. Idioms try from warn missing debug clone copy pub fn tick.

From warn missing docs missing debug clone copy pub fn clear.

Warn missing debug implementations use extern c wasm bindgen js name. Missing debug implementations use extern c wasm bindgen impl fmt use. Docs missing docs missing docs missing debug implementations use js sys.

Missing docs missing debug implementations use wasm bindgen js name clearcanvas.

Debug implementations use wasm bindgen constructor pub fn set text string.

Implementations use web sys date now as u let simple inertia.

Rand use web sys date now as usize use extern crate. Wasm bindgen use js name clearcanvas pub fn clear canvas else. Bindgen constructor pub enum direction downright down downleft direction downright direction."#);

        Self {
            t: 0,
            iteration: 0,
            visited: Grid::new(WIDTH, HEIGHT),
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

        // if current_point == self.end_point {
        //     self.iteration += 1;

        //     // start again, in the opposite direction
        //     let t = self.end_point;
        //     self.end_point = self.start_point;
        //     self.start_point = t;

        //     // clear_canvas();
        //     // self.visited = Grid::new(WIDTH, HEIGHT);

        //     return;
        // }

        for dx in vec![0, -1, 1].into_iter() {
            for dy in vec![0, -1, 1].into_iter() {
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
            let last_point = self.path[self.path.len() - 2];

            // this point has nothing to offer
            self.path.pop();

            // draw_line("rgba(30, 30, 30, 0.75)".into(), 2 + current_point.0 * 8, 2 + current_point.1 * 8, 2 + last_point.0 * 8, 2 + last_point.1 * 8);
        } else {
            let random_index = Range::new(0, free_points.len()).ind_sample(&mut self.rng);

            let mut best_index = random_index;
            let mut best_distance = 9999;

            for (i, ((x, y), _)) in free_points.iter().enumerate() {
                let simple_distance = (self.end_point.0 as isize - *x as isize).abs() + (self.end_point.1 as isize - *y as isize).abs();
                if simple_distance < best_distance {
                    best_distance = simple_distance;
                    best_index = i;
                }
            }

            let mut inertial_index = random_index;
            let mut best_inertia = 0;

            if self.path.len() >= 2 {
                let last_point = self.path[self.path.len() - 2];
                for (i, ((x, y), _)) in free_points.iter().enumerate() {
                    let simple_inertia =
                        if (last_point.0 as isize - *x as isize).abs() == 2 { 1 } else { 0 } +
                        if (last_point.1 as isize - *y as isize).abs() == 2 { 1 } else { 0 };
                    if simple_inertia > best_inertia {
                        best_inertia = simple_inertia;
                        inertial_index = i;
                    }
                }
            }

            let first_index = 0;

            let index = match self.t % 6 {
                1 | 2 => random_index,
                3 => inertial_index,
                _ => best_index,
            };
            // 
            
            let (point, direction) = free_points[index];

            let color = format!("rgb({}, {}, {})",
                0,
                (self.t as isize % 512 - 256).abs(),
                ((self.t as isize) % 192 - 96).abs()).into();
            draw_line(color, 2 + current_point.0 * 8, 2 + current_point.1 * 8, 2 + point.0 * 8, 2 + point.1 * 8);

            self.path.push(point);
            self.visited[point] = true;
        }

        // let mut text = String::new();
        // text.push_str(&format!("{:#?}\n\n", self));
        // text.push_str(&format!("{}\n\n", self.visited));
        // set_text(&text);
        self.t += 1;
    }
}
