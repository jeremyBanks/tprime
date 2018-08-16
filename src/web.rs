//! tprime wasm library
#![feature(rust_2018_preview, try_from)]
#![warn(missing_docs, missing_debug_implementations)]

mod mods;
use self::mods::pathfinding;

use serde_derive::Serialize;
use serdebug::SerDebug;

use std::convert::TryFrom;

use log::{debug, error, info, log, trace, warn, Log};

use js_sys;
use rand::distributions::{Distribution, Range};
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
    color: String,
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

        Application {
            rng,
            width: u32::try_from(width).unwrap(),
            height: u32::try_from(height).unwrap(),
            render_scale: 32,
        }
    }

    pub fn tick(&mut self) -> JsValue {
        let width = self.width * self.render_scale;
        let height = self.height * self.render_scale;
        let any_running = false;

        JsValue::from_serde(&Output {
            timeout: if any_running { 0 } else { 4000 },
            width,
            height,
            lines: vec![],
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
