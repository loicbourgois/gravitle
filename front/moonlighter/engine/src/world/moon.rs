use crate::world::Kind;
use crate::world::World;
pub fn dust_to_solid(world: &mut World) {
    // Dust -> Solid
    for i1 in 0..world.particles.len() {
        let p1 = world.pm(i1);
        match p1.kind {
            Kind::Press => {
                if p1.moon_q == p1.moon_c && p1.solidmoon_q == 0 && p1.progress_q < p1.progress_c {
                    p1.progress_q += 1
                } else if p1.moon_q == p1.moon_c
                    && p1.solidmoon_q == 0
                    && p1.progress_q == p1.progress_c
                {
                    p1.moon_q = 0;
                    p1.solidmoon_q = 1;
                    p1.progress_q = 0;
                }
            }
            _ => {}
        }
    }
}
