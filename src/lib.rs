use std::panic::set_hook;
use wasm_bindgen::prelude::*;

#[wasm_bindgen()]
extern "C" {
    fn render() -> ();
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: String);
    #[wasm_bindgen(js_namespace=console)]
    fn error(s: String);
}

#[wasm_bindgen]
pub fn game_start() {
    init();
}

fn init() {
    set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
pub fn game_tick() {}
