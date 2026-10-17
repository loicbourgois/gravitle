// TODO: optimize
use crate::particle::Particle;
use crate::link::Link;
use crate::math::Cri;
use crate::math::collision_response;
use crate::math::delta;
use crate::math::wrap_around_v2;
use crate::point::Point;
use crate::sfc32::Sfc32;
use crate::wasm_bindgen;
use serde::Serialize;
use std::collections::HashMap;
use std::collections::HashSet;


const LINK_STRENGH: f32 = 0.2;

#[wasm_bindgen]
#[repr(u32)]
#[derive(PartialEq, Copy, Clone)]
pub enum Kind {
    Armor = 0,
    Booster = 1,
    Core = 2,
    Asteroid = 4,
    Unlighted = 5,
    Lighted = 6,
}

#[wasm_bindgen]
#[repr(u32)]
#[derive(PartialEq, Copy, Clone)]
pub enum UserKind {
    User = 1,
    Ghost = 2,
    Other = 3,
}

#[derive(Serialize)]
struct ActivationEvent {
    c: u32,
    a: u32,
}

#[wasm_bindgen]
pub struct World {
    particles: Vec<Particle>,
    links: Vec<Link>,
    pub victory: u8,
    pub step: u32,
    pub victory_duration: Option<u32>,
    pub victory_fuel_used: Option<u32>,
    fuel_used: u32,
    pub victory_end: Option<u32>,
    pub move_start: Option<u32>,
    activation_events: HashMap<u32, Vec<ActivationEvent>>,
    // How many cell per side we have
    // resolution = 10 
    // -> grid of 10x10 
    // -> each grid cell is 0.1 x 0.1 in size
    grid_resolution: usize,
    // Stores particles indexes
    // Used for colisions detection
    // The goal is to reduce the number of checks needed.
    grid: Vec<HashSet<usize>>,
    // Potential collision pairs
    // Used during collision detection
    // Built from the grid
    // Left idx must always be less than right idx: we don't want
    // to double process a pair. 
    pairs: HashSet<(usize, usize)>,
}
impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}


#[wasm_bindgen]
impl World {
    
    
    pub fn new() -> World {
        let grid_resolution = 10;
        World {
            particles: Vec::new(),
            links: Vec::new(),
            step: 0,
            victory: 0,
            fuel_used: 0,
            victory_duration: None,
            move_start: None,
            victory_end: None,
            victory_fuel_used: None,
            activation_events: HashMap::new(),
            grid_resolution ,
            grid: (0..grid_resolution*grid_resolution).map(|_| HashSet::new()).collect(),
            pairs: HashSet::new(),
        }
    }


    pub fn setup(
        &mut self,
        seed: &[u32],
        asteroid_count: u32,
        star_count: u32,
        user_kind: UserKind,
    ) {
        let mut sfc32 = Sfc32::new(seed);
        for _ in 0..asteroid_count {
            let diameter = 0.025;
            let mut iterations = 0;
            loop {
                iterations += 1;
                assert!(iterations < 100, "too many iterations");
                let x = sfc32.next_f32();
                let y = sfc32.next_f32();
                let mut ok = true;
                for particle in &self.particles {
                    let wa = wrap_around_v2(particle.p, Point { x, y });
                    let diams = particle.diameter + diameter * 1.5;
                    let colliding = wa.d_sqrd < diams * diams;
                    if colliding {
                        ok = false;
                        break;
                    }
                }
                if ok {
                    for _ in 0..16 {
                        let x2 = x + (sfc32.next_f32()-0.5) * diameter * 1.0;
                        let y2 = y + (sfc32.next_f32()-0.5) * diameter * 1.0;
                        self.add_particle(x2, y2, diameter, Kind::Asteroid, user_kind);
                    }
                    break;
                }
            }
        }

        for _ in 0..star_count {
            let diameter = 0.015;
            let mut iterations = 0;
            loop {
                iterations += 1;
                assert!(iterations < 200, "too many iterations");
                let x = sfc32.next_f32();
                let y = sfc32.next_f32();
                let mut ok = true;
                for particle in &self.particles {
                    let wa = wrap_around_v2(particle.p, Point { x, y });
                    let diams = particle.diameter * 0.5 + diameter * 5.0;
                    let colliding = wa.d_sqrd < diams * diams;
                    if colliding {
                        ok = false;
                        break;
                    }
                }
                if ok {
                    self.add_particle(x, y, diameter, Kind::Unlighted, user_kind);
                    break;
                }
            }
        }
    }
    pub fn link_exists(&self, aidx: usize, bidx: usize) -> bool {
        for l in &self.links {
            if l.au == aidx && l.bu == bidx {
                return true;
            }
            if l.au == bidx && l.bu == aidx {
                return true;
            }
        }
        false
    }


    pub fn run_step(&mut self) {
        self.update_01();
        self.update_grid();
        self.update_pairs();
        self.update_02();
        self.update_03();
        self.update_04();
        self.step += 1;
    }


    fn update_grid(&mut self) {
        for cell in &mut self.grid {
            cell.clear();
        }
        let resolution = self.grid_resolution as isize;
        let cell_size = 1.0 / self.grid_resolution as f32;
        for (index, particle) in self.particles.iter().enumerate() {
            let radius = particle.diameter * 0.5;
            let x_min = ((particle.np.x - radius) / cell_size).floor() as isize;
            let x_max = ((particle.np.x + radius) / cell_size).floor() as isize;
            let y_min = ((particle.np.y - radius) / cell_size).floor() as isize;
            let y_max = ((particle.np.y + radius) / cell_size).floor() as isize;
            for y in y_min..=y_max {
                for x in x_min..=x_max {
                    let cell_x = x.rem_euclid(resolution) as usize;
                    let cell_y = y.rem_euclid(resolution) as usize;
                    self.grid[cell_y * self.grid_resolution + cell_x].insert(index);
                }
            }
        }
    }


    fn update_pairs(&mut self) {
        self.pairs.clear();
        for cell in &self.grid {
            let mut indices: Vec<_> = cell.iter().copied().collect();
            indices.sort_unstable();
            for (position, &a) in indices.iter().enumerate() {
                for &b in &indices[position + 1..] {
                    let pa = &self.particles[a];
                    let pb = &self.particles[b];
                    match (pa.kind, pb.kind) {
                        (Kind::Asteroid, Kind::Asteroid)
                        | (Kind::Unlighted, Kind::Unlighted)
                        | (Kind::Lighted, Kind::Lighted)
                        | (Kind::Asteroid, Kind::Unlighted)
                        | (Kind::Unlighted, Kind::Asteroid)
                        | (Kind::Asteroid, Kind::Lighted)
                        | (Kind::Lighted, Kind::Asteroid) 
                        => {}
                        _ => {
                            self.pairs.insert((a, b));
                        }
                    }
                }
            }
        }
    }
    

    /// Calculate each particle's link-based direction and displacement, apply active
    /// booster thrust, record activation changes, and reset response accumulators.
    pub fn update_01(&mut self) {
        let particles_ptr = self.particles.as_mut_ptr();
        // We must use unsafe for conccurent access to `particles`.
        unsafe {
            let particles_slice_a = std::slice::from_raw_parts_mut(particles_ptr, self.particles.len());
            for ca in particles_slice_a {
                ca.direction.reset();
                for l in &self.links {
                    if l.a == ca.idx {
                        let cb = &self.particles[l.bu];
                        let wa = wrap_around_v2(ca.p, cb.p);
                        ca.direction += delta(wa.b, wa.a);
                    }
                    if l.b == ca.idx {
                        let cb = &self.particles[l.au];
                        let wa = wrap_around_v2(ca.p, cb.p);
                        ca.direction += delta(wa.b, wa.a);
                    }
                }
                ca.direction.normalize();
                ca.dp.x = ca.p.x - ca.pp.x;
                ca.dp.y = ca.p.y - ca.pp.y;
                if ca.kind == Kind::Booster && ca.activated == 1 {
                    ca.dp.x -= ca.direction.x * 0.0001;
                    ca.dp.y -= ca.direction.y * 0.0001;
                    self.fuel_used += 1;
                    if self.move_start.is_none() {
                        self.move_start = Some(self.step);
                    }
                }
                if ca.activated_previous != ca.activated {
                    self.activation_events
                        // we can't adjust the start time back to 0
                        // because we'd be removing tiny fluctuation
                        // and derailling the full course later on
                        // .entry(self.step - self.move_start.unwrap())
                        .entry(self.step)
                        .or_default()
                        .push(ActivationEvent {
                            c: ca.idx,
                            a: ca.activated,
                        });
                }
                ca.activated_previous = ca.activated;
                ca.np.x = ca.p.x + ca.dp.x;
                ca.np.y = ca.p.y + ca.dp.y;
                ca.link_response.reset();
                ca.collision_response.reset();
                ca.collision_response_count = 0;
            }
        }
    }


    /// Check predicted particle positions for collisions, accumulate collision
    /// responses, and light stars touched by ship particles.
    pub fn update_02(&mut self) {
        let particles_ptr = self.particles.as_mut_ptr();
        unsafe {
            let particles_slice_a = std::slice::from_raw_parts_mut(particles_ptr, self.particles.len());
            let particles_slice_b = std::slice::from_raw_parts_mut(particles_ptr, self.particles.len());
            for &(ia, ib) in &self.pairs {
                let ca = &mut particles_slice_a[ia];
                let cb = &mut particles_slice_b[ib];
                let wa = wrap_around_v2(ca.np, cb.np);
                let cria = Cri {
                    np: Point {
                        x: wa.a.x,
                        y: wa.a.y,
                    },
                    dp: ca.dp,
                };
                let crib = Cri {
                    np: Point {
                        x: wa.b.x,
                        y: wa.b.y,
                    },
                    dp: cb.dp,
                };
                let diams = (ca.diameter + cb.diameter) * 0.5;
                let colliding = wa.d_sqrd < diams * diams;
                if colliding
                    && ca.kind != Kind::Unlighted
                    && ca.kind != Kind::Lighted
                    && cb.kind != Kind::Unlighted
                    && cb.kind != Kind::Lighted
                {
                    let mut cr = collision_response(&cria, &crib);
                    if self.link_exists(ia, ib) {
                        cr.x *= 0.5;
                        cr.y *= 0.5;
                    }
                    ca.collision_response.x -= cr.x;
                    ca.collision_response.y -= cr.y;
                    ca.collision_response_count += 1;
                    cb.collision_response.x += cr.x;
                    cb.collision_response.y += cr.y;
                    cb.collision_response_count += 1;
                }
                if colliding
                    && ca.kind == Kind::Unlighted
                    && [Kind::Armor, Kind::Booster, Kind::Core].contains(&cb.kind)
                {
                    ca.kind = Kind::Lighted;
                }
                if colliding
                    && cb.kind == Kind::Unlighted
                    && [Kind::Armor, Kind::Booster, Kind::Core].contains(&ca.kind)
                {
                    cb.kind = Kind::Lighted;
                }
            }
        }
    }


    /// Accumulate position corrections that keep linked particles together.
    pub fn update_03(&mut self) {
        let particles_ptr = self.particles.as_mut_ptr();
        unsafe {
            let particles_slice_a = std::slice::from_raw_parts_mut(particles_ptr, self.particles.len());
            let particles_slice_b = std::slice::from_raw_parts_mut(particles_ptr, self.particles.len());
            for l in &self.links {
                let ca = &mut particles_slice_a[l.a as usize];
                let cb = &mut particles_slice_b[l.b as usize];
                let wa = wrap_around_v2(ca.np, cb.np);
                let d = (wa.d_sqrd).sqrt();
                let n = delta(wa.a, wa.b).normalize_2(d);
                let ds = (ca.diameter + cb.diameter) * 0.5;
                let factor = (ds - d) * LINK_STRENGH;
                ca.link_response.x -= n.x * factor * 0.5;
                ca.link_response.y -= n.y * factor * 0.5;
                cb.link_response.x += n.x * factor * 0.5;
                cb.link_response.y += n.y * factor * 0.5;
            }
        }
    }


    /// Apply accumulated responses, advance movable particles, and record victory
    /// when all stars are lighted after movement has begun.
    pub fn update_04(&mut self) {
        let mut c_unlighted = 0;
        for p in &mut self.particles {
            if p.kind == Kind::Unlighted {
                c_unlighted += 1;
            }
            if p.collision_response_count > 0 {
                p.collision_response.x /= p.collision_response_count as f32;
                p.collision_response.y /= p.collision_response_count as f32;
                p.np.x += p.collision_response.x;
                p.np.y += p.collision_response.y;
                p.np.x += p.link_response.x;
                p.np.y += p.link_response.y;
            }
            if p.kind == Kind::Asteroid || p.kind == Kind::Unlighted || p.kind == Kind::Lighted {
                // pass
            } else {
                p.p.x = (p.np.x + 1.0) % 1.0;
                p.p.y = (p.np.y + 1.0) % 1.0;
                p.pp.x = p.p.x - p.dp.x - p.collision_response.x - p.link_response.x;
                p.pp.y = p.p.y - p.dp.y - p.collision_response.y - p.link_response.y;
            }
        }
        if c_unlighted == 0 && self.victory != 1 && self.move_start.is_some() {
            self.victory_end = Some(self.step);
            self.victory_duration = Some(self.step - self.move_start.unwrap());
            self.victory_fuel_used = Some(self.fuel_used);
            self.victory = 1;
        }
    }


    pub fn get_activation_events(&self) -> String {
        serde_json::to_string(&self.activation_events)
            .unwrap_or_else(|_| "Error serializing activation_events to JSON".to_string())
    }


    // Particles management
    pub fn add_particle(
        &mut self,
        x: f32,
        y: f32,
        diameter: f32,
        kind: Kind,
        user_kind: UserKind,
    ) -> u32 {
        let l = self.particles.len();
        let l_u32 = l as u32;
        self.particles.push(Particle::new(l_u32, diameter, kind, user_kind));
        let particle = &mut self.particles[l];
        particle.set_position(x, y);
        l_u32
    }
    pub fn set_particle_activated(&mut self, idx: u32, activated: u32) {
        self.particles[idx as usize].activated = activated;
    }
    pub fn switch_particle_activated(&mut self, idx: u32) {
        self.particles[idx as usize].activated = (self.particles[idx as usize].activated + 1) % 2;
    }
    pub fn set_particle_kind(&mut self, idx: u32, kind: Kind) {
        self.particles[idx as usize].kind = kind;
    }
    pub fn set_particle_position_x(&mut self, idx: u32, x: f32) {
        self.particles[idx as usize].p.x = x;
    }
    pub fn set_particle_position_y(&mut self, idx: u32, y: f32) {
        self.particles[idx as usize].p.y = y;
    }
    pub fn set_particle_pp_x(&mut self, idx: u32, x: f32) {
        self.particles[idx as usize].pp.x = x;
    }
    pub fn set_particle_pp_y(&mut self, idx: u32, y: f32) {
        self.particles[idx as usize].pp.y = y;
    }
    pub fn set_particle_np_x(&mut self, idx: u32, x: f32) {
        self.particles[idx as usize].np.x = x;
    }
    pub fn set_particle_np_y(&mut self, idx: u32, y: f32) {
        self.particles[idx as usize].np.y = y;
    }
    pub fn set_particle_direction_x(&mut self, idx: u32, x: f32) {
        self.particles[idx as usize].direction.x = x;
    }
    pub fn set_particle_direction_y(&mut self, idx: u32, y: f32) {
        self.particles[idx as usize].direction.y = y;
    }
    pub fn set_particle_dp_x(&mut self, idx: u32, x: f32) {
        self.particles[idx as usize].dp.x = x;
    }
    pub fn set_particle_dp_y(&mut self, idx: u32, y: f32) {
        self.particles[idx as usize].dp.y = y;
    }
    pub fn set_particle_diameter(&mut self, idx: u32, diameter: f32) {
        self.particles[idx as usize].diameter = diameter;
    }
    pub fn particles(&self) -> *const Particle {
        self.particles.as_ptr()
    }
    pub fn particles_count(&self) -> u32 {
        self.particles.len() as u32
    }


    // Links management
    pub fn add_link(&mut self, a: u32, b: u32) -> u32 {
        let l = self.links.len();
        let l_u32 = l as u32;
        self.links.push(Link::new(l_u32, a, b));
        l_u32
    }
    pub fn links_count(&self) -> u32 {
        self.links.len() as u32
    }
    pub fn links(&self) -> *const Link {
        self.links.as_ptr()
    }


    // Pairs
    pub fn pairs_count(&self) -> u32 {
        self.pairs.len() as u32
    }
}
