use wasm_bindgen::prelude::*;

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

//
// Bind `console.log` manually, without the help of `web_sys`.
//
#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
    fn error(s: &str);
}

//
// Log a message to the browser console
//
#[macro_export]
macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (utils::log(&format_args!($($t)*).to_string()))
}

//
// Log an error message to the browser console
//
#[macro_export]
macro_rules! console_warning {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => {
        let message = &format_args!($($t)*).to_string();
        utils::log(&format_args!("[Warning] {}", message).to_string());
    }
}

//
// Log an error message to the browser console
//
#[macro_export]
macro_rules! console_error {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => {
        let message = &format_args!($($t)*).to_string();
        utils::log(&format_args!("[Error] {}", message).to_string());
    }
}
