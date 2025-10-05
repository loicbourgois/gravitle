use crate::cell::Cell;
use crate::link::Link;
use crate::math::Cri;
use crate::math::collision_response;
use crate::math::delta;
use crate::math::wrap_around;
use crate::point::Point;
use crate::wasm_bindgen;
use serde::Serialize;
use std::collections::HashMap;
const LINK_STRENGH: f32 = 0.2;

#[wasm_bindgen]
#[repr(u8)]
#[derive(PartialEq, Copy, Clone)]
pub enum Kind {
    Armor = 0,
    Booster = 1,
    Core = 2,
    Asteroid = 4,
    Unlighted = 5,
    Lighted = 6,
}

#[derive(Serialize)]
struct ActivationEvent {
    c: u32,
    a: u8,
}

#[wasm_bindgen]
pub struct World {
    cells: Vec<Cell>,
    links: Vec<Link>,
    pub victory: u8,
    pub step: u32,
    pub victory_duration: Option<u32>,
    pub victory_end: Option<u32>,
    pub move_start: Option<u32>,
    activation_events: HashMap<u32, Vec<ActivationEvent>>,
}
impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
impl World {
    pub fn new() -> World {
        World {
            cells: Vec::new(),
            links: Vec::new(),
            step: 0,
            victory: 0,
            victory_duration: None,
            move_start: None,
            victory_end: None,
            activation_events: HashMap::new(),
        }
    }
    pub fn link_exists(&self, aidx: usize, bidx: usize) -> bool {
        for l in &self.links {
            if l.au == aidx && l.bu == bidx {
                return true;
            }
            if l.au == bidx && l.bu == aidx {
                return true;
            }
        }
        false
    }
    pub fn run_step(&mut self) {
        self.update_01();
        self.update_02();
        self.update_03();
        self.update_04();
        self.step += 1;
    }
    pub fn update_01(&mut self) {
        let cells_ptr = self.cells.as_mut_ptr();
        unsafe {
            let cells_slice_a = std::slice::from_raw_parts_mut(cells_ptr, self.cells.len());
            for ca in cells_slice_a {
                ca.direction.reset();
                for l in &self.links {
                    if l.a == ca.idx {
                        let cb = &self.cells[l.bu];
                        let wa = wrap_around(ca.p, cb.p);
                        ca.direction += delta(wa.b, wa.a);
                    }
                    if l.b == ca.idx {
                        let cb = &self.cells[l.au];
                        let wa = wrap_around(ca.p, cb.p);
                        ca.direction += delta(wa.b, wa.a);
                    }
                }
                ca.direction.normalize();
                ca.dp.x = ca.p.x - ca.pp.x;
                ca.dp.y = ca.p.y - ca.pp.y;
                if ca.kind == Kind::Booster && ca.activated == 1 {
                    ca.dp.x -= ca.direction.x * 0.0001;
                    ca.dp.y -= ca.direction.y * 0.0001;
                    if self.move_start.is_none() {
                        self.move_start = Some(self.step);
                    }
                }
                if ca.activated_previous != ca.activated {
                    self.activation_events
                        // we can't adjust the start time back to 0
                        // because we'd be removing tiny fluctuation
                        // and derailling the full course later on
                        // .entry(self.step - self.move_start.unwrap())
                        .entry(self.step)
                        .or_default()
                        .push(ActivationEvent {
                            c: ca.idx,
                            a: ca.activated,
                        });
                }
                ca.activated_previous = ca.activated;
                ca.np.x = ca.p.x + ca.dp.x;
                ca.np.y = ca.p.y + ca.dp.y;
                ca.link_response.reset();
                ca.collision_response.reset();
                ca.collision_response_count = 0;
            }
        }
    }
    pub fn update_02(&mut self) {
        let cells_ptr = self.cells.as_mut_ptr();
        unsafe {
            let cells_slice_a = std::slice::from_raw_parts_mut(cells_ptr, self.cells.len());
            let cells_slice_b = std::slice::from_raw_parts_mut(cells_ptr, self.cells.len());
            for ia in 0..cells_slice_a.len() - 1 {
                for (ib, cb) in cells_slice_b.iter_mut().enumerate().skip(ia + 1) {
                    let ca = &mut cells_slice_a[ia];
                    let wa = wrap_around(ca.np, cb.np);
                    let cria = Cri {
                        np: Point {
                            x: wa.a.x,
                            y: wa.a.y,
                        },
                        dp: ca.dp,
                    };
                    let crib = Cri {
                        np: Point {
                            x: wa.b.x,
                            y: wa.b.y,
                        },
                        dp: cb.dp,
                    };
                    let diams = (ca.diameter + cb.diameter) * 0.5;
                    let colliding = wa.d_sqrd < diams * diams;
                    if colliding
                        && ca.kind != Kind::Unlighted
                        && ca.kind != Kind::Lighted
                        && cb.kind != Kind::Unlighted
                        && cb.kind != Kind::Lighted
                    {
                        let mut cr = collision_response(&cria, &crib);
                        if self.link_exists(ia, ib) {
                            cr.x *= 0.5;
                            cr.y *= 0.5;
                        }
                        ca.collision_response.x -= cr.x;
                        ca.collision_response.y -= cr.y;
                        ca.collision_response_count += 1;
                        cb.collision_response.x += cr.x;
                        cb.collision_response.y += cr.y;
                        cb.collision_response_count += 1;
                    }
                    if colliding
                        && ca.kind == Kind::Unlighted
                        && [Kind::Armor, Kind::Booster, Kind::Core].contains(&cb.kind)
                    {
                        ca.kind = Kind::Lighted;
                    }
                    if colliding
                        && cb.kind == Kind::Unlighted
                        && [Kind::Armor, Kind::Booster, Kind::Core].contains(&ca.kind)
                    {
                        cb.kind = Kind::Lighted;
                    }
                }
            }
        }
    }
    pub fn update_03(&mut self) {
        let cells_ptr = self.cells.as_mut_ptr();
        unsafe {
            let cells_slice_a = std::slice::from_raw_parts_mut(cells_ptr, self.cells.len());
            let cells_slice_b = std::slice::from_raw_parts_mut(cells_ptr, self.cells.len());
            for l in &self.links {
                let ca = &mut cells_slice_a[l.a as usize];
                let cb = &mut cells_slice_b[l.b as usize];
                let wa = wrap_around(ca.np, cb.np);
                let d = (wa.d_sqrd).sqrt();
                let n = delta(wa.a, wa.b).normalize_2(d);
                let ds = (ca.diameter + cb.diameter) * 0.5;
                let factor = (ds - d) * LINK_STRENGH;
                ca.link_response.x -= n.x * factor * 0.5;
                ca.link_response.y -= n.y * factor * 0.5;
                cb.link_response.x += n.x * factor * 0.5;
                cb.link_response.y += n.y * factor * 0.5;
            }
        }
    }
    pub fn update_04(&mut self) {
        let mut c_unlighted = 0;
        for p in &mut self.cells {
            if p.kind == Kind::Unlighted {
                c_unlighted += 1;
            }
            if p.collision_response_count > 0 {
                p.collision_response.x /= p.collision_response_count as f32;
                p.collision_response.y /= p.collision_response_count as f32;
                p.np.x += p.collision_response.x;
                p.np.y += p.collision_response.y;
                p.np.x += p.link_response.x;
                p.np.y += p.link_response.y;
            }
            if p.kind == Kind::Asteroid || p.kind == Kind::Unlighted || p.kind == Kind::Lighted {
                // pass
            } else {
                p.p.x = (p.np.x + 1.0) % 1.0;
                p.p.y = (p.np.y + 1.0) % 1.0;
                p.pp.x = p.p.x - p.dp.x - p.collision_response.x - p.link_response.x;
                p.pp.y = p.p.y - p.dp.y - p.collision_response.y - p.link_response.y;
            }
        }
        if c_unlighted == 0 && self.victory != 1 && self.move_start.is_some() {
            self.victory_end = Some(self.step);
            self.victory_duration = Some(self.step - self.move_start.unwrap());
            self.victory = 1;
        }
    }
    pub fn get_activation_events(&self) -> String {
        serde_json::to_string(&self.activation_events)
            .unwrap_or_else(|_| "Error serializing activation_events to JSON".to_string())
    }
    // Cell
    pub fn add_cell(&mut self, x: f32, y: f32, diameter: f32, kind: Kind) -> u32 {
        let l = self.cells.len();
        let l_u32 = l as u32;
        self.cells.push(Cell::new(l_u32, diameter, kind));
        let cell = &mut self.cells[l];
        cell.set_position(x, y);
        l_u32
    }
    pub fn set_cell_activated(&mut self, idx: u32, activated: u8) {
        self.cells[idx as usize].activated = activated;
    }
    pub fn switch_cell_activated(&mut self, idx: u32) {
        self.cells[idx as usize].activated = (self.cells[idx as usize].activated + 1) % 2;
    }
    pub fn set_cell_kind(&mut self, idx: u32, kind: Kind) {
        self.cells[idx as usize].kind = kind;
    }
    pub fn set_cell_position_x(&mut self, idx: u32, x: f32) {
        self.cells[idx as usize].p.x = x;
    }
    pub fn set_cell_position_y(&mut self, idx: u32, y: f32) {
        self.cells[idx as usize].p.y = y;
    }
    pub fn set_cell_pp_x(&mut self, idx: u32, x: f32) {
        self.cells[idx as usize].pp.x = x;
    }
    pub fn set_cell_pp_y(&mut self, idx: u32, y: f32) {
        self.cells[idx as usize].pp.y = y;
    }
    pub fn set_cell_np_x(&mut self, idx: u32, x: f32) {
        self.cells[idx as usize].np.x = x;
    }
    pub fn set_cell_np_y(&mut self, idx: u32, y: f32) {
        self.cells[idx as usize].np.y = y;
    }
    pub fn set_cell_direction_x(&mut self, idx: u32, x: f32) {
        self.cells[idx as usize].direction.x = x;
    }
    pub fn set_cell_direction_y(&mut self, idx: u32, y: f32) {
        self.cells[idx as usize].direction.y = y;
    }
    pub fn set_cell_dp_x(&mut self, idx: u32, x: f32) {
        self.cells[idx as usize].dp.x = x;
    }
    pub fn set_cell_dp_y(&mut self, idx: u32, y: f32) {
        self.cells[idx as usize].dp.y = y;
    }
    pub fn set_cell_diameter(&mut self, idx: u32, diameter: f32) {
        self.cells[idx as usize].diameter = diameter;
    }
    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }
    pub fn cells_count(&self) -> u32 {
        self.cells.len() as u32
    }
    // Link
    pub fn add_link(&mut self, a: u32, b: u32) -> u32 {
        let l = self.links.len();
        let l_u32 = l as u32;
        self.links.push(Link::new(l_u32, a, b));
        l_u32
    }
    pub fn links_count(&self) -> u32 {
        self.links.len() as u32
    }
    pub fn links(&self) -> *const Link {
        self.links.as_ptr()
    }
}
