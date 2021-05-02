use wasm_bindgen::prelude::*;
use web_sys::console;
use std::vec;


// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    console::log_1(&JsValue::from_str("Initializing universe"));
    Ok(())
}
const P_SIZE: usize = 2;
#[wasm_bindgen]
//#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Particle {
    x: f32,
    y: f32,
}
#[wasm_bindgen]
pub struct Universe {
    step: u32,
    particles: Vec<Particle>,
}
#[wasm_bindgen]
pub struct ClientData {
    step: u32,
}
#[wasm_bindgen]
impl ClientData {
    pub fn step(&self) -> u32 {
        return self.step
    }
}
#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        let mut particles = Vec::new();
        particles.push(Particle{
            x: 2.9,
            y: 3.7,
        });
        return Universe {
            step: 0,
            particles: particles,
        };
    }
    pub fn tick(&mut self) {
        self.step += 1;
    }
    pub fn get_client_data(&self) -> ClientData {
        return ClientData {
            step: self.step
        };
    }
    pub fn get_particles_ptr(&self) -> *const Particle {
        return self.particles.as_ptr();
    }
    pub fn get_particles_buffer_size(&self) -> usize {
        return self.particles.len() * P_SIZE;
    }
}
