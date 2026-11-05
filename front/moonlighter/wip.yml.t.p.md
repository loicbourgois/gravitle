# General Context

You are improving the codebase.
Your role is to address any TODO left in the code.
Only write down the actual changes, with some context around.
Do not format as a git diff.
Answer as a markdown document:


## Output
````
# Reasoning
<thorough reasoning>


# Updates


## <path #1>


### <change #1>
```<language>
<code>
```


### <change #2>
```<language>
<code>
```

````


# Specific Rules
- keep code tidy
- if replacing code, give context into what to remove
- TODO= draw each link useing the wgsl shader . for now, only draw a simple pink debug line
- todo: 
  Uncaptured WebGPU error: Validation Error

  Caused by:
    In Device::create_bind_group
      Number of bindings in bind group descriptor (4) does not match the number of bindings defined in the bind group layout (3)


# Files
## file://./../../../gravitle/front/moonlighter/engine/src/world.rs

```rust
use crate::kind::Kind;
use crate::link::Link;
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
```

## file://./../../../gravitle/front/moonlighter/engine/src/particle.rs

```rust
use crate::kind::Kind;
use crate::point::Point;
use crate::wasm_bindgen;

#[wasm_bindgen]
#[repr(C)]
pub struct Particle {
    pub idx: u32,

    pub diameter: f32,

    // position
    pub p: Point,

    // previous position
    pub pp: Point,

    // new position
    pub np: Point,

    // Delta Position
    pub dp: Point,

    // direction in which the particle points
    // away from neighbours
    pub direction: Point,

    // delta velocity
    // Not used currently, but kept for potential future usage.
    // We might add logic or graphics on top of it.
    pub dv: Point,

    pub link_response: Point,
    pub collision_response: Point,
    pub collision_response_count: u32,
    pub kind: Kind,

    // c:  capacity
    // q:  quantity
    // qd: quantity delta
    // qn: quantity new
    pub pressure_c: i32,
    pub pressure_q: i32,
    pub pressure_qn: i32,

    pub moon_c: i32,
    pub moon_q: i32,
    pub moon_qd: i32,

    pub electricity_c: i32,
    pub electricity_q: i32,
    pub electricity_qd: i32,

    pub activation: i32,

    pub signal_q: i32,
    pub signal_qn: i32,

    pub solidmoon_c: i32,
    pub solidmoon_q: i32,
    pub solidmoon_qd: i32,

    // For wasm
    pub padding: u32,
}

#[wasm_bindgen]
impl Particle {
    pub fn new(idx: u32, kind: Kind, x: f32, y: f32) -> Particle {
        let mut p = Particle {
            idx,
            diameter: match kind {
                Kind::Wire
                | Kind::Led
                | Kind::Switch
                | Kind::SensorMoonFull
                | Kind::SensorMoonEmpty
                | Kind::SignalInput
                | Kind::SignalOutput
                | Kind::MoonBall
                | Kind::Trit
                | Kind::SignalInverter
                | Kind::SignalWire => 0.25,
                Kind::Pipe => 0.75,
                _ => 1.0,
            },
            p: Point::new(0.0, 0.0),
            np: Point::new(0.0, 0.0),
            pp: Point::new(0.0, 0.0),
            dp: Point::new(0.0, 0.0),
            dv: Point::new(0.0, 0.0),
            direction: Point::new(0.0, 0.0),
            kind,
            collision_response: Point::new(0.0, 0.0),
            collision_response_count: 0,
            link_response: Point::new(0.0, 0.0),
            pressure_c: match kind {
                Kind::Pump => -50,
                Kind::Blower => 50,
                _ => 0,
            },
            pressure_q: 0,
            pressure_qn: 0,
            moon_c: match kind {
                Kind::Moon => 1_000_000,
                Kind::Tank => 1000,
                Kind::Press => 40,
                Kind::Pipe | Kind::Filter => 10,
                Kind::Valve | Kind::Pump | Kind::Blower => 1,
                _ => 0,
            },
            moon_q: match kind {
                Kind::Moon => 1_000_000,
                _ => 0,
            },
            moon_qd: 0,
            electricity_c: match kind {
                Kind::Battery => 100,
                Kind::SolarPanel => 10,
                Kind::Pump => 4,
                Kind::Wire | Kind::Led | Kind::Switch | Kind::Blower => 10,
                _ => 0,
            },
            electricity_q: 0,
            electricity_qd: 0,
            activation: 0,
            signal_q: 0,
            signal_qn: 0,
            solidmoon_c: match kind {
                Kind::Press | Kind::Launcher | Kind::Conveyor => 1,
                _ => 0,
            },
            solidmoon_q: 0,
            solidmoon_qd: 0,
            padding: 0,
        };
        p.set_position(x, y);
        p
    }
    pub fn size() -> u32 {
        size_of::<Particle>() as u32
    }
    pub fn set_position(&mut self, x: f32, y: f32) {
        self.p.x = x;
        self.p.y = y;
        self.pp.x = x;
        self.pp.y = y;
        self.np.x = x;
        self.np.y = y;
    }
}
```

## file://./../../../gravitle/front/moonlighter/engine/src/link.rs

```rust
use crate::wasm_bindgen;
#[wasm_bindgen]
#[repr(C)]
pub struct Link {
    pub idx: u32,
    pub a: u32,
    pub b: u32,
}
#[wasm_bindgen]
impl Link {
    pub fn new(idx: u32, a: u32, b: u32) -> Link {
        Link { idx, a, b }
    }
    pub fn size() -> u32 {
        size_of::<Link>() as u32
    }
}
```

## file://./../../../gravitle/front/moonlighter/engine/src/kind.rs

```rust
use crate::wasm_bindgen;

#[wasm_bindgen]
#[repr(u32)]
#[derive(PartialEq, Copy, Clone)]
pub enum Kind {
    Moon = 0,
    Pipe = 1,
    Tank = 2,
    Filter = 3,
    Pump = 4,
    SolarPanel = 5,
    Battery = 6,
    Wire = 7,
    Switch = 8,
    Led = 9,
    Blower = 10,
    Valve = 11,
    SensorMoonFull = 12,
    SensorMoonEmpty = 13,
    SignalInput = 14,
    SignalOutput = 15,
    SignalWire = 16,
    Trit = 17,
    SignalInverter = 18,
    Press = 19,
    Storage = 20,
    Conveyor = 21,
    Launcher = 22,
    MoonBall = 23,
    Earth = 24,
    Dead = 25,
}

#[wasm_bindgen]
pub fn base_color(k: Kind) -> String {
    match k {
        Kind::Moon => "#eee".to_string(),
        Kind::Pipe => "#b88".to_string(),
        Kind::Tank => "#866".to_string(),
        Kind::Filter => "rgb(136, 164, 255)".to_string(),
        Kind::Pump => "#d88".to_string(),
        Kind::SolarPanel => "rgb(227, 219, 183)".to_string(),
        Kind::Battery => "rgb(204, 194, 147)".to_string(),
        Kind::Wire => "rgb(209, 117, 71)".to_string(),
        Kind::Switch => "rgb(118, 118, 117)".to_string(),
        Kind::Led => "rgb(51, 113, 57)".to_string(),
        Kind::Blower => "rgb(191, 124, 200)".to_string(),
        Kind::Valve => "rgb(124, 149, 200)".to_string(),

        Kind::SensorMoonFull => "rgb(231, 232, 127)".to_string(),
        Kind::SensorMoonEmpty => "rgb(124, 149, 200)".to_string(),
        Kind::SignalInput => "rgb(124, 149, 200)".to_string(),
        Kind::SignalOutput => "rgb(124, 149, 200)".to_string(),
        Kind::SignalWire => "rgb(168, 172, 118)".to_string(),
        Kind::Trit => "rgb(182, 182, 181)".to_string(),
        Kind::SignalInverter => "rgb(182, 182, 181)".to_string(),

        Kind::Press => "rgb(103, 169, 208)".to_string(),
        Kind::Storage => "rgb(126, 190, 157)".to_string(),
        Kind::Conveyor => "rgb(136, 126, 190)".to_string(),
        Kind::Launcher => "rgb(190, 188, 126)".to_string(),
        Kind::MoonBall => "#eee".to_string(),
        Kind::Earth => "#48f".to_string(),
        Kind::Dead => "#1111".to_string(),
        // _ => "#f0f".to_string(),
    }
}

#[wasm_bindgen]
pub fn draw_link_as_pipe(ak: Kind, bk: Kind) -> bool {
    draw_link_as_pipe_inner(ak, bk) || draw_link_as_pipe_inner(bk, ak)
}

fn draw_link_as_pipe_inner(ak: Kind, bk: Kind) -> bool {
    matches!(
        (ak, bk),
        (Kind::Pipe, Kind::Pipe)
            | (Kind::Pipe, Kind::Valve)
            | (Kind::Pipe, Kind::Moon)
            | (Kind::Pipe, Kind::Tank)
            | (Kind::Pipe, Kind::Filter)
            | (Kind::Filter, Kind::Filter)
            | (Kind::Filter, Kind::Pump)
            | (Kind::Filter, Kind::Press)
            | (Kind::Pipe, Kind::Press)
            | (Kind::Filter, Kind::Blower)
            | (Kind::Pipe, Kind::Blower)
            | (Kind::Valve, Kind::Tank)
            | (Kind::Valve, Kind::Pump)
            | (Kind::Valve, Kind::Filter)
            | (Kind::Moon, Kind::Press)
    )
}

#[wasm_bindgen]
pub fn draw_link_as_electricty(ak: Kind, bk: Kind) -> bool {
    draw_link_as_electricty_inner(ak, bk) || draw_link_as_electricty_inner(bk, ak)
}

fn draw_link_as_electricty_inner(ak: Kind, bk: Kind) -> bool {
    matches!(
        (ak, bk),
        (Kind::SolarPanel, Kind::Pump)
            | (Kind::SolarPanel, Kind::Switch)
            | (Kind::Pump, Kind::Switch)
            | (Kind::Blower, Kind::Switch)
            | (Kind::Wire, Kind::Switch)
            | (Kind::Wire, Kind::Wire)
            | (Kind::Switch, Kind::Led)
            | (Kind::Blower, Kind::Led)
            | (Kind::SolarPanel, Kind::Wire)
    )
}

#[wasm_bindgen]
pub fn draw_signal(ak: Kind, bk: Kind) -> bool {
    draw_signal_inner(ak, bk) || draw_signal_inner(bk, ak)
}

fn draw_signal_inner(ak: Kind, bk: Kind) -> bool {
    matches!(
        (ak, bk),
        (Kind::Switch, Kind::Trit)
            | (Kind::Switch, Kind::SignalInverter)
            | (Kind::SensorMoonEmpty, Kind::Trit)
            | (Kind::SignalInverter, Kind::Trit)
            | (Kind::SensorMoonFull, Kind::Trit)
            | (Kind::SensorMoonEmpty, Kind::Filter)
            | (Kind::SensorMoonFull, Kind::Filter)
            | (Kind::Valve, Kind::Trit)
            | (Kind::Valve, Kind::SignalInverter)
    )
}

#[wasm_bindgen]
pub fn draw_solid_moon(ak: Kind, bk: Kind) -> bool {
    draw_solid_moon_inner(ak, bk) || draw_solid_moon_inner(bk, ak)
}

fn draw_solid_moon_inner(ak: Kind, bk: Kind) -> bool {
    matches!(
        (ak, bk),
        (Kind::Press, Kind::Conveyor) | (Kind::Conveyor, Kind::Launcher)
    )
}
```

## file://./../../../gravitle/front/moonlighter/draw.js

```javascript
import { ZOOM } from "./consts.js";
import { get_particle } from "./particle.js";
import { get_link } from "./link.js";


const history = {
    esum: [],
    emax: [],
    psum: [],
    pmax: [],
}


const draw_diam = (canvas) => {
    return Math.min(canvas.height, canvas.width) * ZOOM;
}


const draw = (ctx, center, world, engine, memory) => {
    ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height);

    // for elec and pressure, draw {n}max as well
    // we should always see {n}max squares
    // use the full color for esum, then the empty color for the rest
    // const ec_empty = "#4408"
    // const ec_full = "#8808"
    // const pc_empty = "#244"
    // const pc_full = "#484"

    // // Elect draw
    // // Keep the first sample at the bottom-left; later samples extend right.
    // const scrollHistory = true
    // const squareSize = 6;
    // const squareStep = squareSize + 1;
    // const visibleColumns = Math.floor(
    //     (ctx.canvas.width - squareSize) / squareStep,
    // ) + 1;
    // const startColumn = scrollHistory
    //     ? Math.max(0, history.esum.length - Math.max(0, visibleColumns))
    //     : 0;
    // ctx.fillStyle = ec_full;
    // for (
    //     let column = 0;
    //     column < visibleColumns &&
    //     startColumn + column < history.esum.length;
    //     column++
    // ) {
    //     const index = startColumn + column;
    //     const esum = history.esum[index];
    //     const emax = history.emax[index];
    //     const x = column * squareStep;
    //     for (
    //         let row = 0;
    //         row < emax &&
    //         row * squareStep + squareSize <= ctx.canvas.height;
    //         row++
    //     ) {
    //         const y = ctx.canvas.height - squareSize - row * squareStep;
    //         ctx.fillStyle = row < esum ? ec_full : ec_empty;
    //         ctx.fillRect(x, y, squareSize, squareSize);
    //     }
    // }
    // // Pressure draw
    // for (
    //     let column = 0;
    //     column < visibleColumns &&
    //     startColumn + column < history.psum.length;
    //     column++
    // ) {
    //     const psum = -history.psum[startColumn + column];
    //     const pmax = -history.pmax[startColumn + column];
    //     const x = column * squareStep;
    //     const yStart = 0;  // Start from top
    //     for (
    //         let row = 0;
    //         row < pmax &&
    //         row * squareStep + squareSize <= ctx.canvas.height;
    //         row++
    //     ) {
    //         const y = yStart + row * squareStep;
    //         ctx.fillStyle = row < psum ? pc_full : pc_empty;
    //         ctx.fillRect(x, y, squareSize, squareSize);
    //     }
    // }


    const diameter = draw_diam(ctx.canvas);
    const particles_ptr = world.particles();
	const particle_size = engine.Particle.size();
	const particles_view = new DataView(
		memory.buffer,
		particles_ptr,
		world.particles_count() * particle_size,
	);
    let aa = `${world.step}\t|`
    let esum = 0
    let psum = 0
    let emax = 0
    let pmax = 0
	for (let i = 0; i < world.particles_count(); i++) {
		const particle = get_particle(particles_view, particle_size, i);
        esum += particle.electricity_q
        emax += particle.electricity_c
        aa = `${aa}\t${particle.electricity_q}`
	}
    aa = `${aa}\t${esum}\t|`
    for (let i = 0; i < world.particles_count(); i++) {
		const particle = get_particle(particles_view, particle_size, i);
        aa = `${aa}\t${particle.pressure_q}`
        if (particle.kind === engine.Kind.Pump) {  // Only for pumps
            psum += particle.pressure_q
            pmax += particle.pressure_c
        }
	}
    history.esum.push(esum)
    history.psum.push(psum)
    history.emax.push(emax)
    history.pmax.push(pmax)
    if (world.step <= 24) {
        // console.log(aa)
    }
    // for (let i = 0; i < world.particles_count(); i++) {
	// 	const particle = get_particle(particles_view, particle_size, i);
    //     const p = particle.ap;
    //     ctx.beginPath();
    //     ctx.arc(p.x*diameter+center.x, -p.y*diameter+center.y, diameter/2*particle.diameter, 0, Math.PI * 2);
    //     ctx.fillStyle = engine.base_color(particle.kind);
    //     ctx.fill();
    //     if (particle.kind == engine.Kind.Led && particle.electricity_q > 0) {
    //         ctx.beginPath();
    //         ctx.arc(p.x*diameter+center.x, -p.y*diameter+center.y, diameter/2*particle.diameter*0.6, 0, Math.PI * 2);
    //         ctx.fillStyle = "#0f0";
    //         ctx.fill();
    //     }
    //     if (particle.kind == engine.Kind.Switch && particle.activation > 0) {
    //         ctx.beginPath();
    //         ctx.arc(p.x*diameter+center.x, -p.y*diameter+center.y, diameter/2*particle.diameter*0.6, 0, Math.PI * 2);
    //         ctx.fillStyle = "#0c0";
    //         ctx.fill();
    //     }
    //     if (particle.kind == engine.Kind.Switch && particle.activation == 0) {
    //         ctx.beginPath();
    //         ctx.arc(p.x*diameter+center.x, -p.y*diameter+center.y, diameter/2*particle.diameter*0.6, 0, Math.PI * 2);
    //         ctx.fillStyle = "#f00";
    //         ctx.fill();
    //     }
	// }
    // const links_ptr = world.links();
	// const link_size = engine.Link.size();
	// const links_view = new DataView(
	// 	memory.buffer,
	// 	links_ptr,
	// 	world.links_count() * link_size,
	// );

    // for (let i = 0; i < world.links_count(); i++) {
    //     const link = get_link(links_view, link_size, i);
    //     const a = get_particle(particles_view, particle_size, link.a);
    //     const b = get_particle(particles_view, particle_size, link.b);
    //     if (engine.draw_link_as_pipe(a.kind, b.kind)) {
    //         ctx.beginPath();
    //         ctx.moveTo(a.ap.x * diameter + center.x, -a.ap.y * diameter + center.y);
    //         ctx.lineTo(b.ap.x * diameter + center.x, -b.ap.y * diameter + center.y);
    //         ctx.strokeStyle = engine.base_color(a.kind);
    //         ctx.lineWidth = diameter*a.diameter*0.5;
    //         ctx.stroke();
    //     }
    // }

    

    // for (let i = 0; i < world.links_count(); i++) {
    //     const link = get_link(links_view, link_size, i);
    //     const a = get_particle(particles_view, particle_size, link.a);
    //     const b = get_particle(particles_view, particle_size, link.b);
    //     if (
    //         !engine.draw_link_as_electricty(a.kind, b.kind)
    //         && !engine.draw_signal(a.kind, b.kind)
    //         && !engine.draw_link_as_pipe(a.kind, b.kind)
    //         && !engine.draw_solid_moon(a.kind, b.kind)
    //     ) {
    //         ctx.beginPath();
    //         ctx.moveTo(a.ap.x * diameter + center.x, -a.ap.y * diameter + center.y);
    //         ctx.lineTo(b.ap.x * diameter + center.x, -b.ap.y * diameter + center.y);
    //         ctx.lineWidth=100
    //         ctx.strokeStyle = "#f8f";
    //         ctx.stroke();
    //         console.error(a.kind, b.kind)
    //     }
    // }


    // for (let i = 0; i < world.links_count(); i++) {
    //     const link = get_link(links_view, link_size, i);
    //     const a = get_particle(particles_view, particle_size, link.a);
    //     const b = get_particle(particles_view, particle_size, link.b);
    //     if (engine.draw_link_as_electricty(a.kind, b.kind)) {
    //         ctx.beginPath();
    //         ctx.moveTo(a.ap.x * diameter + center.x, -a.ap.y * diameter + center.y);
    //         ctx.lineTo(b.ap.x * diameter + center.x, -b.ap.y * diameter + center.y);
    //         ctx.strokeStyle = `#edbf`;
    //         ctx.lineWidth = 8;
    //         ctx.stroke();
    //     }
    // }

    // for (let i = 0; i < world.links_count(); i++) {
    //     const link = get_link(links_view, link_size, i);
    //     const a = get_particle(particles_view, particle_size, link.a);
    //     const b = get_particle(particles_view, particle_size, link.b);
    //     if (engine.draw_signal(a.kind, b.kind)) {
    //         ctx.beginPath();
    //         ctx.moveTo(a.ap.x * diameter + center.x, -a.ap.y * diameter + center.y);
    //         ctx.lineTo(b.ap.x * diameter + center.x, -b.ap.y * diameter + center.y);
    //         ctx.strokeStyle = `#f0f`;
    //         ctx.lineWidth = 2;
    //         ctx.stroke();
    //     }
    // }

    // for (let i = 0; i < world.links_count(); i++) {
    //     const link = get_link(links_view, link_size, i);
    //     const a = get_particle(particles_view, particle_size, link.a);
    //     const b = get_particle(particles_view, particle_size, link.b);
    //     if (engine.draw_solid_moon(a.kind, b.kind)) {
    //         ctx.beginPath();
    //         ctx.moveTo(a.ap.x * diameter + center.x, -a.ap.y * diameter + center.y);
    //         ctx.lineTo(b.ap.x * diameter + center.x, -b.ap.y * diameter + center.y);
    //         ctx.strokeStyle = `#bbb`;
    //         ctx.lineWidth = 10;
    //         ctx.stroke();
    //     }
    // }

    // for (let i = 0; i < world.links_count(); i++) {
    //     const link = get_link(links_view, link_size, i);
    //     const a = get_particle(particles_view, particle_size, link.a);
    //     const b = get_particle(particles_view, particle_size, link.b);
    //     if (engine.draw_link_as_pipe(a.kind, b.kind)) {
    //         const v = parseInt(225.0 * (a.moon_q / a.moon_c * 0.5 + 0.5))
    //         const v2 = parseInt(225.0 * (b.moon_q / b.moon_c * 0.5 + 0.5))
    //         const v3 = parseInt((v+v2)/2)
    //         ctx.beginPath();
    //         ctx.moveTo(a.ap.x * diameter + center.x, -a.ap.y * diameter + center.y);
    //         ctx.lineTo(b.ap.x * diameter + center.x, -b.ap.y * diameter + center.y);
    //         ctx.strokeStyle = `rgb(${v3}, ${v3}, ${v3})`;
    //         ctx.lineWidth = diameter*a.diameter*0.25;
    //         ctx.stroke();
    //         ctx.beginPath();
    //         ctx.arc(
    //             a.p.x*diameter+center.x, 
    //             -a.p.y*diameter+center.y, 
    //             diameter/2*a.diameter*0.5,
    //             0, Math.PI * 2);
    //         ctx.fillStyle = `rgb(${v}, ${v}, ${v})`;
    //         ctx.fill();
    //         ctx.beginPath();
    //         ctx.arc(
    //             b.p.x*diameter+center.x, 
    //             -b.p.y*diameter+center.y, 
    //             diameter/2*b.diameter*0.5,
    //             0, Math.PI * 2);
    //         ctx.fillStyle = `rgb(${v2}, ${v2}, ${v2})`;
    //         ctx.fill();
    //     } 
    // }


    // for (let i = 0; i < world.particles_count(); i++) {
	// 	const particle = get_particle(particles_view, particle_size, i);
    //     const p = particle.ap;
    //     if (particle.kind === engine.Kind.Moon) {
    //         ctx.beginPath();
    //         ctx.arc(p.x*diameter+center.x, -p.y*diameter+center.y, diameter/2*particle.diameter, 0, Math.PI * 2);
    //         ctx.fillStyle = engine.base_color(particle.kind);
    //         ctx.lineWidth=1
    //         ctx.fill();
    //     }
    //     if (particle.kind == engine.Kind.Valve && particle.activation == 0) {
    //         ctx.beginPath();
    //         ctx.arc(p.x*diameter+center.x, -p.y*diameter+center.y, diameter/2*particle.diameter*0.45, 0, Math.PI * 2);
    //         ctx.fillStyle = engine.base_color(engine.Kind.Valve);
    //         ctx.fill();
    //     }
    // }


    // Debug
    for (let i = 0; i < world.particles_count(); i++) {
        const p = get_particle(particles_view, particle_size, i);
            const x = p.ap.x * diameter + center.x;
            const y = -p.ap.y * diameter + center.y;
            const l = 3
            const label = `p:${String(p.pressure_q).padStart(l, " ")}/${p.pressure_c}`;
            ctx.font = '16px monospace';
            ctx.textAlign = 'left';
            ctx.textBaseline = 'middle';
            const labelX = x  ;
            const labelY = y + 20*3;
            const padding = 3;
            const labelWidth = ctx.measureText(label).width;
            ctx.fillStyle = '#111';
            ctx.fillRect(
                labelX - padding,
                labelY - 8 - padding,
                labelWidth + padding * 2,
                16 + padding * 2,
            );
            ctx.fillStyle = "#eee";
            ctx.fillText(label, labelX, labelY);
    }
    for (let i = 0; i < world.particles_count(); i++) {
        const p = get_particle(particles_view, particle_size, i);
            const x = p.ap.x * diameter + center.x;
            const y = -p.ap.y * diameter + center.y;
            const l = String(p.moon_c).length
            const label = `m:${String(p.moon_q).padStart(l, " ")}/${p.moon_c}`;
            ctx.font = '16px monospace';
            ctx.textAlign = 'left';
            ctx.textBaseline = 'middle';
            const labelX = x;
            const labelY = y + 20;
            const padding = 3;
            const labelWidth = ctx.measureText(label).width;
            ctx.fillStyle = '#111d';
            ctx.fillRect(
                labelX - padding,
                labelY - 8 - padding,
                labelWidth + padding * 2,
                16 + padding * 2,
            );
            ctx.fillStyle = "#eee";
            ctx.fillText(label, labelX, labelY);
    }

    for (let i = 0; i < world.particles_count(); i++) {
        const p = get_particle(particles_view, particle_size, i);
            const x = p.ap.x * diameter + center.x;
            const y = -p.ap.y * diameter + center.y;
            const l = String(p.electricity_c).length
            const label = `e:${String(p.electricity_q).padStart(l, " ")}/${p.electricity_c}`;
            ctx.font = '16px monospace';
            ctx.textAlign = 'left';
            ctx.textBaseline = 'middle';
            const labelX = x;
            const labelY = y + 20*2;
            const padding = 3;
            const labelWidth = ctx.measureText(label).width;
            ctx.fillStyle = '#111';
            ctx.fillRect(
                labelX - padding,
                labelY - 8 - padding,
                labelWidth + padding * 2,
                16 + padding * 2,
            );
            ctx.fillStyle = "#eee";
            ctx.fillText(label, labelX, labelY);
    }


    for (let i = 0; i < world.particles_count(); i++) {
        const p = get_particle(particles_view, particle_size, i);
            const x = p.ap.x * diameter + center.x;
            const y = -p.ap.y * diameter + center.y;
            const l = 3
            const label = `a:${String(p.activation).padStart(l, " ")}`;
            ctx.font = '16px monospace';
            ctx.textAlign = 'left';
            ctx.textBaseline = 'middle';
            const labelX = x;
            const labelY = y + 20*4;
            const padding = 3;
            const labelWidth = ctx.measureText(label).width;
            ctx.fillStyle = '#111';
            ctx.fillRect(
                labelX - padding,
                labelY - 8 - padding,
                labelWidth + padding * 2,
                16 + padding * 2,
            );
            ctx.fillStyle = "#eee";
            ctx.fillText(label, labelX, labelY);
    }

    for (let i = 0; i < world.particles_count(); i++) {
        const p = get_particle(particles_view, particle_size, i);
            const x = p.ap.x * diameter + center.x;
            const y = -p.ap.y * diameter + center.y;
            const l = 3
            const label = `s:${String(p.signal_q).padStart(l, " ")}`;
            ctx.font = '16px monospace';
            ctx.textAlign = 'left';
            ctx.textBaseline = 'middle';
            const labelX = x;
            const labelY = y + 20*5;
            const padding = 3;
            const labelWidth = ctx.measureText(label).width;
            ctx.fillStyle = '#111';
            ctx.fillRect(
                labelX - padding,
                labelY - 8 - padding,
                labelWidth + padding * 2,
                16 + padding * 2,
            );
            ctx.fillStyle = "#eee";
            ctx.fillText(label, labelX, labelY);
    }


    for (let i = 0; i < world.particles_count(); i++) {
        const p = get_particle(particles_view, particle_size, i);
            const x = p.ap.x * diameter + center.x;
            const y = -p.ap.y * diameter + center.y;
            const l = String(p.smc).length
            const label = `sm:${String(p.smq).padStart(l, " ")}/${p.smc}`;
            ctx.font = '16px monospace';
            ctx.textAlign = 'left';
            ctx.textBaseline = 'middle';
            const labelX = x;
            const labelY = y + 20*6;
            const padding = 3;
            const labelWidth = ctx.measureText(label).width;
            ctx.fillStyle = '#111';
            ctx.fillRect(
                labelX - padding,
                labelY - 8 - padding,
                labelWidth + padding * 2,
                16 + padding * 2,
            );
            ctx.fillStyle = "#eee";
            ctx.fillText(label, labelX, labelY);
    }
    
    
}


export {
    draw,
}
```

## file://./../../../gravitle/front/moonlighter/particle.js

```javascript
const get_particle = (particles_view, particle_size, idx) => {
	const particle = {
		index: particles_view.getUint32(idx * particle_size + 0, true),
		diameter: particles_view.getFloat32(idx * particle_size + 4, true),
		p: {
			x: particles_view.getFloat32(idx * particle_size + 8, true),
			y: particles_view.getFloat32(idx * particle_size + 12, true),
		},
		pp: {
			x: particles_view.getFloat32(idx * particle_size + 16, true),
			y: particles_view.getFloat32(idx * particle_size + 20, true),
		},
		np: {
			x: particles_view.getFloat32(idx * particle_size + 24, true),
			y: particles_view.getFloat32(idx * particle_size + 28, true),
		},
		dp: {
			x: particles_view.getFloat32(idx * particle_size + 32, true),
			y: particles_view.getFloat32(idx * particle_size + 36, true),
		},
		direction: {
			x: particles_view.getFloat32(idx * particle_size + 40, true),
			y: particles_view.getFloat32(idx * particle_size + 44, true),
		},
		dv: {
			x: particles_view.getFloat32(idx * particle_size + 48, true),
			y: particles_view.getFloat32(idx * particle_size + 52, true),
		},
		link_response: {
			x: particles_view.getFloat32(idx * particle_size + 56, true),
			y: particles_view.getFloat32(idx * particle_size + 60, true),
		},
		collision_response: {
			x: particles_view.getFloat32(idx * particle_size + 64, true),
			y: particles_view.getFloat32(idx * particle_size + 68, true),
		},
		collision_response_count: particles_view.getUint32(
			idx * particle_size + 72,
			true,
		),
		kind: particles_view.getUint32(idx * particle_size + 76, true),
		pressure_c: particles_view.getInt32(idx * particle_size + 80, true),
		pressure_q: particles_view.getInt32(idx * particle_size + 84, true),
		pressure_qd: particles_view.getInt32(idx * particle_size + 88, true),
		moon_c: particles_view.getInt32(idx * particle_size + 92, true),
		moon_q: particles_view.getInt32(idx * particle_size + 96, true),
		moon_qd: particles_view.getInt32(idx * particle_size + 100, true),
		electricity_c: particles_view.getInt32(idx * particle_size + 104, true),
		electricity_q: particles_view.getInt32(idx * particle_size + 108, true),
		electricity_qd: particles_view.getInt32(idx * particle_size + 112, true),
		activation: particles_view.getInt32(idx * particle_size + 116, true),
		signal_q: particles_view.getInt32(idx * particle_size + 120, true),
		signal_qn: particles_view.getInt32(idx * particle_size + 124, true),
		smc: particles_view.getInt32(idx * particle_size + 128, true),
		smq: particles_view.getInt32(idx * particle_size + 132, true),
		smqd: particles_view.getInt32(idx * particle_size + 136, true),
	};
	particle.ap = {
		x: (particle.p.x + particle.pp.x) * 0.5,
		y: (particle.p.y + particle.pp.y) * 0.5,
	};
	return particle;
};
export { get_particle };
```

## file://./../../../gravitle/front/moonlighter/webgpu.js

```javascript
import { ZOOM } from "./consts.js";

// Rust `Particle` is #[repr(C)] with 4-byte-aligned fields; the WGSL
// `GpuParticle` struct mirrors it exactly, so the raw wasm bytes can be
// copied to the GPU as-is.
const COLOR_STRIDE = 4; // floats per particle color (vec4f)
const LINK_STRIDE = 3; // u32 per Link (idx, a, b)

const color_cache = new Map();

// Parse engine.base_color output: "#eee", "#48f" or "rgb(r, g, b)"
const parse_color = (s) => {
    if (color_cache.has(s)) {
        return color_cache.get(s);
    }
    let rgba;
    if (s.startsWith("#")) {
        const hex = s.slice(1);
        const n = hex.length === 3
            ? hex.split("").map((c) => c + c).join("")
            : hex;
        rgba = [
            parseInt(n.slice(0, 2), 16) / 255,
            parseInt(n.slice(2, 4), 16) / 255,
            parseInt(n.slice(4, 6), 16) / 255,
            1.0,
        ];
    } else {
        const m = s.match(/[\d.]+/g);
        rgba = [
            parseFloat(m[0]) / 255,
            parseFloat(m[1]) / 255,
            parseFloat(m[2]) / 255,
            1.0,
        ];
    }
    color_cache.set(s, rgba);
    return rgba;
}

const make_triangle_pipeline = async (device, format) => {
    const shader_src = await (
        await fetch("./shader.wgsl", { cache: "no-store" })
    ).text();
    const module = device.createShaderModule({ code: shader_src });
    const pipeline = device.createRenderPipeline({
        layout: 'auto',
        vertex: { module, entryPoint: 'vs_main' },
        fragment: { module, entryPoint: 'fs_main', targets: [{ format }] },
        primitive: { topology: 'triangle-list' },
    });
    const uniform_buffer = device.createBuffer({
        size: 32,
        usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
    });
    return {
        pipeline,
        uniform_buffer,
        particle_buffer: null,
        color_buffer: null,
        link_buffer: null,
        particle_capacity: 0,
        link_capacity: 0,
        bind_group_with_particles: null,
    };
}

// Lazily (re)allocate the storage buffers when the count changes,
// and rebuild the bind group accordingly.
const ensure_storage = (device, gpu_pipeline, count, particle_size, link_count) => {
    const particle_ok =
        gpu_pipeline.particle_buffer !== null &&
        gpu_pipeline.particle_capacity >= count;
    const link_ok =
        gpu_pipeline.link_buffer !== null && gpu_pipeline.link_capacity >= link_count;
    if (particle_ok && link_ok) {
        return;
    }
    if (gpu_pipeline.particle_buffer !== null) {
        gpu_pipeline.particle_buffer.destroy();
        gpu_pipeline.color_buffer.destroy();
        gpu_pipeline.link_buffer.destroy();
    }
    const n = Math.max(1, count);
    gpu_pipeline.particle_buffer = device.createBuffer({
        size: n * particle_size,
        usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST,
    });
    gpu_pipeline.color_buffer = device.createBuffer({
        size: n * COLOR_STRIDE * 4,
        usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST,
    });
    const l = Math.max(1, link_count);
    gpu_pipeline.link_buffer = device.createBuffer({
        size: l * LINK_STRIDE * 4,
        usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST,
    });
    gpu_pipeline.particle_capacity = n;
    gpu_pipeline.link_capacity = l;
    gpu_pipeline.bind_group_with_particles = device.createBindGroup({
        layout: gpu_pipeline.pipeline.getBindGroupLayout(0),
        entries: [
            {
                binding: 0,
                resource: { buffer: gpu_pipeline.uniform_buffer },
            },
            {
                binding: 1,
                resource: { buffer: gpu_pipeline.particle_buffer },
            },
            {
                binding: 2,
                resource: { buffer: gpu_pipeline.color_buffer },
            },
            {
                binding: 3,
                resource: { buffer: gpu_pipeline.link_buffer },
            },
        ],
    });
}

const draw_triangle_webgpu = (
    device,
    context,
    gpu_pipeline,
    world,
    engine,
    memory,
) => {
    const height = context.canvas.height;
    const width = context.canvas.width;
    device.queue.writeBuffer(
        gpu_pipeline.uniform_buffer,
        0,
        new Float32Array([
            width,
            height,
            ZOOM,
            performance.now() / 1000, // time in seconds
            world.completed_steps,
            Math.random(),            // fresh random per frame
            0.0,
            0.0,
        ]),
    );

    const count = world.particles_count();
    const link_count = world.links_count();
    const particle_size = engine.Particle.size();
    ensure_storage(device, gpu_pipeline, count, particle_size, link_count);
    if (gpu_pipeline.bind_group_with_particles === null) {
        return;
    }

    // Raw Particle structs, mirrored 1:1 by the WGSL `GpuParticle` struct:
    // upload the wasm bytes directly, no per-field reads or staging copy.
    const f32_per_particle = particle_size / 4;
    const particles_data = new Float32Array(
        memory.buffer,
        world.particles(),
        count * f32_per_particle,
    );
    device.queue.writeBuffer(
        gpu_pipeline.particle_buffer,
        0,
        particles_data,
        0,
        count * f32_per_particle,
    );

    // if (link_count > 0) {
        const links_data = new Uint32Array(memory.buffer, world.links(), link_count * LINK_STRIDE);
        device.queue.writeBuffer(gpu_pipeline.link_buffer, 0, links_data);
    // }

    // Colors must be computed in JS (engine.base_color); `kind` is read
    // directly from the same wasm memory (offset 76 bytes = 19 floats).
    const u32_per_particle = particle_size / 4;
    const kinds = new Uint32Array(
        memory.buffer,
        world.particles(),
        count * u32_per_particle,
    );
    const colors = new Float32Array(count * COLOR_STRIDE);
    for (let i = 0; i < count; i++) {
        const kind = kinds[i * u32_per_particle + 19];
        const [r, g, b, a] = parse_color(engine.base_color(kind));
        const o = i * COLOR_STRIDE;
        colors[o + 0] = r;
        colors[o + 1] = g;
        colors[o + 2] = b;
        colors[o + 3] = a;
    }
    device.queue.writeBuffer(gpu_pipeline.color_buffer, 0, colors);

    const encoder = device.createCommandEncoder();
    const pass = encoder.beginRenderPass({
        colorAttachments: [{
            view: context.getCurrentTexture().createView(),
            clearValue: { r: 0, g: 0, b: 0, a: 0 },
            loadOp: 'clear',
            storeOp: 'store',
        }],
    });
    pass.setPipeline(gpu_pipeline.pipeline);
    pass.setBindGroup(0, gpu_pipeline.bind_group_with_particles);
    pass.draw(9 + 6 * count + 6 * link_count);
    pass.end();
    device.queue.submit([encoder.finish()]);
}


export {
    draw_triangle_webgpu,
    make_triangle_pipeline,
}
```

## file://./../../../gravitle/front/moonlighter/render.js

```javascript
import { resize_canvas } from "./canvas.js";


const setup_rendering = () => {
    const canvas = document.createElement('canvas');
    canvas.id = 'canvas';
    canvas.style.zIndex = '1'
    canvas.style.background = 'transparent';
    document.body.appendChild(canvas);
    resize_canvas(canvas);
    window.addEventListener('resize', () => {resize_canvas(canvas)});
    return canvas.getContext('2d');
}


const setup_rendering_webgpu = async () => {
    if (!navigator.gpu) {
        console.warn("WebGPU not supported, skipping overlay canvas");
        return null;
    }
    const adapter = await navigator.gpu.requestAdapter();
    if (!adapter) {
        console.warn("No WebGPU adapter, skipping overlay canvas");
        return null;
    }
    const device = await adapter.requestDevice();

    const canvas = document.createElement('canvas');
    canvas.id = 'canvas_webgpu';
    canvas.style.position = 'fixed';
    canvas.style.inset = '0';
    canvas.style.pointerEvents = 'none';
    document.body.appendChild(canvas);
    resize_canvas(canvas);
    window.addEventListener('resize', () => {resize_canvas(canvas)});

    const context = canvas.getContext('webgpu');
    const format = navigator.gpu.getPreferredCanvasFormat();
    context.configure({ device, format, alphaMode: 'premultiplied' });

    return { device, context, format };
}


export {
    setup_rendering,
    setup_rendering_webgpu,
}
```

## file://./../../../gravitle/front/moonlighter/shader.wgsl

```text
const LINE_HALF_WIDTH: f32 = 2.0;


struct Uniforms {
    canvas_size: vec2f,
    zoom: f32,
    time: f32,
    step: f32,
    rnd: f32,
    link_count: f32,
    _pad: f32,
};

// Mirrors `Particle` in engine/src/particle.rs (`#[repr(C)]`, 4-byte aligned).
// Fields and offsets must be kept in sync with the Rust struct.
struct GpuParticle {
    idx: u32,
    diameter: f32,
    p: vec2f,                 // position
    pp: vec2f,                // previous position
    np: vec2f,                // new position
    dp: vec2f,                // delta position
    direction: vec2f,
    dv: vec2f,                // delta velocity
    link_response: vec2f,
    collision_response: vec2f,
    collision_response_count: u32,
    kind: u32,
    pressure_c: i32,
    pressure_q: i32,
    pressure_qn: i32,
    moon_c: i32,
    moon_q: i32,
    moon_qd: i32,
    electricity_c: i32,
    electricity_q: i32,
    electricity_qd: i32,
    activation: i32,
    signal_q: i32,
    signal_qn: i32,
    solidmoon_c: i32,
    solidmoon_q: i32,
    solidmoon_qd: i32,
    padding: u32,
};

struct GpuLink {
    idx: u32,
    a: u32,
    b: u32,
};

@group(0) @binding(0) var<uniform> u: Uniforms;
@group(0) @binding(1) var<storage, read> particles: array<GpuParticle>;
// rgba per particle, computed in JS from engine.base_color
@group(0) @binding(2) var<storage, read> colors: array<vec4f>;
@group(0) @binding(3) var<storage, read> links: array<GpuLink>;


fn px_to_ndc(p: vec2f) -> vec2f {
    let norm = p / u.canvas_size;
    return vec2f(norm.x * 2.0 - 1.0, 1.0 - norm.y * 2.0);
}

fn quad_corners(x1: f32, y1: f32, x2: f32, y2: f32) -> array<vec2f, 6> {
    return array<vec2f, 6>(
        vec2f(x1, y1),
        vec2f(x2, y1),
        vec2f(x2, y2),
        vec2f(x1, y1),
        vec2f(x2, y2),
        vec2f(x1, y2),
    );
}

struct VertexOut {
    @builtin(position) pos: vec4f,
    @location(0) color: vec4f,
    // Local coordinates of the fragment relative to the center of the shape,
    // expressed in units of the shape's radius. Fragments with
    // `length(uv) > 1.0` are discarded for round shapes.
    @location(1) uv: vec2f,
    @location(2) moon_ratio: f32, // quantity / capacity
    @location(3) @interpolate(flat) kind: u32,
    // 1.0 for link vertices: disables the disc discard in fs_main
    @location(4) @interpolate(flat) is_link: f32,
};


// Cheap hash-based value noise in [0, 1), seeded by time, step and
// a per-frame random number coming from JS.
fn hash(p: vec3f) -> f32 {
    var v = fract(sin(dot(p, vec3f(127.1, 311.7, 74.7))) * 43758.5453123);
    return v;
}


@vertex
fn vs_main(@builtin(vertex_index) i: u32) -> VertexOut {
    var out: VertexOut;
    out.uv = vec2f(0.0, 0.0);
    out.color = vec4f(1.0);
    // out.moon_ratio = 1.0;
    // out.kind = 0;
    if (i < 3u) {
        // Debug triangle
        var positions = array<vec2f, 3>(
            vec2f( 0.0,  0.6),
            vec2f(-0.6, -0.6),
            vec2f( 0.6, -0.6),
        );
        var colors = array<vec3f, 3>(
            vec3f(1.0, 0.2, 0.2),
            vec3f(0.2, 1.0, 0.2),
            vec3f(0.2, 0.2, 1.0),
        );
        out.pos = vec4f(positions[i], 0.0, 1.0);
        out.color = vec4f(colors[i], 0.8);
        out.color = vec4f(0.0);
    } else if (i < 9u) {
        // Debug 100x100 px square in the top-left corner (pixel coords, y down)
        out.pos = vec4f(px_to_ndc(quad_corners(0.0, 0.0, 100.0, 100.0)[i - 3u]), 0.0, 1.0);
        out.color = vec4f(1.0, 0.8, 0.2, 0.8);
    } else {
        // One quad (drawn as a disc) per particle
        let pi = (i - 9u) / 6u;
        let corner = quad_corners(-1.0, -1.0, 1.0, 1.0)[(i - 9u) % 6u];
        let particle = particles[pi];
        let aa = min(u.canvas_size.x, u.canvas_size.y) * u.zoom;
        let center = vec2f(
            u.canvas_size.x * 0.5 + particle.p.x * aa,
            u.canvas_size.y * 0.5 - particle.p.y * aa,
        );
        let radius = particle.diameter * 0.5 * aa;
        let c = center + corner * radius;
        out.pos = vec4f(px_to_ndc(c), 0.0, 1.0);
        out.color = colors[pi];
        out.uv = corner;
        out.kind = particle.kind;
        out.moon_ratio = f32(particle.moon_q) / f32(particle.moon_c);
    }
    return out;
}

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4f {
    if (length(in.uv) > 1.0) {
        discard;
    }
    if (
        (
            in.kind == 1 
            || in.kind == 2 
            || in.kind == 3 
            || in.kind == 10
            || in.kind == 4
            || in.kind == 19
        )
        && length(in.uv) < 0.75
    ) {
        var seed = vec3f(
            in.uv * 24.0 + u.canvas_size * 0.013,
            19.0,
        );
        let n = hash(floor(seed));
        let aa = select(0.0, 1.0, n < in.moon_ratio * 0.5);
        return vec4f(aa, aa, aa, 1.0);
    } else {
        return in.color;
    }
};
```

