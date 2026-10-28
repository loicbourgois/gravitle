use crate::kind::Kind;
use crate::point::Point;
use crate::wasm_bindgen;

#[wasm_bindgen]
#[repr(C)]
pub struct Particle {
    pub idx: u32,

    pub diameter: f32,

    // position
    pub p: Point,

    // previous position
    pub pp: Point,

    // new position
    pub np: Point,

    // Delta Position
    pub dp: Point,

    // direction in which the particle points
    // away from neighbours
    pub direction: Point,

    // delta velocity
    // Not used currently, but kept for potential future usage.
    // We might add logic or graphics on top of it.
    pub dv: Point,

    pub link_response: Point,
    pub collision_response: Point,
    pub collision_response_count: u32,
    pub kind: Kind,

    // For wasm
    pub padding: u32,
}

#[wasm_bindgen]
impl Particle {
    pub fn new(idx: u32, diameter: f32, kind: Kind, x: f32, y: f32) -> Particle {
        let mut p = Particle {
            idx,
            diameter,
            p: Point::new(0.0, 0.0),
            np: Point::new(0.0, 0.0),
            pp: Point::new(0.0, 0.0),
            dp: Point::new(0.0, 0.0),
            dv: Point::new(0.0, 0.0),
            direction: Point::new(0.0, 0.0),
            kind,
            collision_response: Point::new(0.0, 0.0),
            collision_response_count: 0,
            link_response: Point::new(0.0, 0.0),
            padding: 0,
        };
        p.set_position(x, y);
        p
    }
    pub fn size() -> u32 {
        size_of::<Particle>() as u32
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
