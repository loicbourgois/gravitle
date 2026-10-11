use crate::wasm_bindgen;

pub struct Stat {
    pub js: StatJs,
    pub values: Vec<f32>,
}

#[wasm_bindgen]
#[derive(Debug, Copy, Clone)]
pub struct StatJs {
    pub avg: f32,
    pub p99: f32,
}

impl Stat {
    pub fn update(&mut self, perf_array_len: usize) {
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
