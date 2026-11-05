use crate::world::Kind;
use crate::world::World;

pub fn activation(world: &mut World) {
    for i1 in 0..world.particles.len() {
        match world.p(i1).kind {
            Kind::SensorMoonFull => {
                world.pm(i1).signal_q = 0;
                for i2 in world.ns(i1) {
                    let p2 = world.p(i2);
                    if p2.moon_c > 0 && p2.moon_c == p2.moon_q {
                        world.pm(i1).signal_q = 1;
                    }
                }
                world.pm(i1).signal_qn = world.pm(i1).signal_q;
            }
            Kind::SensorMoonEmpty => {
                world.pm(i1).signal_q = 0;
                for i2 in world.ns(i1) {
                    let p2 = world.p(i2);
                    if p2.moon_c > 0 && p2.moon_q == 0 {
                        world.pm(i1).signal_q = -1;
                    }
                }
                world.pm(i1).signal_qn = world.pm(i1).signal_q;
            }
            _ => {}
        }
    }
}
