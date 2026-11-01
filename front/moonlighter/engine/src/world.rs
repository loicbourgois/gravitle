use crate::kind::Kind;
use crate::link::Link;
use crate::log;
use crate::particle::Particle;
use crate::sfc32::Sfc32;
use crate::wasm_bindgen;
use std::collections::{BTreeSet, HashSet};

// The probability for dust to get into the filter
// and start clogging it.
const FILTER_PASS_PROBABILITY: f32 = 0.10;

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

        // Pressure management
        for aidx in 0..self.particles.len() {
            let ak = self.particles[aidx].kind;
            let aa = self.particles[aidx].activation;
            match ak {
                Kind::Pipe | Kind::Tank | Kind::Filter | Kind::Press => {
                    // pass
                }
                Kind::Valve if aa > 0 => {
                    // pass
                }
                _ => continue,
            }
            let mut total: f32 = 0.0;
            let mut count: f32 = 0.0;
            for &(a, b) in &pairs {
                if a == aidx {
                    if self.particles[b].kind == Kind::Valve && self.particles[b].activation == 0 {
                    } else if self.particles[b].kind == Kind::Valve
                        || self.particles[b].kind == Kind::Tank
                        || self.particles[b].kind == Kind::Pipe
                        || self.particles[b].kind == Kind::Pump
                        || self.particles[b].kind == Kind::Blower
                        || self.particles[b].kind == Kind::Filter
                        || self.particles[b].kind == Kind::Press
                    {
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
            if destination.kind == Kind::Filter && self.rng.next_f32() >= FILTER_PASS_PROBABILITY {
                continue;
            }
            if destination.kind == Kind::Filter && source.kind == Kind::Filter {
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
            let source_overcapacity =
                source.electricity_q + source.electricity_qd - source.electricity_c;
            if (source_ratio > dest_ratio && source_q > 0 && dest_free_capacity > 0)
                || (source_overcapacity > 0 && dest_free_capacity > 0)
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
            } else if particle.kind == Kind::Blower {
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

        // If filter is full, blocks the pressure
        for aidx in 0..self.particles.len() {
            if self.particles[aidx].kind != Kind::Filter
                || self.particles[aidx].moon_q < self.particles[aidx].moon_c
            {
                continue;
            }
            // filter out neighbour if it's a closed valve
            if let Some(pressure) = pairs
                .iter()
                .filter(|&&(a, b)| {
                    a == aidx
                        && !(self.particles[b].kind == Kind::Valve
                            && self.particles[b].activation == 0)
                })
                .map(|&(_, b)| self.particles[b].pressure_q)
                .max()
            {
                self.particles[aidx].pressure_q = pressure;
            }
        }

        // Leaking
        // Helps with above filter blocking
        for particle in &mut self.particles {
            match particle.kind {
                Kind::Pump | Kind::Blower => {}
                _ => {
                    particle.pressure_q -= particle.pressure_q.signum();
                }
            }
        }

        for i1 in 0..self.particles.len() {
            match self.p(i1).kind {
                Kind::SensorMoonFull => {
                    self.pm(i1).signal_q = 0;
                    for i2 in self.ns(i1) {
                        let p2 = self.p(i2);
                        if p2.moon_c > 0 && p2.moon_c == p2.moon_q {
                            self.pm(i1).signal_q = 1;
                        }
                    }
                    self.pm(i1).signal_qn = self.pm(i1).signal_q;
                }
                Kind::SensorMoonEmpty => {
                    self.pm(i1).signal_q = 0;
                    for i2 in self.ns(i1) {
                        let p2 = self.p(i2);
                        if p2.moon_c > 0 && p2.moon_q == 0 {
                            self.pm(i1).signal_q = -1;
                        }
                    }
                    self.pm(i1).signal_qn = self.pm(i1).signal_q;
                }
                _ => {}
            }
        }

        for i1 in 0..self.particles.len() {
            match self.p(i1).kind {
                Kind::SignalWire => {
                    let mut total: f32 = 0.0;
                    let mut count: f32 = 0.0;
                    for i2 in self.ns(i1) {
                        let p2 = self.p(i2);
                        match p2.kind {
                            Kind::SignalWire | Kind::SignalInput => {
                                total += p2.signal_q as f32;
                                count += 1.0;
                            }
                            _ => {}
                        }
                    }
                    if count > 0.0 {
                        let average = total / count;
                        self.pm(i1).signal_qn = if average > 0.0 {
                            average.ceil() as i32
                        } else {
                            average.floor() as i32
                        };
                    }
                }
                Kind::SignalInput => {
                    let mut total: f32 = 0.0;
                    let mut count: f32 = 0.0;
                    for i2 in self.ns(i1) {
                        let p2 = self.p(i2);
                        match p2.kind {
                            Kind::SensorMoonFull => {
                                total += p2.signal_q as f32;
                                count += 1.0;
                            }
                            _ => {}
                        }
                    }
                    if count > 0.0 {
                        let average = total / count;
                        self.pm(i1).signal_qn = if average > 0.0 {
                            average.ceil() as i32
                        } else {
                            average.floor() as i32
                        };
                    }
                }
                Kind::Trit => {
                    let mut total: i32 = 0;
                    for i2 in self.ns(i1) {
                        let p2 = self.p(i2);
                        match p2.kind {
                            Kind::SensorMoonFull | Kind::SensorMoonEmpty => {
                                total += p2.signal_q;
                            }
                            _ => {}
                        }
                    }
                    self.pm(i1).signal_qn = match total.signum() {
                        1 => 1,
                        -1 => -1,
                        _ => self.p(i1).signal_q,
                    };
                }
                Kind::SignalInverter => {
                    let mut total: i32 = 0;
                    for i2 in self.ns(i1) {
                        let p2 = self.p(i2);
                        match p2.kind {
                            Kind::Trit => {
                                total += p2.signal_q;
                            }
                            _ => {}
                        }
                    }
                    self.pm(i1).signal_qn = match total.signum() {
                        1 => -1,
                        -1 => 1,
                        _ => self.p(i1).signal_q,
                    };
                }
                Kind::Valve | Kind::Switch => {
                    let mut total: i32 = 0;
                    for i2 in self.ns(i1) {
                        let p2 = self.p(i2);
                        match p2.kind {
                            Kind::SignalInverter | Kind::Trit => {
                                total += p2.signal_q;
                            }
                            _ => {}
                        }
                    }
                    self.pm(i1).signal_qn = match total.signum() {
                        1 => 1,
                        -1 => -1,
                        _ => self.p(i1).signal_q,
                    };
                }
                _ => {}
            }
        }
        for particle in &mut self.particles {
            particle.signal_q = particle.signal_qn;
        }
        for particle in &mut self.particles {
            match particle.kind {
                Kind::Valve | Kind::Switch => {
                    if particle.signal_q == 1 {
                        particle.activation = 1;
                    }
                    if particle.signal_q == -1 {
                        particle.activation = 0;
                    }
                }
                _ => {}
            }
        }

        // Dust -> Solid
        for i1 in 0..self.particles.len() {
            let p1 = self.pm(i1);
            match p1.kind {
                Kind::Press => {
                    if p1.moon_q == p1.moon_c && p1.solidmoon_q == 0 {
                        p1.moon_q = 0;
                        p1.solidmoon_q = 1;
                    }
                }
                _ => {}
            }
        }

        // Launch
        let mut particles_to_add = Vec::new();
        for i1 in 0..self.particles.len() {
            match self.p(i1).kind {
                Kind::Launcher => {
                    if self.p(i1).solidmoon_q == 1 {
                        self.pm(i1).solidmoon_q = 0;
                        particles_to_add.push((self.p(i1).p.x, self.p(i1).p.y, Kind::MoonBall));
                    }
                }
                _ => {}
            }
        }
        for p in particles_to_add {
            self.add_particle(p.0, p.1, p.2);
        }

        // SolidMoon movement
        // Movement happens in two steps:
        // 1. Calculate deltas (solidmoon_qd) for all particles
        // 2. Apply the deltas to the actual quantities (solidmoon_q)
        //
        // Valid transitions:
        // - Press -> Conveyor
        // - Conveyor -> Launcher
        // Movement only happens if:
        // - Source has solidmoon (source_q > 0)
        // - Destination has capacity (dest_q < dest_c)
        for p in &mut self.particles {
            p.solidmoon_qd = 0;
        }
        for (a, b) in &pairs {
            let source_kind = self.particles[*a].kind;
            let dest_kind = self.particles[*b].kind;
            let mut source_q = self.particles[*a].solidmoon_q + self.particles[*a].solidmoon_qd;
            source_q = source_q.min(self.particles[*a].solidmoon_q);
            let dest_q = self.particles[*b].solidmoon_q + self.particles[*b].solidmoon_qd;
            let dest_c = self.particles[*b].solidmoon_c;
            let valid = matches!(
                (source_kind, dest_kind),
                (Kind::Press, Kind::Conveyor) | (Kind::Conveyor, Kind::Launcher)
            );
            if !valid || source_q <= 0 || dest_q >= dest_c {
                continue;
            }
            self.particles[*a].solidmoon_qd -= 1;
            self.particles[*b].solidmoon_qd += 1;
        }
        for p in &mut self.particles {
            p.solidmoon_q += p.solidmoon_qd;
        }

        // Movement
        for p in &mut self.particles {
            p.pp.x = p.p.x;
            p.pp.y = p.p.y;
        }
        for p in &mut self.particles {
            match p.kind {
                Kind::MoonBall => {
                    p.p.y += 0.02;
                    p.p.x -= 0.02;
                }
                _ => {}
            }
        }

        // Collision
        let mut crs = Vec::new();
        for p1 in &self.particles {
            for p2 in &self.particles {
                if p1.kind == Kind::MoonBall && p2.kind == Kind::Earth && Self::colision(p1, p2) {
                    crs.push((p1.idx as usize, p2.idx as usize));
                }
            }
        }
        for cr in crs {
            self.particles[cr.0].kind = Kind::Dead;
            self.particles[cr.1].solidmoon_q += 1;
        }

        // Debug
        for particle in &mut self.particles {
            assert!(particle.moon_q <= particle.moon_c);
            assert!(particle.electricity_q <= particle.electricity_c);
            assert!(particle.electricity_q >= 0);
        }

        // Done
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
