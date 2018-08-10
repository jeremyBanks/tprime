use log;

pub struct WebConsoleLogger;

mod web_console {
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace=console)]
        pub fn error(s: &str);
        #[wasm_bindgen(js_namespace=console)]
        pub fn warn(s: &str);
        #[wasm_bindgen(js_namespace=console)]
        pub fn info(s: &str);
        #[wasm_bindgen(js_namespace=console)]
        pub fn debug(s: &str);
    }
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
            Error => web_console::error(&record.args().to_string()),
            Warn => web_console::warn(&record.args().to_string()),
            Info => web_console::info(&record.args().to_string()),
            Debug | Trace => web_console::debug(&record.args().to_string()),
        }
    }

    fn flush(&self) {}
}
