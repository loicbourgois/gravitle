use crate::link::Link;
use crate::particle::Particle;
use crate::kind::Kind;
use crate::wasm_bindgen;

#[wasm_bindgen]
pub struct World {
    particles: Vec<Particle>,
    links: Vec<Link>,
    pub step: u32,
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
            step: 0,
        }
    }

    pub fn add_particle(&mut self, x: f32, y: f32, k: Kind) {
        let index = self.particles.len() as u32;
        self.particles.push(Particle::new(
            index,
            1.0,
            k,
            x, y
        ));
    }

    pub fn particles(&self) -> *const Particle {
        self.particles.as_ptr()
    }
    pub fn particles_count(&self) -> u32 {
        self.particles.len() as u32
    }

    pub fn add_link(&mut self, a: u32, b: u32) {
        let index = self.links.len() as u32;
        self.links.push(Link::new(
            index,
            a,
            b,
        ));
    }

    pub fn links(&self) -> *const Link {
        self.links.as_ptr()
    }
    pub fn links_count(&self) -> u32 {
        self.links.len() as u32
    }
}
