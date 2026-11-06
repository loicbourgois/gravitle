use crate::world::World;
use std::collections::BTreeSet;
pub fn reset_particles(world: &mut World) {
    for particle in &mut world.particles {
        particle.pressure_qn = 0;
        particle.moon_qd = 0;
        particle.electricity_qd = 0;
        particle.signal_qn = 0;
        particle.solidmoon_qd = 0;
        particle.progress_qd = 0;
    }
}
pub fn reset_pairs(world: &mut World) -> BTreeSet<(usize, usize)> {
    let mut pairs: BTreeSet<(usize, usize)> = BTreeSet::new();
    for link in &world.links {
        let a = link.a as usize;
        let b = link.b as usize;
        pairs.insert((a, b));
        pairs.insert((b, a));
    }
    pairs
}
