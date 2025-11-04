use crate::point::Point;
use crate::wasm_bindgen;
#[wasm_bindgen]
#[repr(C)] // https://doc.rust-lang.org/nomicon/other-reprs.html#reprc
pub struct Cell {
    // pub idx: u32,
    // pub diameter: f32,
    // position
    pub p: Point,
    // // previous position
    // pub pp: Point,
    // // new position
    // pub np: Point,
    // // delta position
    // pub dp: Point,
    // // direction in which the cell points
    // // away from neighbours (tbc)
    // pub direction: Point,
    // // delta velocity
    // pub dv: Point,
    // pub link_response: Point,
    // pub collision_response: Point,
    // pub collision_response_count: u32,
    // pub activated: u32,
    // pub activated_previous: u32,
    pub kind: u32,
    // pub user_kind: UserKind,
    // pub padding: u32,
}
#[wasm_bindgen]
impl Cell {
    pub fn new(x: f32, y: f32, kind: u32) -> Cell {
        Cell {
            p: Point::new(x, y),
            kind,
        }
    }
}
