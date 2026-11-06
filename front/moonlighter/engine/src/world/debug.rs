use crate::world::World;

pub fn assert(world: &mut World) {
    // Debug
    for particle in &mut world.particles {
        assert!(particle.moon_q <= particle.moon_c);
        assert!(particle.electricity_q <= particle.electricity_c);
        assert!(particle.electricity_q >= 0);
    }
}
