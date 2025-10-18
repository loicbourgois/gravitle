use crate::point::Point;
use crate::wasm_bindgen;
use crate::world::Kind;
use crate::world::UserKind;
#[wasm_bindgen]
#[repr(C)]
pub struct Cell {
    pub idx: u32,
    pub diameter: f32,
    // position
    pub p: Point,
    // previous position
    pub pp: Point,
    // new position
    pub np: Point,
    // delta position
    pub dp: Point,
    // direction in which the cell points
    // away from neighbours (tbc)
    pub direction: Point,
    // delta velocity
    pub dv: Point,
    pub link_response: Point,
    pub collision_response: Point,
    pub collision_response_count: u32,
    pub activated: u32,
    pub activated_previous: u32,
    pub kind: Kind,
    pub user_kind: UserKind,
    pub padding: u32,
}
#[wasm_bindgen]
impl Cell {
    pub fn new(idx: u32, diameter: f32, kind: Kind, user_kind: UserKind) -> Cell {
        Cell {
            idx,
            diameter,
            p: Point::new(0.0, 0.0),
            np: Point::new(0.0, 0.0),
            pp: Point::new(0.0, 0.0),
            dp: Point::new(0.0, 0.0),
            dv: Point::new(0.0, 0.0),
            direction: Point::new(0.0, 0.0),
            activated: 0,
            activated_previous: 0,
            kind,
            user_kind,
            collision_response: Point::new(0.0, 0.0),
            collision_response_count: 0,
            link_response: Point::new(0.0, 0.0),
            padding: 0,
        }
    }
    pub fn size() -> u32 {
        size_of::<Cell>() as u32
    }
    pub fn set_position(&mut self, x: f32, y: f32) {
        self.p.x = x;
        self.p.y = y;
        self.pp.x = x;
        self.pp.y = y;
        self.np.x = x;
        self.np.y = y;
    }
}
