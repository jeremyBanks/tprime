//! tprime wasm library
#![feature(rust_2018_preview, try_from)]
#![warn(missing_docs)]

mod mods;
use self::mods::pathfinding;

use serde_derive::Serialize;
use serdebug::SerDebug;

use std::convert::TryFrom;

#[allow(unused_imports)]
use log::{debug, error, info, log, trace, warn, Log};

use js_sys;
use rand::prng::chacha::ChaChaCore;
use rand::SeedableRng;
use rand_core::block::BlockRng;
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

#[derive(Serialize, SerDebug)]
struct Output {
    timeout: u32,
    width: u32,
    height: u32,
    lines: Vec<OutputLine>,
}

#[derive(Serialize, SerDebug)]
struct OutputLine {
    color: &'static str,
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
pub struct Application {
    render_scale: u32,
    width: u32,
    height: u32,

    demo_iteration: usize,
    rng: BlockRng<ChaChaCore>,

    pathfinders: Vec<pathfinding::AStarPathfinder>,
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
        // set_text("A*");
        debug!("Logging to web console at level {:?}.", log::max_level());

        let timestamp_js = js_sys::Date::now();

        let timestamp = timestamp_js as u64;
        let mut seed = [0u8; 32];
        for i in 0..8 {
            seed[i] = (timestamp >> (8 * i)) as u8;
        }
        debug!("Seeding RNG with {:?} from timestamp {}.", seed, timestamp);
        let rng = BlockRng::new(ChaChaCore::from_seed(seed));

        let width = 64;
        let height = 64;

        Application {
            rng,
            demo_iteration: 0,
            width: u32::try_from(width).unwrap(),
            height: u32::try_from(height).unwrap(),
            render_scale: 32,
            pathfinders: vec![pathfinding::AStarPathfinder::demo(0)],
        }
    }

    pub fn tick(&mut self) -> JsValue {
        let width = self.width * self.render_scale;
        let height = self.height * self.render_scale;
        let mut any_working = false;

        let scale = self.render_scale;
        let scale_f64 = scale as f64;
        let scale_point = |(x, y): &(usize, usize)| -> (f64, f64) {
            let xp = ((*x as u32) * scale + (scale / 2)) as f64;
            let yp = ((*y as u32) * scale + (scale / 2)) as f64;
            (xp, yp)
        };

        let mut lines = vec![];

        for (i, pathfinder) in self.pathfinders.iter_mut().enumerate() {
            if pathfinder.working() {
                any_working = true;

                pathfinder.get_path();

                // for _ in 0..128 {
                //     pathfinder.step();
                // }

                if !pathfinder.working() {
                    any_working = false;
                }
            }

            for ((x, y), info) in pathfinder.data().iter() {
                use self::mods::pathfinding::AStarCellState::*;
                let (xp, yp) = scale_point(&(x, y));
                match info.state() {
                    Blocked => {
                        lines.push(OutputLine {
                            color: "rgba(192, 0, 64, 1.0)",
                            width: 0.3 * scale_f64,
                            points: vec![
                                (xp - scale_f64 / 3., yp - scale_f64 / 3.),
                                (xp + scale_f64 / 3., yp - scale_f64 / 3.),
                                (xp + scale_f64 / 3., yp + scale_f64 / 3.),
                                (xp - scale_f64 / 3., yp + scale_f64 / 3.),
                                (xp - scale_f64 / 3., yp - scale_f64 / 3.),
                            ],
                        });
                    }
                    VisitedFrom(position) => {
                        lines.push(OutputLine {
                            color: "rgba(192, 192, 64, 1.0)",
                            width: 0.125 * (scale as f64),
                            points: vec![scale_point(&position), (xp, yp)],
                        });
                    }
                    Free => {}
                }
            }

            lines.push(OutputLine {
                color: "rgba(64, 192, 64, 1.0)",
                width: 0.5 * (scale as f64),
                points: pathfinder
                    .peek_path()
                    .unwrap()
                    .iter()
                    .map(scale_point)
                    .collect(),
            });

            if !pathfinder.working() {
                self.demo_iteration += 1;
                *pathfinder = pathfinding::AStarPathfinder::demo(self.demo_iteration);
            }
        }

        JsValue::from_serde(&Output {
            timeout: if any_working { 250 } else { 250 },
            width,
            height,
            lines,
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
