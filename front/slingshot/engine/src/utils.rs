extern crate console_error_panic_hook;
pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

// #[cfg(target_arch = "wasm32")]
pub fn elapsed_secs_f32(instant: f64) -> f32 {
    ((now() - instant) / 1000.0) as f32
}
// #[cfg(target_arch = "wasm32")]
pub fn now() -> f64 {
    js_sys::Date::now()
}
