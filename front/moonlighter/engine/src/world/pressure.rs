use crate::world::Kind;
use crate::world::World;
use std::collections::BTreeSet;

pub fn generate(world: &mut World) {
    for particle in &mut world.particles {
        let eq = particle.electricity_q + particle.electricity_qd;
        match (particle.kind, eq > 0) {
            // pump with energy available
            (Kind::Pump, true) => {
                particle.electricity_qd -= 1;
                particle.pressure_qn = particle.pressure_q - 1;
                particle.pressure_qn = particle.pressure_qn.max(particle.pressure_c);
            }
            // pump without energy available
            (Kind::Pump, false) => {
                particle.pressure_qn = particle.pressure_q + 1;
                particle.pressure_qn = particle.pressure_qn.min(0);
            }
            // blower with energy available
            (Kind::Blower, true) => {
                particle.electricity_qd -= 1;
                particle.pressure_qn = particle.pressure_q + 1;
                particle.pressure_qn = particle.pressure_qn.min(particle.pressure_c);
            }
            // blower without energy available
            (Kind::Blower, false) => {
                particle.pressure_qn = particle.pressure_q - 1;
                particle.pressure_qn = particle.pressure_qn.max(0);
            }
            _ => {}
        }
    }
}

pub fn equalize(world: &mut World, pairs: &BTreeSet<(usize, usize)>) {
    for aidx in 0..world.particles.len() {
        let ak = world.particles[aidx].kind;
        let aa = world.particles[aidx].activation;
        match ak {
            // For these kinds as destination,
            // we always update the pressure
            Kind::Pipe | Kind::Tank | Kind::Filter | Kind::Press => {}
            // For the valve, we only proceed if it is open
            // closed: activation == 0
            // open:   activation == 1
            Kind::Valve if aa == 1 => {}
            // For other kinds, we don't transfer pressure.
            _ => continue,
        }
        let mut total: f32 = 0.0;
        let mut count: f32 = 0.0;
        for &(a, b) in pairs {
            if a != aidx {
                continue;
            }
            let pb = &world.particles[b];
            match (pb.kind, pb.activation) {
                // a closed valve blocks pressure transfer
                (Kind::Valve, 0) => {}
                (Kind::Valve, _)
                | (Kind::Tank, _)
                | (Kind::Pipe, _)
                | (Kind::Pump, _)
                | (Kind::Blower, _)
                | (Kind::Filter, _)
                | (Kind::Press, _) => {
                    total += pb.pressure_q as f32;
                    count += 1.0;
                }
                _ => {}
            }
        }
        if count > 0.0 {
            let average = total / count;
            world.particles[aidx].pressure_qn = if average > 0.0 {
                average.ceil() as i32
            } else {
                average.floor() as i32
            };
        }
    }
}


pub fn apply(world: &mut World) {
    for particle in &mut world.particles {
        particle.pressure_q = particle.pressure_qn;
    }
}


pub fn block(world: &mut World, pairs: &BTreeSet<(usize, usize)>) {
    for aidx in 0..world.particles.len() {
        if world.particles[aidx].kind == Kind::Filter
            && world.particles[aidx].moon_q >= world.particles[aidx].moon_c
        {
            // If filter is full, blocks the pressure equalization
            if let Some(pressure) = pairs
                .iter()
                .filter(|&&(a, b)| {
                    a == aidx
                        && !(
                            // filter out neighbour if it's a closed valve
                            world.particles[b].kind == Kind::Valve
                                && world.particles[b].activation == 0
                        )
                })
                .map(|&(_, b)| world.particles[b].pressure_q)
                // take the value of the neighbour closest to 0
                // 2 | -1 -> -1
                // 2 | -5 -> -5
                // 2 | 10 -> 2
                // .min_by_key(|&pressure| pressure.abs())
                .max()
            {
                world.particles[aidx].pressure_q = pressure;
            }
        } else {
            // pass
        }
    }
}

pub fn leak(world: &mut World) {
    // Without leaks, pressure equalize fully.
    // Helps with ceil / floor logic.
    // Also helps with blocking logic.
    for particle in &mut world.particles {
        match particle.kind {
            Kind::Pump | Kind::Blower => {}
            _ => {
                particle.pressure_q -= particle.pressure_q.signum();
            }
        }
    }
}
