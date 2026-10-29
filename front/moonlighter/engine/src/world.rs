use crate::link::Link;
use crate::particle::Particle;
use crate::kind::Kind;
use crate::wasm_bindgen;
use crate::sfc32::Sfc32;
use std::collections::HashSet;

const FILTER_PASS_PROBABILITY: f32 = 0.01;

#[wasm_bindgen]
pub struct World {
    particles: Vec<Particle>,
    links: Vec<Link>,
    pub step: u32,
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
            step: 0,
            rng: crate::sfc32::Sfc32::new(&[0, 0, 0, 0]),
        }
    }


    pub fn tick(&mut self) {
        // Electricity
        // for solar panel, electricity_qd is always 1
        // electricty should then flow equally to everything
        // if 2 neighbours ar at (2,5), next step they should be at (3,4)
        // we only distributed by increment of 1
        // (10,1) -> (9,2)
        // we must first equalize the network, then push the overflow to the biggest
        // (10/100,2/2,0/2) -> (10/100,2/2,1/2) -> (10/100,2/2,2/2) -> (11/100,2/2,2/2)

        // Pressure
        // The pressure comparison permits transfer only from the lower-
        // pressure endpoint to the higher-pressure endpoint, so the reverse
        // visit cannot transfer in the opposite direction during this step.


        self.step = self.step.wrapping_add(1);
        
        
        let mut pairs: HashSet<(usize, usize)> = HashSet::new();
        for link in &self.links {
            let a = link.a as usize;
            let b = link.b as usize;
            pairs.insert((a, b));
            pairs.insert((b, a));
        }

        
        for particle in &mut self.particles {
            particle.moon_qd = 0;
            particle.pressure_qn = 0;
            particle.electricity_qd = 0;
        }


        for particle in &mut self.particles {
            if particle.kind == Kind::Pump {
                particle.pressure_qn = (particle.pressure_q - 1).max(-6);
            }
            if particle.kind == Kind::SolarPanel && particle.electricity_q < particle.electricity_c {
                particle.electricity_qd += 1;
            }
        }


        for &(a, b) in &pairs {
            match self.particles[a].kind {
                Kind::Pipe | Kind::Moon | Kind::Tank | Kind::Filter => {
                    let neighboor_pressure = self.particles[b].pressure_q;
                    self.particles[a].pressure_qn = self.particles[a].pressure_qn.min(neighboor_pressure.saturating_add(1));
                }
                _ => {
                    // pass
                }
            }
        }


        for (a, b) in &pairs {
            let source = &self.particles[*a];
            let destination = &self.particles[*b];
            if source.kind == Kind::Filter
                || destination.pressure_q >= source.pressure_q
                || source.moon_q <= 0
                || source.moon_q + source.moon_qd <= 0
                || destination.moon_q >= destination.moon_c
                || destination.moon_q + destination.moon_qd >= destination.moon_c
            {
                continue;
            }
            if destination.kind == Kind::Filter
                && self.rng.next_f32() >= FILTER_PASS_PROBABILITY
            {
                continue;
            }
            self.particles[*a].moon_qd -= 1;
            self.particles[*b].moon_qd += 1;
        }

        
        


        // for (a, b) in &pairs {
        //     let source = &self.particles[*a];
        //     let destination = &self.particles[*b];
        //     let source_q = source.electricity_q + source.electricity_qd;
        //     let dest_q = destination.electricity_q + destination.electricity_qd;
        //     let dest_free_capacity = destination.electricity_c - dest_q;
        //     let source_c = source.electricity_c;
        //     let transfer = ((source_q - dest_q + 1) / 2).max(1).min(dest_free_capacity);
        //     if (
        //         source_q > dest_q 
        //         && source_q > 0 
        //         && dest_free_capacity > 0
        //     ) 
        //     || (
        //         source_q == source_c
        //         && source_q > 0
        //         && dest_free_capacity > 0 
        //     ) 
        //     {
        //         self.particles[*a].electricity_qd -= transfer;
        //         self.particles[*b].electricity_qd += transfer;
        //     }
        // }


        for particle in &mut self.particles {
            particle.pressure_q = particle.pressure_qn;
            particle.moon_q += particle.moon_qd;
            particle.electricity_q += particle.electricity_qd;
            assert!(particle.electricity_q <= particle.electricity_c);
            assert!(particle.electricity_q >= 0);
        }
    }


    pub fn add_particle(&mut self, x: f32, y: f32, k: Kind) -> u32 {
        let index = self.particles.len() as u32;
        self.particles.push(Particle::new(
            index,
            1.0,
            k,
            x, y
        ));
        index
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
