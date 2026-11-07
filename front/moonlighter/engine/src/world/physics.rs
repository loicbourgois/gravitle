use crate::world::Kind;
use crate::world::World;
pub fn update(world: &mut World) {
    // Particle Movement
    for p in &mut world.particles {
        p.pp.x = p.p.x;
        p.pp.y = p.p.y;
    }
    for p in &mut world.particles {
        match p.kind {
            Kind::MoonBall => {
                p.p.y += 0.01;
                p.p.x -= 0.01;
            }
            _ => {}
        }
    }

    // Particle Collision
    let mut crs = Vec::new();
    for p1 in &world.particles {
        for p2 in &world.particles {
            if p1.kind == Kind::MoonBall && p2.kind == Kind::Earth && World::colision(p1, p2) {
                crs.push((p1.idx as usize, p2.idx as usize));
            }
        }
    }
    for cr in crs {
        world.particles[cr.0].kind = Kind::Dead;
        world.particles[cr.1].solidmoon_q += 1;
    }
}
