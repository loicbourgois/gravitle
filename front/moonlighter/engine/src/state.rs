use crate::wasm_bindgen;


#[wasm_bindgen]
#[repr(u32)]
#[derive(PartialEq, Copy, Clone)]
pub enum State {
    Default = 0,
    ReadyEmpty = 1,
    LoadingMoonDust = 2,
    Pressing = 3,
    Releasing = 4,
    ReadyFull = 5,
    ReadyStart = 6,
    ReadyEnd = 7,
    MovingAway = 8,
    MovingBack = 9,
}
