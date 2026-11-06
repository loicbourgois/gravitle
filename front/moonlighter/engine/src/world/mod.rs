mod debug;
mod electricity;
mod moon;
mod moon_dust;
mod moon_solid;
mod physics;
mod pressure;
mod reset;
mod sensor;
mod signal;

use crate::kind::Kind;
use crate::link::Link;
use crate::particle::Particle;
use crate::sfc32::Sfc32;
use crate::wasm_bindgen;
use std::collections::HashSet;

#[wasm_bindgen]
pub struct World {
    particles: Vec<Particle>,
    links: Vec<Link>,
    pub completed_steps: u32,
    rng: Sfc32,
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
            particles: Vec::new(),
            links: Vec::new(),
            completed_steps: 0,
            rng: crate::sfc32::Sfc32::new(&[19, 4768, 9087, 35]),
        }
    }

    pub fn tick(&mut self) {
        let pairs = reset::reset_pairs(self);
        reset::reset_particles(self);
        electricity::generate(self);
        pressure::equalize(self, &pairs);
        moon_dust::equalize(self, &pairs);
        electricity::equalize(self, &pairs);
        pressure::generate(self);
        pressure::apply(self);
        moon_dust::apply(self);
        electricity::apply(self);
        pressure::block(self, &pairs);
        pressure::leak(self);
        sensor::activation(self);
        signal::tansmit(self);
        signal::apply(self);
        moon::dust_to_solid(self);
        moon_solid::launch(self);
        moon_solid::move_(self, &pairs);
        physics::update(self);
        debug::assert(self);
        self.completed_steps += 1;
    }

    // Returns true when the two particles' circles overlap.
    // Uses squared distances to avoid a sqrt per pair.
    fn colision(a: &Particle, b: &Particle) -> bool {
        let dx = a.p.x - b.p.x;
        let dy = a.p.y - b.p.y;
        let radius_sum = a.diameter * 0.5 + b.diameter * 0.5;
        dx * dx + dy * dy <= radius_sum * radius_sum
    }

    // Get particle
    fn p(&self, i: usize) -> &Particle {
        &self.particles[i]
    }

    // Get mutable particle
    fn pm(&mut self, i: usize) -> &mut Particle {
        &mut self.particles[i]
    }

    // Get neighbours
    fn ns(&self, idx: usize) -> HashSet<usize> {
        self.links
            .iter()
            .filter_map(|link| {
                if link.a as usize == idx {
                    Some(link.b as usize)
                } else if link.b as usize == idx {
                    Some(link.a as usize)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn add_particle(&mut self, x: f32, y: f32, k: Kind) -> u32 {
        let index = self.particles.len() as u32;
        self.particles.push(Particle::new(index, k, x, y));
        index
    }

    pub fn particles(&self) -> *const Particle {
        self.particles.as_ptr()
    }
    pub fn particles_count(&self) -> u32 {
        self.particles.len() as u32
    }

    pub fn set_activation(&mut self, pidx: usize, v: i32) {
        self.particles[pidx].activation = v;
    }

    pub fn add_link(&mut self, a: u32, b: u32) {
        let index = self.links.len() as u32;
        self.links.push(Link::new(index, a, b));
    }

    pub fn links(&self) -> *const Link {
        self.links.as_ptr()
    }
    pub fn links_count(&self) -> u32 {
        self.links.len() as u32
    }
}
