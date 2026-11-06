use crate::world::Kind;
use crate::world::World;
use std::collections::BTreeSet;
use crate::particle::State;

// The probability for dust to get into the filter
// and start clogging it.
const FILTER_PASS_PROBABILITY: f32 = 0.10;

pub fn equalize(world: &mut World, pairs: &BTreeSet<(usize, usize)>) {
    for (a, b) in pairs {
        let source = &world.particles[*a];
        let destination = &world.particles[*b];
            // Invalid pressure between destination and source
        if  destination.pressure_q >= source.pressure_q
            // Source is empty now
            || source.moon_q <= 0
            // Source would be empty next turn
            || source.moon_q + source.moon_qd <= 0
            // Destination is full
            || destination.moon_q >= destination.moon_c
            || destination.moon_q + destination.moon_qd >= destination.moon_c
            // a Press already holding a solid moon
            // can not store more dust until it is picked up
            || (destination.kind == Kind::Press && destination.state != State::LoadingMoonDust)
            // a Press in progress can not let go dust
            || (source.kind == Kind::Press && source.state != State::LoadingMoonDust)
            // ..
            || (destination.kind == Kind::Filter && world.rng.next_f32() >= FILTER_PASS_PROBABILITY)
            // .. 
            || (destination.kind == Kind::Filter && source.kind == Kind::Filter)
        {
            continue;
        }
        world.particles[*a].moon_qd -= 1;
        world.particles[*b].moon_qd += 1;
    }
}

pub fn apply(world: &mut World) {
    for particle in &mut world.particles {
        particle.moon_q += particle.moon_qd;
    }
}
