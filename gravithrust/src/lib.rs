use wasm_bindgen::prelude::wasm_bindgen;
mod blueprint;
mod gravithrust;
mod gravithrust_tick;
mod grid;
mod kind;
mod link;
mod math;
mod particle;
mod ship;
mod test;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
