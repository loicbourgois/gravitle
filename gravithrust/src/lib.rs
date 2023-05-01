#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;
mod action;
mod alchemy;
mod blueprint;
mod gravithrust;
mod gravithrust_check_job;
mod gravithrust_tick;
mod grid;
mod job;
mod kind;
mod kind_generated;
mod link;
mod math;
mod particle;
mod ship;
mod test;
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
}
#[cfg(not(target_arch = "wasm32"))]
fn log(s: &str) {
    println!("{s}");
}
#[cfg(not(target_arch = "wasm32"))]
fn error(s: &str) {
    println!("{s}");
}
