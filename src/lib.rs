use std::thread::sleep;
use std::time::Duration;
use wasm_bindgen::prelude::*;

#[wasm_bindgen()]
extern "C" {
    fn render() -> ();
    fn log(string: String) -> ();
    fn error(string: String) -> ();
}

#[wasm_bindgen]
pub fn game_start() {
    init();
    loop {
        game_tick();
    }
}

fn init() {}

fn game_tick() {
    log("one tick has passed".to_string());
    sleep(Duration::from_millis(17));
}
