use crate::point::Point;
use crate::wasm_bindgen;
#[wasm_bindgen]
#[repr(C)] // https://doc.rust-lang.org/nomicon/other-reprs.html#reprc
pub struct Cell {
    pub p: Point,  // position
    pub pp: Point, // previous position
    pub ap: Point, // average position
    pub dp: Point, // delta position
    pub dv: Point, // delta velocity
    pub material_idx: u32,
    pub mass: f32,
    pub diameter: f32,
    pub fixed: u32,
    pub collision_count: f32,
    pub padding: u32,
}
#[wasm_bindgen]
impl Cell {
    pub fn new(material_idx: usize, x: f32, y: f32, diameter: f32) -> Cell {
        Cell {
            p: Point::new(x, y),
            pp: Point::new(x, y),
            ap: Point::new(x, y),
            material_idx: material_idx as u32,
            diameter,
            dp: Point::new(0.0, 0.0),
            dv: Point::new(0.0, 0.0),
            mass: 1.0,
            fixed: 0,
            collision_count: 0.0,
            padding: 0,
        }
    }
    pub fn size() -> u32 {
        size_of::<Cell>() as u32
    }
}
