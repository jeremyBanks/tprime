//! tprime wasm library
#![feature(rust_2018_preview, rust_2018_idioms, try_from)]
#![warn(missing_docs, missing_debug_implementations)]

mod mods;
use self::mods::*;

use serde;
use serde_derive::Serialize;
#[macro_use]
use serde_derive::serialize;
use serdebug::SerDebug;

use wasm_bindgen::prelude::{wasm_bindgen, JsValue};
use rand;
use js_sys;

use rand_core::block::BlockRng;
use rand::prng::chacha::ChaChaCore;

use std::fmt;


#[derive(Serialize)]
struct Output {
    text: String,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name=rustSetOutput)]
    fn set_output_jsvalue(output: &JsValue);
}

fn set_output(output: &Output) {
    set_output_jsvalue(&JsValue::from_serde(output).unwrap());
}

/// The root application state, exposed to JavaScript.
#[wasm_bindgen]
pub struct Application {
    t: u32,
    #[serde(with = "mods::ellipsis_serializer")]
    rng: BlockRng<ChaChaCore>,
}


#[wasm_bindgen]
impl Application {
    /// Instantitates everything.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let timestamp = js_sys::Date::now() as u32;
        let time_bytes = (0..4).into_iter().map(|i| (timestamp >> (i * 8)) as u8).collect();
        let rng = BlockRng::new(ChaChaCore::from_seed(time_bytes));

        Self {
            t: 0,

        }
    }

    pub fn tick() -> JsValue {
        Output {
            
        }.into_serde().unwrap()
    }
}