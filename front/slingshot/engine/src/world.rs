use crate::cell::Cell;
use crate::wasm_bindgen;

#[wasm_bindgen]
pub struct World {
    cells: Vec<Cell>,
}
impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}
#[wasm_bindgen]
impl World {
    pub fn new() -> World {
        World { cells: Vec::new() }
    }
    pub fn add_cell(&mut self, x: f32, y: f32) -> usize {
        let idx = self.cells.len();
        self.cells.push(Cell::new(x, y));
        idx
    }
    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }
    pub fn cells_count(&self) -> u32 {
        self.cells.len() as u32
    }
}
