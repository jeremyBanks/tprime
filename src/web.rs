//! tprime wasm library
#![feature(rust_2018_preview, rust_2018_idioms, try_from)]
#![warn(missing_docs, missing_debug_implementations)]

mod mods;
use self::mods::*;

use serde_derive::Serialize;
use serdebug::SerDebug;

use log::{Log, log, trace, debug, info, warn, error};

use wasm_bindgen::prelude::{wasm_bindgen, JsValue};
use js_sys;
use rand_core::block::BlockRng;
use rand::SeedableRng;
use rand::prng::chacha::ChaChaCore;


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
    #[serde(with = "mods::ellipsis_serializer")]
    rng: BlockRng<ChaChaCore>,
}

const line_scale: u32 = 8;
const width: u32 = 64;
const height: u32 = 64;

static LOGGER: &'static (dyn log::Log + 'static) = &WebConsoleLogger;

#[wasm_bindgen]
impl Application {
    /// Instantitates everything.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        log::set_logger(LOGGER);
        log::set_max_level(log::LevelFilter::Trace);

        set_title("t'");
        set_text(&format!("Logging to web console at level {}.", log::max_level()));

        let timestamp = js_sys::Date::now() as u32;
        let mut seed = [0u8; 32];
        seed[0] = (timestamp >> (8 * 0)) as u8;
        seed[1] = (timestamp >> (8 * 1)) as u8;
        seed[2] = (timestamp >> (8 * 2)) as u8;
        seed[3] = (timestamp >> (8 * 3)) as u8;
        let rng = BlockRng::new(ChaChaCore::from_seed(seed));

        Application {
            rng
        }
    }

    pub fn tick(&mut self) -> JsValue {
        debug!("tick!");

        JsValue::from_serde(&Output {
            timeout: 1000,
            width: width * line_scale,
            height: height * line_scale,
            lines: vec![
                OutputLine {
                    color: "#4B8",
                    width: 3.5,
                    points: vec![
                        (0., 0.),
                        (100., 100.),
                        (0., 300.),
                    ]
                }
            ],
        }).unwrap()
    }
}


pub struct WebConsoleLogger;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=console, js_name=error)]
    pub fn js_error(s: &str);
    #[wasm_bindgen(js_namespace=console, js_name=warn)]
    pub fn js_warn(s: &str);
    #[wasm_bindgen(js_namespace=console, js_name=info)]
    pub fn js_info(s: &str);
    #[wasm_bindgen(js_namespace=console, js_name=debug)]
    pub fn js_debug(s: &str);
}

impl log::Log for WebConsoleLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        use log::Level::*;
        match record.level() {
            Error => js_error(&record.args().to_string()),
            Warn => js_warn(&record.args().to_string()),
            Info => js_info(&record.args().to_string()),
            Debug | Trace => js_debug(&record.args().to_string()),
        }
    }

    fn flush(&self) {}
}
