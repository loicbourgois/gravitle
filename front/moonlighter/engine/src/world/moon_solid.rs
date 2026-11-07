use crate::world::Kind;
use crate::world::World;
use std::collections::BTreeSet;
use crate::state::State;

pub fn launch(world: &mut World) {
    // Launch
    let mut particles_to_add = Vec::new();
    for i1 in 0..world.particles.len() {
        match world.p(i1).kind {
            Kind::Launcher => {
                if world.p(i1).solidmoon_q == 1 {
                    world.pm(i1).solidmoon_q = 0;
                    particles_to_add.push((world.p(i1).p.x, world.p(i1).p.y, Kind::MoonBall));
                }
            }
            _ => {}
        }
    }
    for p in particles_to_add {
        world.add_particle(p.0, p.1, p.2);
    }
}

pub fn move_(world: &mut World, pairs: &BTreeSet<(usize, usize)>) {
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
    for p in &mut world.particles {
        p.solidmoon_qd = 0;
    }
    for (a, b) in pairs {
        let source = &world.particles[*a];
        let dest = &world.particles[*b];
        let dest_q = dest.solidmoon_q + dest.solidmoon_qd;
        let dest_c = dest.solidmoon_c;
        let valid_source_dest = matches!(
            (source.kind, dest.kind),
            (Kind::Press, Kind::Conveyor) | (Kind::Conveyor, Kind::Launcher)
        );
            // is not a valid source->dest pair 
        if  !valid_source_dest 
            // source is empty
            || source.solidmoon_q <= 0 
            // dest is full
            || dest_q >= dest_c 
            // press is not ready
            || (source.kind == Kind::Press && source.state != State::ReadyFull )
            // 
            || (source.kind == Kind::Conveyor && source.state != State::ReadyEnd )
            // 
            || (dest.kind == Kind::Conveyor && dest.state != State::ReadyStart )
        {
            continue;
        }
        world.particles[*a].solidmoon_qd -= 1;
        world.particles[*b].solidmoon_qd += 1;
    }
    for p in &mut world.particles {
        p.solidmoon_q += p.solidmoon_qd;
    }
}



// let source = &world.particles[*a];
//         let destination = &world.particles[*b];
//             // Invalid pressure between destination and source
//         if  destination.pressure_q >= source.pressure_q
//             // Source is empty now
//             || source.moon_q <= 0
//             // Source would be empty next turn
//             || source.moon_q + source.moon_qd <= 0
//             // Destination is full
//             || destination.moon_q >= destination.moon_c
//             || destination.moon_q + destination.moon_qd >= destination.moon_c
//             // a Press already holding a solid moon
//             // can not store more dust until it is picked up
//             || (destination.kind == Kind::Press && destination.solidmoon_q > 0)
//             // a Press in progress can not let go dust
//             || (source.kind == Kind::Press && matches!(source.state, State::LoadingMoonDust))
//             // ..
//             || (destination.kind == Kind::Filter && world.rng.next_f32() >= FILTER_PASS_PROBABILITY)
//             // .. 
//             || (destination.kind == Kind::Filter && source.kind == Kind::Filter)
//         {
//             continue;
//         }