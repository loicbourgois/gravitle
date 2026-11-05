use crate::world::Kind;
use crate::world::World;
use std::collections::BTreeSet;

// The probability for dust to get into the filter
// and start clogging it.
const FILTER_PASS_PROBABILITY: f32 = 0.10;

pub fn equalize(world: &mut World, pairs: &BTreeSet<(usize, usize)>) {
    for (a, b) in pairs {
        let source = &world.particles[*a];
        let destination = &world.particles[*b];
        if destination.pressure_q >= source.pressure_q
            || source.moon_q <= 0
            || source.moon_q + source.moon_qd <= 0
            || destination.moon_q >= destination.moon_c
            || destination.moon_q + destination.moon_qd >= destination.moon_c
            // a Press already holding a solid moon
            // must not store more dust until it is picked up
            || (destination.kind == Kind::Press && destination.solidmoon_q > 0)
            // a Press in progress can not let go dust
            || (source.kind == Kind::Press && source.progress_q > 0)
        {
            continue;
        }
        if destination.kind == Kind::Filter && world.rng.next_f32() >= FILTER_PASS_PROBABILITY {
            continue;
        }
        if destination.kind == Kind::Filter && source.kind == Kind::Filter {
            continue;
        }
        world.particles[*a].moon_qd -= 1;
        world.particles[*b].moon_qd += 1;
    }
}
