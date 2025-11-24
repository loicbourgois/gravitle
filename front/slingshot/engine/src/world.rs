use crate::blueprint::Blueprint;
use crate::cell::Cell;
use crate::color::Color;
use crate::material::Material;
use crate::material_definition::MaterialDefinition;
use crate::material_definition::ValueOrReference;
use crate::point::Point;
use crate::wasm_bindgen;
use std::collections::HashMap;
use std::collections::HashSet;

#[wasm_bindgen]
pub struct World {
    cells: Vec<Cell>,
    materials: Vec<Material>,
    materials_2: HashMap<String, usize>,
    zones: HashMap<(i32, i32), HashSet<usize>>,
    pub rdp: f32,
    pub rdv: f32,
    zonesize: f32,
    pairs: HashSet<(usize, usize)>,
    pub gravity: f32,
    pub gravity_2: f32,
    pub crdp: f32,
    pub crdv: f32,
    pub rdv_during_colision: f32,
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
            materials: Vec::new(),
            materials_2: HashMap::new(),
            zones: HashMap::new(),
            gravity: 0.0,
            crdv: 0.0,
            crdp: 0.0,
            rdv: 0.0,
            rdp: 0.0,
            gravity_2: 0.0,
            rdv_during_colision: 0.0,
            // spring: 0.1,
            zonesize: 1.0,
            pairs: HashSet::new(),
        }
    }
    // file://./../../../chrono/engine/src/world.rs
    // file://./../../../../../miniciv/src/world.rs
    pub fn tick(&mut self) {
        // for cell in &mut self.cells {
        //     cell.p.x += 0.0001;
        // }
        self.tick_01();
        self.tick_02();
        self.tick_03();
        self.tick_04();
        self.tick_05();
    }
    pub fn tick_01(&mut self) {
        self.zones.clear();
        for (cell_idx, cell) in &mut self.cells.iter_mut().enumerate() {
            cell.dp = (cell.p - cell.pp) * self.rdp;
            cell.dv = (cell.p - cell.pp)
                * (1.0 - self.rdv)
                * (1.0 - self.rdv_during_colision * cell.collision_count);
            cell.collision_count = 0.0;
            let x1: i32 = ((cell.p.x - cell.diameter * 0.5) / self.zonesize).floor() as i32;
            let x2: i32 = ((cell.p.x + cell.diameter * 0.5) / self.zonesize).ceil() as i32;
            let y1: i32 = ((cell.p.y - cell.diameter * 0.5) / self.zonesize).floor() as i32;
            let y2: i32 = ((cell.p.y + cell.diameter * 0.5) / self.zonesize).ceil() as i32;
            for x in x1..=x2 {
                for y in y1..=y2 {
                    self.zones.entry((x, y)).or_default().insert(cell_idx);
                }
            }
        }
    }
    pub fn tick_02(&mut self) {
        self.pairs.clear();
        for zone in self.zones.values() {
            let mut ids: Vec<usize> = zone.iter().copied().collect();
            ids.sort_unstable();
            for (idx, ia) in ids.iter().enumerate().take(ids.len() - 1) {
                for ib in ids.iter().take(ids.len()).skip(idx + 1) {
                    self.pairs.insert((*ia, *ib));
                }
            }
        }
    }
    pub fn tick_03(&mut self) {
        let cells_ptr = self.cells.as_mut_ptr();
        let pairs_ = self.pairs.clone();
        unsafe {
            let cells_slice_a = std::slice::from_raw_parts_mut(cells_ptr, self.cells.len());
            let cells_slice_b = std::slice::from_raw_parts_mut(cells_ptr, self.cells.len());
            for pair in &pairs_ {
                let ia = pair.0;
                let ib = pair.1;
                let ca = &mut cells_slice_a[ia];
                let cb = &mut cells_slice_b[ib];
                let dist = ca.p.distance(cb.p);
                let diam_a_ratio = cb.diameter / ca.diameter;
                let diam_b_ratio = ca.diameter / cb.diameter;
                if dist < (ca.diameter + cb.diameter) * 0.5 {
                    let n = (ca.p - cb.p).normalize();
                    ca.dv += n * self.crdv * diam_a_ratio;
                    cb.dv -= n * self.crdv * diam_b_ratio;
                    ca.dp += n * self.crdp * diam_a_ratio;
                    cb.dp -= n * self.crdp * diam_b_ratio;
                    ca.collision_count += 1.0;
                    cb.collision_count += 1.0;
                } else {
                    // pass
                }
            }
        }
    }
    pub fn tick_04(&mut self) {
        let cells_ptr = self.cells.as_mut_ptr();
        unsafe {
            let cells_slice_a = std::slice::from_raw_parts_mut(cells_ptr, self.cells.len());
            let cells_slice_b = std::slice::from_raw_parts_mut(cells_ptr, self.cells.len());
            for (ia, ca) in cells_slice_a.iter_mut().enumerate().take(self.cells.len()) {
                // for ia in 0..self.cells.len() {
                for (ib, cb) in cells_slice_b
                    .iter_mut()
                    .enumerate()
                    .take(self.cells.len())
                    .skip(ia + 1)
                {
                    // for ib in ia + 1..self.cells.len() {
                    assert!(ia < ib);
                    // let ca = &mut cells_slice_a[ia];
                    // let cb = &mut cells_slice_b[ib];
                    let ca_density = self.materials[ca.material_idx as usize].density;
                    let cb_density = self.materials[cb.material_idx as usize].density;
                    let ma = ca_density * ca.diameter * ca.diameter;
                    let mb = cb_density * cb.diameter * cb.diameter;
                    let dist = ca.p.distance(cb.p);
                    let g = (ma * mb) / (dist * dist);
                    let n = (ca.p - cb.p).normalize();
                    ca.dv -= n * g * self.gravity_2;
                    cb.dv += n * g * self.gravity_2;
                }
            }
        }
    }
    pub fn tick_05(&mut self) {
        let center = Point { x: 0.0, y: 0.0 };
        for cell in &mut self.cells {
            let gravity = (cell.p - center).normalize() * -self.gravity * cell.mass;
            if cell.fixed != 1 {
                cell.p += cell.dp;
            }
            cell.pp = cell.p;
            if cell.fixed != 1 {
                cell.p += gravity + cell.dv;
            }
            cell.ap = (cell.p + cell.pp) * 0.5;
        }
    }
    pub fn set_cell_diameter(&mut self, idx: usize, diameter: f32) {
        self.cells[idx].diameter = diameter;
    }
    pub fn set_cell_fixed(&mut self, idx: usize) {
        self.cells[idx].fixed = 1;
    }
    pub fn add_cell(&mut self, material_url: &str, x: f32, y: f32, diameter: f32) -> usize {
        let idx = self.cells.len();
        self.cells
            .push(Cell::new(self.materials_2[material_url], x, y, diameter));
        idx
    }
    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }
    pub fn cells_count(&self) -> u32 {
        self.cells.len() as u32
    }
    pub fn materials(&self) -> *const Material {
        self.materials.as_ptr()
    }
    pub fn materials_count(&self) -> u32 {
        self.materials.len() as u32
    }
    pub fn add_material(&mut self, url: String, definition: &str) {
        let idx = self.materials.len();
        let material_definition: MaterialDefinition = match serde_json::from_str(definition) {
            Ok(m) => m,
            Err(e) => {
                panic!("Failed to parse material JSON from {url}: {e}");
            }
        };
        self.materials.push(self.as_material(material_definition));
        self.materials_2.insert(url, idx);
    }
    pub fn add_from_blueprint(&mut self, blueprint_str: &str, x: f32, y: f32) {
        let blueprint = Blueprint::new(blueprint_str);
        for c in &blueprint.parts {
            self.add_cell(&c.material_url, c.p.x + x, c.p.y + y, c.d);
        }
    }
}

impl World {
    pub fn as_material(&self, md: MaterialDefinition) -> Material {
        Material {
            density: match md.density {
                ValueOrReference::Value(v) => v,
                ValueOrReference::Reference(url) => self.materials[self.materials_2[&url]].density,
            },
            color: Color::from_hex(&md.color).unwrap(),
        }
    }
}
