use crate::cell::Cell;
use crate::material::Material;
use crate::wasm_bindgen;
// use serde_json;
use std::collections::HashMap;

#[wasm_bindgen]
pub struct World {
    cells: Vec<Cell>,
    materials: Vec<Material>,
    materials_2: HashMap<String, usize>,
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
        }
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
    pub fn add_material(&mut self, url: String, definition: &str) {
        let idx = self.materials.len();
        let material: Material = match serde_json::from_str(definition) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("Failed to parse material JSON from {url}: {e}");
                return;
            }
        };
        self.materials.push(material);
        self.materials_2.insert(url, idx);
    }
}
