use crate::world::Kind;
use crate::world::World;
use std::collections::BTreeSet;

pub fn generate(world: &mut World) {
    for particle in &mut world.particles {
        if particle.kind == Kind::SolarPanel {
            particle.electricity_qd += 1;
        }
    }
}

pub fn equalize(world: &mut World, pairs: &BTreeSet<(usize, usize)>) {
    for (a, b) in pairs {
        let source = &world.particles[*a];
        let destination = &world.particles[*b];
        let dest_c = destination.electricity_c;
        let source_c = source.electricity_c;
        // we don't want to consider particles
        // if they can't hold electricity
        if source_c <= 0 || dest_c <= 0 {
            continue;
        }
        // if the switch is turned off, it can not receive electricity
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
            world.particles[*a].electricity_qd -= 1;
            world.particles[*b].electricity_qd += 1;
        }
    }
}
