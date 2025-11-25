use crate::blueprint::Blueprint;
use crate::cell::Cell;
use crate::color::Color;
use crate::material::Material;
use crate::material_definition::MaterialDefinition;
use crate::material_definition::ValueOrReference;
use crate::point::Point;
use crate::point::equilateral_third_point;
use crate::utils::elapsed_secs_f32;
use crate::utils::now;
use crate::wasm_bindgen;
use std::collections::HashMap;
use std::collections::HashSet;

#[wasm_bindgen]
#[derive(Debug, Copy, Clone)]
pub struct WorldConfig {
    pub rdp: f32,
    pub rdv: f32,
    pub zonesize: f32,
    pub gravity: f32,
    pub gravity_2: f32,
    pub crdp: f32,
    pub crdv: f32,
    pub rdv_during_colision: f32,
    pub c2c_gravity: bool,
    pub c2c_colision: bool,
}

struct Stat {
    js: StatJs,
    values: Vec<f32>,
}

#[wasm_bindgen]
#[derive(Debug, Copy, Clone)]
pub struct StatJs {
    pub avg: f32,
    pub p99: f32,
}

impl Stat {
    fn update(&mut self, perf_array_len: usize) {
        while self.values.len() > perf_array_len {
            self.values.remove(0);
        }
        if self.values.is_empty() {
            self.js.avg = 0.0;
            self.js.p99 = 0.0;
        } else {
            let mut sorted: Vec<f32> = self.values.clone();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            let p99_index = ((sorted.len() as f32 - 1.0) * 0.99).floor() as usize;
            self.js.avg = self.values.iter().sum::<f32>() / self.values.len() as f32;
            self.js.p99 = *sorted.get(p99_index).unwrap_or(sorted.last().unwrap());
        }
    }
}

#[derive(Clone)]
pub struct Event {
    func: String,
    cell_id: usize,
    value: f32,
}

#[wasm_bindgen]
pub struct World {
    cells: Vec<Cell>,
    materials: Vec<Material>,
    materials_2: HashMap<String, usize>,
    zones: HashMap<(i32, i32), HashSet<usize>>,
    pairs: HashSet<(usize, usize)>,
    pub c: WorldConfig,
    stats: HashMap<String, Stat>,
    pub perf_array_len: usize,
    tick: usize,
    events: HashMap<usize, Vec<Event>>,
}
impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
impl World {
    pub fn add_event(&mut self, tick: usize, func: String, cell_id: usize, value: f32) {
        let v = self.events.entry(tick).or_default();
        v.push(Event {
            func,
            cell_id,
            value,
        });
    }
    pub fn get_tick(&self) -> usize {
        self.tick
    }
    pub fn new() -> World {
        let mut w = World {
            cells: Vec::new(),
            materials: Vec::new(),
            materials_2: HashMap::new(),
            zones: HashMap::new(),
            pairs: HashSet::new(),
            perf_array_len: 500,
            stats: HashMap::new(),
            tick: 0,
            c: WorldConfig {
                gravity: 0.0,
                crdv: 0.0,
                crdp: 0.0,
                rdv: 0.0,
                rdp: 0.0,
                gravity_2: 0.0,
                rdv_during_colision: 0.0,
                zonesize: 1.0,
                c2c_gravity: false,
                c2c_colision: false,
                // spring: 0.1,
            },
            events: HashMap::new(),
        };
        w.add_stat("logic".to_string());
        w.add_stat("render".to_string());
        w.add_stat("stats".to_string());
        w.add_stat("tick".to_string());
        w.add_stat("tick_01".to_string());
        w.add_stat("tick_02".to_string());
        w.add_stat("tick_03".to_string());
        w.add_stat("tick_04".to_string());
        w.add_stat("tick_05".to_string());
        w.add_stat("tick_06".to_string());
        w.add_stat("tick_handle_events".to_string());
        w
    }
    pub fn set_gravity_2(&mut self, value: f32) {
        self.c.gravity_2 = value;
    }
    pub fn set_gravity(&mut self, value: f32) {
        self.c.gravity = value;
    }
    pub fn set_crdp(&mut self, value: f32) {
        self.c.crdp = value;
    }
    pub fn set_crdv(&mut self, value: f32) {
        self.c.crdv = value;
    }
    pub fn set_c2c_gravity(&mut self, value: bool) {
        self.c.c2c_gravity = value;
    }
    pub fn set_c2c_colision(&mut self, value: bool) {
        self.c.c2c_colision = value;
    }
    // file://./../../../chrono/engine/src/world.rs
    // file://./../../../../../miniciv/src/world.rs
    pub fn tick(&mut self) {
        let n = now();
        self.tick_handle_events();
        self.add_duration("tick_handle_events", elapsed_secs_f32(n));
        let n = now();
        self.tick_01();
        self.add_duration("tick_01", elapsed_secs_f32(n));
        let n = now();
        self.tick_02();
        self.add_duration("tick_02", elapsed_secs_f32(n));
        let n = now();
        self.tick_03();
        self.add_duration("tick_03", elapsed_secs_f32(n));
        let n = now();
        self.tick_04();
        self.add_duration("tick_04", elapsed_secs_f32(n));
        let n = now();
        self.tick_05();
        self.add_duration("tick_05", elapsed_secs_f32(n));
        let n = now();
        self.tick_06();
        self.add_duration("tick_06", elapsed_secs_f32(n));
        self.tick += 1;
    }
    pub fn tick_handle_events(&mut self) {
        let events: Vec<Event> = match self.events.get(&self.tick) {
            Some(events) => events.clone(),
            None => Vec::new(),
        };
        for e in events {
            match e.func.as_str() {
                "set_cell_diameter" => {
                    self.set_cell_diameter(e.cell_id, e.value);
                }
                _ => panic!("invalid event: {}", e.func),
            }
        }
    }
    pub fn add_duration(&mut self, id: &str, value: f32) {
        self.stats.get_mut(id).unwrap().values.push(value);
    }
    pub fn add_stat(&mut self, id: String) {
        self.stats.insert(
            id,
            Stat {
                values: Vec::new(),
                js: StatJs { p99: 0.0, avg: 0.0 },
            },
        );
    }
    pub fn get_stats(&self, id: &str) -> StatJs {
        self.stats[id].js
    }
    pub fn tick_01(&mut self) {
        self.zones.clear();
        let rdv_i = 1.0 - self.c.rdv;
        for cell in &mut self.cells.iter_mut() {
            let dp = cell.p - cell.pp;
            cell.dp = dp * self.c.rdp;
            cell.dv = dp * rdv_i * (1.0 - self.c.rdv_during_colision * cell.collision_count);
            cell.collision_count = 0.0;
        }
        if self.c.c2c_colision {
            for (cell_idx, cell) in &mut self.cells.iter_mut().enumerate() {
                let x1: i32 = ((cell.p.x - cell.diameter * 0.5) / self.c.zonesize).floor() as i32;
                let x2: i32 = ((cell.p.x + cell.diameter * 0.5) / self.c.zonesize).ceil() as i32;
                let y1: i32 = ((cell.p.y - cell.diameter * 0.5) / self.c.zonesize).floor() as i32;
                let y2: i32 = ((cell.p.y + cell.diameter * 0.5) / self.c.zonesize).ceil() as i32;
                for x in x1..=x2 {
                    for y in y1..=y2 {
                        self.zones.entry((x, y)).or_default().insert(cell_idx);
                    }
                }
            }
        }
    }
    pub fn tick_02(&mut self) {
        self.pairs.clear();
        if self.c.c2c_colision {
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
                    ca.dv += n * self.c.crdv * diam_a_ratio;
                    cb.dv -= n * self.c.crdv * diam_b_ratio;
                    ca.dp += n * self.c.crdp * diam_a_ratio;
                    cb.dp -= n * self.c.crdp * diam_b_ratio;
                    ca.collision_count += 1.0;
                    cb.collision_count += 1.0;
                } else {
                    // pass
                }
            }
        }
    }
    pub fn tick_04(&mut self) {
        if self.c.c2c_gravity {
            let cells_ptr = self.cells.as_mut_ptr();
            let cells_len = self.cells.len();
            unsafe {
                let cells_slice_a = std::slice::from_raw_parts_mut(cells_ptr, cells_len);
                let cells_slice_b = std::slice::from_raw_parts_mut(cells_ptr, cells_len);
                for (ia, ca) in cells_slice_a.iter_mut().enumerate().take(cells_len) {
                    for (ib, cb) in cells_slice_b
                        .iter_mut()
                        .enumerate()
                        .take(cells_len)
                        .skip(ia + 1)
                    {
                        assert!(ia < ib);
                        let ca_density = self.materials[ca.material_idx as usize].density;
                        let cb_density = self.materials[cb.material_idx as usize].density;
                        let ma = ca_density * ca.diameter * ca.diameter;
                        let mb = cb_density * cb.diameter * cb.diameter;
                        let dist = ca.p.distance(cb.p);
                        let g = (ma * mb) / (dist * dist);
                        let n = (ca.p - cb.p).normalize();
                        ca.dv -= n * g * self.c.gravity_2;
                        cb.dv += n * g * self.c.gravity_2;
                        // ca.p += Point {
                        //     x: 0.000001,
                        //     y: 0.0,
                        // };
                        // log("test");
                    }
                }
            }
        }
    }
    pub fn tick_05(&mut self) {
        let center = Point { x: 0.0, y: 0.0 };
        for cell in &mut self.cells {
            let gravity = (cell.p - center).normalize() * -self.c.gravity * cell.mass;
            if cell.fixed != 1 {
                cell.p += cell.dp;
            }
            cell.pp = cell.p;
            if cell.fixed != 1 {
                cell.p += gravity + cell.dv;
                // cell.p += Point {
                //     x: 0.00000001,
                //     y: 0.0,
                // }
            }
            cell.ap = (cell.p + cell.pp) * 0.5;
        }
    }
    pub fn tick_06(&mut self) {
        if self.tick.is_multiple_of(20) {
            for stat in self.stats.values_mut() {
                stat.update(self.perf_array_len);
            }
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
    pub fn add_cell_up(&mut self, material_url: &str, idx: usize, diameter: f32) -> usize {
        let c = &self.cells[idx];
        let x = c.p.x;
        let y = c.p.y + c.diameter * 0.5 + diameter * 0.5;
        self.add_cell(material_url, x, y, diameter)
    }
    pub fn add_cell_2(
        &mut self,
        material_url: &str,
        idx_1: usize,
        idx_2: usize,
        diameter: f32,
    ) -> usize {
        let c1 = &self.cells[idx_1];
        let c2 = &self.cells[idx_2];
        let p = equilateral_third_point(c1.p, c2.p);
        self.add_cell(material_url, p.x, p.y, diameter)
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
