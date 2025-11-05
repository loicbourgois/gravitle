pub mod blueprint;
mod cell;
mod material;
mod point;
mod utils;
pub mod world;
use crate::utils::set_panic_hook;
use crate::world::World;
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
#[must_use]
#[wasm_bindgen]
pub fn setup() -> World {
    log("setup - start");
    set_panic_hook();
    log("setup - ok");
    World::new()
}
