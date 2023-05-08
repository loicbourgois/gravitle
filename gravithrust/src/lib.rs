#![cfg_attr(feature = "bench", feature(test))]
// #![warn(clippy::disallowed_types)]
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;
mod action;
mod alchemy;
mod bench;
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
#[cfg(target_arch = "wasm32")]
fn elapsed_secs_f32(instant: f64) -> f32 {
    ((now() - instant) / 1000.0) as f32
}
#[cfg(target_arch = "wasm32")]
fn now() -> f64 {
    js_sys::Date::now()
}
#[cfg(not(target_arch = "wasm32"))]
fn log(s: &str) {
    println!("{s}");
}
#[cfg(not(target_arch = "wasm32"))]
fn error(s: &str) {
    println!("{s}");
}
#[cfg(not(target_arch = "wasm32"))]
fn now() -> std::time::Instant {
    std::time::Instant::now()
}
#[cfg(not(target_arch = "wasm32"))]
fn elapsed_secs_f32(instant: std::time::Instant) -> f32 {
    instant.elapsed().as_secs_f32()
}
