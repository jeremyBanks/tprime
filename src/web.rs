//! tprime wasm library
#![feature(rust_2018_preview, rust_2018_idioms, try_from)]
#![warn(missing_docs, missing_debug_implementations)]

mod mods;
use self::mods::*;

use serde_derive::Serialize;
use serdebug::SerDebug;

use std::convert::TryFrom;

use log::{debug, error, info, log, trace, warn, Log};

use self::mods::grid::Grid;

use js_sys;
use rand::distributions::{Distribution, Range};
use rand::prng::chacha::ChaChaCore;
use rand::SeedableRng;
use rand_core::block::BlockRng;
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

#[derive(Serialize, SerDebug)]
struct Output<'a> {
    timeout: u32,
    width: u32,
    height: u32,
    lines: Vec<OutputLine<'a>>,
}

#[derive(Serialize, SerDebug)]
struct OutputLine<'a> {
    color: &'a str,
    width: f64,
    points: Vec<(f64, f64)>,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name=setTitle)]
    fn set_title(output: &str);

    #[wasm_bindgen(js_name=setStyle)]
    fn set_style(output: &str);

    #[wasm_bindgen(js_name=setText)]
    fn set_text(output: &str);
}

/// The root application state, exposed to JavaScript.
#[wasm_bindgen]
#[derive(Serialize, SerDebug)]
pub struct Application {
    render_scale: u32,
    width: u32,
    height: u32,

    #[serde(with = "mods::ellipsis_serializer")]
    walkers:
        Vec<mods::walker::Walker<(usize, usize), square_grid::SquareGrid<mods::walker::NodeInfo>>>,

    #[serde(with = "mods::ellipsis_serializer")]
    rng: BlockRng<ChaChaCore>,
}

static LOGGER: &'static (dyn log::Log + 'static) = &WebConsoleLogger;

#[wasm_bindgen]
impl Application {
    /// Instantitates everything.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        log::set_logger(LOGGER);
        log::set_max_level(log::LevelFilter::Trace);

        set_title("t′");
        debug!("Logging to web console at level {:?}.", log::max_level());

        let timestamp_js = js_sys::Date::now();

        let timestamp = timestamp_js as u64;
        let mut seed = [0u8; 32];
        for i in 0..8 {
            seed[i] = (timestamp >> (8 * i)) as u8;
        }
        debug!("Seeding RNG with {:?} from timestamp {}.", seed, timestamp);
        let rng = BlockRng::new(ChaChaCore::from_seed(seed));

        let width = 32;
        let height = 32;

        type StrategyFn = fn(
            neighbours: Vec<(usize, usize)>,
            target: (usize, usize),
            grid: &square_grid::SquareGrid<walker::NodeInfo>,
        ) -> Vec<(usize, usize)>;

        let greedy: StrategyFn = |neighbours, target, grid| {
            let mut min_distance = u32::max_value();
            for neighbour in neighbours.iter() {
                let distance = grid.min_distance(*neighbour, target);
                if distance < min_distance {
                    min_distance = distance;
                }
            }
            neighbours
                .into_iter()
                .filter(|neighbour| min_distance == grid.min_distance(*neighbour, target))
                .collect()
        };

        let mindless: StrategyFn = |neighbours, _, _| neighbours;

        let clockwise: StrategyFn = |neighbours, _, _| neighbours.into_iter().take(1).collect();

        let sixteenth = width.min(height) / 16;

        let grid_with_hole = || {
            let mut grid = square_grid::SquareGrid::<walker::NodeInfo>::new(width, height);

            for x in (width / 2 - sixteenth * 6)..=(width / 2 + sixteenth * 6) {
                for y in 0..=(height / 2 + sixteenth * 5) {
                    if x < width / 2 + sixteenth * 5 && y < height / 2 + sixteenth * 4 {
                        continue;
                    }
                    grid[(x, y)].visited = true;
                }
            }

            grid
        };

        let walkers = vec![
            walker::Walker::new(
                grid_with_hole(),
                (sixteenth, sixteenth),
                (width - sixteenth - 1, height - sixteenth - 1),
                clockwise,
            ),
            walker::Walker::new(
                grid_with_hole(),
                (sixteenth, sixteenth),
                (width - sixteenth - 1, height - sixteenth - 1),
                mindless,
            ),
            walker::Walker::new(
                grid_with_hole(),
                (sixteenth, sixteenth),
                (width - sixteenth - 1, height - sixteenth - 1),
                greedy,
            ),
            walker::Walker::new(
                grid_with_hole(),
                (sixteenth, sixteenth),
                (width - sixteenth - 1, height - sixteenth - 1),
                greedy,
            ),
            walker::Walker::new(
                grid_with_hole(),
                (sixteenth, sixteenth),
                (width - sixteenth - 1, height - sixteenth - 1),
                greedy,
            ),
        ];

        Application {
            rng,
            walkers,
            width: u32::try_from(width).unwrap(),
            height: u32::try_from(height).unwrap(),
            render_scale: 32,
        }
    }

    pub fn tick(&mut self) -> JsValue {
        let width = self.width * self.render_scale;
        let height = self.height * self.render_scale;
        let mut any_running = false;

        for ref mut walker in self.walkers.iter_mut() {
            let warp_factor = 1;
            for _ in 0..(warp_factor - 1) {
                walker.step(&mut self.rng);
            }

            let running = walker.step(&mut self.rng);
            if running {
                any_running = true;
            }
        }

        JsValue::from_serde(&Output {
            timeout: if any_running { 0 } else { 4000 },
            width,
            height,
            lines: self
                .walkers
                .iter()
                .enumerate()
                .map(|(i, walker)| OutputLine {
                    color: [
                        "rgba(0, 0, 100, 0.875)",
                        "rgba(100, 0, 0, 0.875)",
                        "rgba(200, 100, 50, 0.875)",
                        "rgba(100, 50, 200, 0.875)",
                        "rgba(50, 200, 100, 0.875)",
                    ][i % 5],
                    width: 0.5 * (self.render_scale as f64),
                    points: walker
                        .current_path()
                        .iter()
                        .map(|(x, y)| {
                            let xp = ((*x as u32) * self.render_scale) as f64;
                            let yp = ((*y as u32) * self.render_scale) as f64;
                            (xp, yp)
                        }).collect(),
                }).collect(),
        }).unwrap()
    }
}

#[derive(Debug)]
struct WebConsoleLogger;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=console, js_name=error)]
    pub fn js_error(a: &str, b: &str, c: &str);
    #[wasm_bindgen(js_namespace=console, js_name=warn)]
    pub fn js_warn(a: &str, b: &str, c: &str);
    #[wasm_bindgen(js_namespace=console, js_name=info)]
    pub fn js_info(a: &str, b: &str, c: &str);
    #[wasm_bindgen(js_namespace=console, js_name=debug)]
    pub fn js_debug(a: &str, b: &str, c: &str);
}

impl log::Log for WebConsoleLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let a = format!("%c[{}]", record.metadata().target());
        let b = "font-weight: bold;";
        let c = record.args().to_string();

        use log::Level::*;
        match record.level() {
            Error => js_error(&a, &b, &c),
            Warn => js_warn(&a, &b, &c),
            Info => js_info(&a, &b, &c),
            Debug | Trace => js_debug(&a, &b, &c),
        }
    }

    fn flush(&self) {}
}
