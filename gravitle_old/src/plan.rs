use crate::{Float, Kind};
use wasm_bindgen::prelude::wasm_bindgen;
#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub struct PartPlan {
    pub a: usize,
    pub b: usize,
    pub k: Kind,
}
#[wasm_bindgen]
pub struct Plan {
    pub k1: Kind,
    pub f: Float,
    pub k2: Kind,
    part_plans: Vec<PartPlan>,
}
#[wasm_bindgen]
impl Plan {
    pub fn new(k1: Kind, f: Float, k2: Kind) -> Plan {
        return Plan {
            k1: k1,
            f: f,
            k2: k2,
            part_plans: Vec::new(),
        };
    }
    pub fn add(&mut self, a: usize, b: usize, k: Kind) {
        self.part_plans.push(PartPlan { a: a, b: b, k: k });
    }
}
impl Plan {
    pub fn part_plans_(&self) -> &Vec<PartPlan> {
        &self.part_plans
    }
    // pub fn part_count(&self) -> usize {
    //     self.part_plans.len()
    // }
}

// #[wasm_bindgen]
// impl PartPlan {
//     pub fn new(a: u32,
//     b: u32,
//     k: Kind,) -> PartPlan {
//         return ;
//     }
// }
