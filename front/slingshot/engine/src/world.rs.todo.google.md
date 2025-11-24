To address the `TODO` in `add_cell_2`, we need to implement the geometric calculation for finding a circle tangent to two other circles.

Here's the plan:
1.  **Correct the function signature**: The return type should be `usize` (the index of the newly added cell), not `{`.
2.  **Correct parameter usage**: `idx` in the original `TODO` should be `idx_1`.
3.  **Geometric Calculation**:
    *   Let the two existing cells be `c1` and `c2`, with centers `p1`, `p2` and radii `r1`, `r2`.
    *   Let the new cell be `c3`, with center `p3` and radius `r3`.
    *   For `c3` to be tangent to `c1`, the distance `p3` to `p1` must be `r1 + r3`.
    *   For `c3` to be tangent to `c2`, the distance `p3` to `p2` must be `r2 + r3`.
    *   This reduces to finding the intersection points of two "helper" circles:
        *   Helper Circle 1: Center `p1`, Radius `R_A = r1 + r3`.
        *   Helper Circle 2: Center `p2`, Radius `R_B = r2 + r3`.
    *   We'll use the standard formula for circle-circle intersection to find the coordinates `(x, y)` for `p3`.
4.  **Edge Cases**:
    *   **Existing cells at the same location (`p1 == p2`)**: This is a degenerate case. If their radii are also the same (`r1 == r2`), there are infinite solutions (any point at `r1 + r3` distance from `p1`). We'll pick a canonical one (e.g., directly "above" `p1`). If radii differ, it's impossible.
    *   **No real intersection points**: If the helper circles are too far apart or one is completely contained within the other without touching, there are no solutions. We'll `panic!` in such cases, as this indicates an impossible geometric configuration for the given inputs.
    *   **Floating point precision**: Handle potential small negative values when calculating square roots near tangent points by clamping to zero.
5.  **Choose one solution**: The intersection of two circles can yield zero, one (tangent), or two points. If two points are found, we'll pick one (e.g., the one with the positive perpendicular offset relative to the line connecting `p1` and `p2`).

```rust
use crate::blueprint::Blueprint;
use crate::cell::Cell;
use crate::color::Color;
use crate::material::Material;
use crate::material_definition::MaterialDefinition;
use crate::material_definition::ValueOrReference;
use crate::point::Point;
use crate::utils::elapsed_secs_f32;
use crate::utils::now;
use crate::wasm_bindgen;
use std::collections::HashMap;
use std::collections::HashSet;

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
    stats: HashMap<String, Stat>,
    pub perf_array_len: usize,
}
impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
impl World {
    pub fn new() -> World {
        let mut w = World {
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
            perf_array_len: 100,
            stats: HashMap::new(),
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
        w
    }
    // file://./../../../chrono/engine/src/world.rs
    // file://./../../../../../miniciv/src/world.rs
    pub fn tick(&mut self) {
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
    pub fn tick_06(&mut self) {
        for stat in self.stats.values_mut() {
            stat.update(self.perf_array_len);
        }
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
                for (ib, cb) in cells_slice_b
                    .iter_mut()
                    .enumerate()
                    .take(self.cells.len())
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
    pub fn add_cell_up(&mut self, material_url: &str, idx: usize, diameter: f32) -> usize {
        let c = &self.cells[idx];
        let x = c.p.x;
        let y = c.p.y + c.diameter * 0.5 + diameter * 0.5;
        self.add_cell(material_url, x, y, diameter)
    }
    /// Adds a new cell tangent to two existing cells.
    ///
    /// This function calculates the position (x, y) for a new cell (c3) such that it is
    /// just touching two existing cells (c1 and c2). It solves a geometric problem
    /// involving the intersection of two circles, where the centers of the circles are
    /// the centers of c1 and c2, and their radii are (r1 + r3) and (r2 + r3) respectively
    /// (r1, r2, r3 are the radii of c1, c2, and the new cell).
    ///
    /// If there are two possible tangent points, it picks one (the one corresponding to
    /// the 'positive' perpendicular offset from the line connecting c1 and c2).
    ///
    /// # Panics
    /// - If the two existing cells (c1 and c2) are at the same location but have
    ///   different diameters, making a tangent placement impossible.
    /// - If the geometry of the three circles makes it impossible to find a real point
    ///   where the new cell can be tangent to both existing cells (e.g., the existing
    ///   cells are too far apart, or one is completely contained within the required
    ///   tangency radius of the other), considering floating point precision.
    pub fn add_cell_2(
        &mut self, material_url: &str, idx_1: usize, idx_2: usize, diameter: f32
    ) -> usize {
        let c1 = &self.cells[idx_1];
        let c2 = &self.cells[idx_2];

        let p1 = c1.p;
        let p2 = c2.p;
        let r1 = c1.diameter * 0.5;
        let r2 = c2.diameter * 0.5;
        let r3 = diameter * 0.5;

        let dist_p1_p2 = p1.distance(p2);

        // Radii of the two helper circles whose intersection points are the centers of the new cell
        let r_helper_1 = r1 + r3;
        let r_helper_2 = r2 + r3;

        // Handle degenerate case: p1 and p2 are the same point
        if dist_p1_p2 < f32::EPSILON { // Use epsilon for float comparison
            if (r1 - r2).abs() < f32::EPSILON {
                // If r1 == r2, there are infinite solutions forming a circle around p1/p2.
                // We choose one point, directly 'above' p1.
                return self.add_cell(material_url, p1.x, p1.y + r_helper_1, diameter);
            } else {
                panic!("Cannot place new cell tangent to two cells at the same location with different radii.");
            }
        }

        // Calculate x_prime, the distance along the line p1-p2 from p1 to the intersection chord.
        // This is derived from the standard formula for circle-circle intersection.
        let x_prime = (dist_p1_p2 * dist_p1_p2 + r_helper_1 * r_helper_1 - r_helper_2 * r_helper_2) / (2.0 * dist_p1_p2);

        // Calculate y_prime_squared, the square of the perpendicular distance from the line p1-p2 to the intersection points.
        let y_prime_squared = r_helper_1 * r_helper_1 - x_prime * x_prime;

        // Check for impossible geometry (no real solutions)
        // y_prime_squared might be slightly negative due to floating-point inaccuracies
        // when circles are nearly tangent; we allow small negative values and clamp to 0.
        if y_prime_squared < -f32::EPSILON * r_helper_1 * r_helper_1 {
            panic!("Cannot place new cell tangent to existing cells: geometric impossibility (helper circles do not intersect).");
        }
        let y_prime = y_prime_squared.max(0.0).sqrt(); // Clamp to 0 before sqrt

        // Vector from p1 to p2
        let p1_to_p2 = p2 - p1;
        let p1_to_p2_normalized = p1_to_p2.normalize();

        // Point on the line p1-p2 at distance x_prime from p1
        let point_on_line = p1 + p1_to_p2_normalized * x_prime;

        // Perpendicular vector to p1_to_p2_normalized (rotated 90 degrees)
        let p1_to_p2_perp = Point { x: -p1_to_p2_normalized.y, y: p1_to_p2_normalized.x };

        // Choose one of the two possible intersection points (e.g., the one with positive y_prime component relative to the p1-p2 vector)
        let p3 = point_on_line + p1_to_p2_perp * y_prime;

        self.add_cell(material_url, p3.x, p3.y, diameter)
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

```