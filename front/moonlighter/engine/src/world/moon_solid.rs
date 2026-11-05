use crate::world::Kind;
use crate::world::World;
use std::collections::BTreeSet;

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
        let source_kind = world.particles[*a].kind;
        let dest_kind = world.particles[*b].kind;
        let mut source_q = world.particles[*a].solidmoon_q + world.particles[*a].solidmoon_qd;
        source_q = source_q.min(world.particles[*a].solidmoon_q);
        let dest_q = world.particles[*b].solidmoon_q + world.particles[*b].solidmoon_qd;
        let dest_c = world.particles[*b].solidmoon_c;
        let valid = matches!(
            (source_kind, dest_kind),
            (Kind::Press, Kind::Conveyor) | (Kind::Conveyor, Kind::Launcher)
        );
        if !valid || source_q <= 0 || dest_q >= dest_c {
            continue;
        }
        world.particles[*a].solidmoon_qd -= 1;
        world.particles[*b].solidmoon_qd += 1;
    }
    for p in &mut world.particles {
        p.solidmoon_q += p.solidmoon_qd;
    }
}
