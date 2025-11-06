use crate::blueprint::Blueprint;
use crate::cell::Cell;
use crate::color::Color;
use crate::material::Material;
use crate::material_definition::MaterialDefinition;
use crate::material_definition::ValueOrReference;
use crate::wasm_bindgen;
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
    pub fn tick(&mut self) {
        TODO
        for cell in &mut self.cells {
            cell.p.x += 0.0001;
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
