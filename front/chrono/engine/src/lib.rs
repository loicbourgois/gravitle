mod cell;
mod link;
mod math;
mod point;
mod utils;
mod world;
use crate::utils::set_panic_hook;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_f32(a: f32);
}
#[wasm_bindgen]
pub fn setup() {
    log("setup - start");
    set_panic_hook();
    log("setup - ok");
}
