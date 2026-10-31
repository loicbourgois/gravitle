use crate::link::Link;
use crate::particle::Particle;
use crate::kind::Kind;
use crate::wasm_bindgen;
use crate::sfc32::Sfc32;
use std::collections::BTreeSet;


// The probability for dust to get into the filter
// and start clogging it.
const FILTER_PASS_PROBABILITY: f32 = 0.1;


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
            rng: crate::sfc32::Sfc32::new(&[19, 4768, 9087, 35]),
        }
    }


    pub fn tick(&mut self) {
        // Electricity
        //
        // Solar Panel
        // A panal generates 1 electricity per tick
        // 
        // We only distribute by increment of 1 by step
        // We want the electricity to flow gradually. 
        // No exception.
        // 
        // Electricty should flow equally to everything
        // (10,1) -> (9,2)
        // we must first equalize the network, then push the overflow to the biggest
        // (10/100,2/2,0/2) -> (10/100,2/2,1/2) -> (10/100,2/2,2/2) -> (11/100,2/2,2/2)


        self.step = self.step.wrapping_add(1);
        
        
        let mut pairs: BTreeSet<(usize, usize)> = BTreeSet::new();
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
            if particle.kind == Kind::SolarPanel {
                particle.electricity_qd += 1;
            }
        }


        // `Kind::Moon` must remain a fixed high-pressure source (0)
        // Otherwise, a moon connected to a low-pressure pipe 
        // would adopt that lower pressure, destroying the gradient 
        // that drives dust toward the pumps.
        // for &(a, b) in &pairs {
        //     let ak = self.particles[a].kind;
        //     let bk = self.particles[b].kind;
        //     match ak {
        //         Kind::Pipe | Kind::Tank | Kind::Filter => {
        //             let neighboor_pressure = self.particles[b].pressure_q;
        //             let qn_up = self.particles[a].pressure_qn.min(neighboor_pressure + 1);
        //             let qn_down = self.particles[a].pressure_qn.min(neighboor_pressure - 1);
        //             self.particles[a].pressure_qn = ;
        //         }
        //         Kind::Valve 
        //             if self.particles[a].activation > 0 => {
        //                 let neighboor_pressure = self.particles[b].pressure_q;
        //                 self.particles[a].pressure_qn = self.particles[a].pressure_qn.min(neighboor_pressure.saturating_add(1));
        //         }
        //         _ => {
        //             // pass
        //         }
        //     }
        // }

        
        for aidx in 0..self.particles.len() {
            let ak = self.particles[aidx].kind;
            let aa = self.particles[aidx].activation;
            match ak {
                Kind::Pipe | Kind::Tank | Kind::Filter => {
                    // pass
                }
                Kind::Valve if aa > 0 => {
                    // pass
                }
                _ => {
                    continue
                }
            }
            let mut total: f32 = 0.0;
            let mut count: f32 = 0.0;
            for &(a, b) in &pairs {
                if a == aidx {
                    if self.particles[b].kind == Kind::Valve && self.particles[b].activation == 0 {

                    } else {
                        total += self.particles[b].pressure_q as f32;
                        count += 1.0;
                    }
                }
            }
            if count > 0.0 {
                let average = total / count;
                self.particles[aidx].pressure_qn = if average > 0.0 {
                    average.ceil() as i32 
                } else {
                    average.floor() as i32 
                };
                // self.particles[aidx].pressure_qn = average.round() as i32 ;
            }
        } 


        for (a, b) in &pairs {
            let source = &self.particles[*a];
            let destination = &self.particles[*b];
            if destination.pressure_q >= source.pressure_q
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


        for (a, b) in &pairs {
            let source = &self.particles[*a];
            let destination = &self.particles[*b];
            let dest_c = destination.electricity_c;
            let source_c = source.electricity_c;
            // we don't want to consider particles 
            // if they can't hold electricity
            if source_c <= 0 || dest_c <= 0 {
                continue;
            }
            // if the swicth if turned off, it can not receive electricity
            if destination.kind == Kind::Switch && destination.activation == 0 {
                continue;
            }
            let mut source_q = source.electricity_q + source.electricity_qd;
            source_q = source_q.min(source.electricity_q);
            let dest_q = destination.electricity_q + destination.electricity_qd;
            let dest_free_capacity = destination.electricity_c - dest_q;
            let source_ratio = source_q as f32 / source_c as f32;
            let dest_ratio = dest_q as f32 / dest_c as f32;
            let source_overcapacity = source.electricity_q + source.electricity_qd - source.electricity_c;
            if (
                source_ratio > dest_ratio 
                && source_q > 0 
                && dest_free_capacity > 0
            ) || (
                source_overcapacity > 0
                && dest_free_capacity > 0
            )
            {
                self.particles[*a].electricity_qd -= 1;
                self.particles[*b].electricity_qd += 1;
            }
        }
        for particle in &mut self.particles {
            let eq = particle.electricity_q + particle.electricity_qd;
            let a = particle.activation;
            if particle.kind == Kind::Pump && eq > 0 {
                particle.electricity_qd -= 1;
                particle.pressure_qn = particle.pressure_q - 1;
                particle.pressure_qn = particle.pressure_qn.max(particle.pressure_c);
            }
            if particle.kind == Kind::Pump && eq <= 0 {
                particle.pressure_qn = particle.pressure_q + 1;
                particle.pressure_qn = particle.pressure_qn.min(0);
            }
            if particle.kind == Kind::Blower && eq > 0 {
                particle.electricity_qd -= 1;
                particle.pressure_qn = particle.pressure_q + 1;
                particle.pressure_qn = particle.pressure_qn.min(particle.pressure_c);
            }
            else if particle.kind == Kind::Blower {
                particle.pressure_qn = particle.pressure_q - 1;
                particle.pressure_qn = particle.pressure_qn.max(0);
            }
        }
        for particle in &mut self.particles {
            particle.pressure_q = particle.pressure_qn;
            particle.moon_q += particle.moon_qd;
            particle.electricity_q += particle.electricity_qd;
        }
        for particle in &mut self.particles {
            if particle.kind == Kind::SolarPanel {
                particle.electricity_q = particle.electricity_q.min(particle.electricity_c);
            }
        }
        for particle in &mut self.particles {
            assert!(particle.electricity_q <= particle.electricity_c);
            assert!(particle.electricity_q >= 0);
        }
    }


    pub fn add_particle(&mut self, x: f32, y: f32, k: Kind) -> u32 {
        let index = self.particles.len() as u32;
        self.particles.push(Particle::new(
            index,
            k,
            x, 
            y,
        ));
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
