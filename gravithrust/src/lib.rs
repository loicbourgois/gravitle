use wasm_bindgen::prelude::wasm_bindgen;
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
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
}
