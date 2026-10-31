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

    // c:  capacity
    // q:  quantity
    // qd: quantity delta
    // qn: quantity new

    pub pressure_c: i32,
    pub pressure_q: i32,
    pub pressure_qn: i32,

    pub moon_c: i32,
    pub moon_q: i32,
    pub moon_qd: i32,

    pub electricity_c: i32,
    pub electricity_q: i32,
    pub electricity_qd: i32,

    // For wasm
    pub padding: u32,
}

#[wasm_bindgen]
impl Particle {
    pub fn new(idx: u32, kind: Kind, x: f32, y: f32) -> Particle {
        let mut p = Particle {
            idx,
            diameter: match kind {
                Kind::Wire => 0.25,
                _ => 1.0,
            },
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
            pressure_c: match kind {
                Kind::Pump => -6,
                _ => 0,
            },
            pressure_q: 0,
            pressure_qn: 0,
            moon_c: match kind {
                Kind::Moon => 10000,
                Kind::Tank => 100,
                Kind::Pipe | Kind::Filter => 10,
                _ => 0,
            },
            moon_q: match kind {
                Kind::Moon => 10000,
                _ => 0,
            },
            moon_qd: 0,
            electricity_c: match kind {
                Kind::Battery => 10,
                Kind::SolarPanel => 5,
                Kind::Pump => 4,
                Kind::Wire => 4,
                _ => 0,
            },
            electricity_q: 0,
            electricity_qd: 0,
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
