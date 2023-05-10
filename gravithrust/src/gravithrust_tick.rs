use crate::alchemy::alchemy_transfer;
use crate::error;
use crate::gravithrust::Gravithrust;
use crate::grid::grid_id_position;
use crate::grid::Grid;
use crate::kind::Kind;
// use crate::kind::Kind;
use crate::link::Link;
use crate::link::LinkJS;
use crate::math::normalize;
use crate::math::wrap_around;
use crate::math::Vector;
use crate::particle;
use crate::particle::collision_response;
use crate::particle::Particle;
use crate::particle::ParticleInternal;
pub fn neighbours<'a>(position: &'a Vector, grid: &'a Grid) -> [&'a Vec<usize>; 9] {
    let gid = grid_id_position(*position, grid.side);
    [
        &grid.pidxs[grid.gids[gid][0]],
        &grid.pidxs[grid.gids[gid][1]],
        &grid.pidxs[grid.gids[gid][2]],
        &grid.pidxs[grid.gids[gid][3]],
        &grid.pidxs[grid.gids[gid][4]],
        &grid.pidxs[grid.gids[gid][5]],
        &grid.pidxs[grid.gids[gid][6]],
        &grid.pidxs[grid.gids[gid][7]],
        &grid.pidxs[grid.gids[gid][8]],
    ]
}
pub fn compute_collision_responses(
    diameter: f32,
    particles: &mut [Particle],
    particles_internal: &mut [ParticleInternal],
    grid: &Grid,
) {
    let crdp = 0.01; // collision response delta (position)
    let crdv = 0.90; // collision response delta (velocity)
    let diameter_sqrd = diameter * diameter;
    unsafe {
        let particles_2 = particles as *mut [Particle];
        let particles_ = particles as *mut [Particle];
        let particles_internal_2 = particles_internal as *mut [ParticleInternal];
        for p1 in particles.iter_mut() {
            if p1.live == 0 {
                continue;
            }
            let p1_ = &mut (*particles_)[p1.idx];
            for ns in neighbours(&p1.p, grid) {
                for pid2 in ns {
                    let p2 = &mut (*particles_2)[*pid2];
                    if p2.live == 0 {
                        error("dead neighbour particle in compute_collision_responses");
                        continue;
                    }
                    if p1.idx < p2.idx {
                        let wa = wrap_around(p1.p, p2.p);
                        if wa.d_sqrd < diameter_sqrd {
                            let pi1 = &mut particles_internal[p1.idx];
                            let pi2 = &mut (*particles_internal_2)[p2.idx];
                            alchemy_transfer(p1_, p2, pi1, pi2);
                            if particle::do_collision(p1) && particle::do_collision(p2) {
                                let cr = collision_response(&wa, p1, p2);
                                if !cr.x.is_nan() && !cr.y.is_nan() {
                                    pi1.dv.x += cr.x * crdv;
                                    pi1.dv.y += cr.y * crdv;
                                    pi2.dv.x -= cr.x * crdv;
                                    pi2.dv.y -= cr.y * crdv;
                                    pi1.dp.x -= wa.d.x * crdp;
                                    pi1.dp.y -= wa.d.y * crdp;
                                    pi2.dp.x += wa.d.x * crdp;
                                    pi2.dp.y += wa.d.y * crdp;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
pub fn compute_link_responses(
    diameter: f32,
    particles: &mut [Particle],
    particles_internal: &mut [ParticleInternal],
    links: &mut [Link],
    links_js: &mut [LinkJS],
) {
    let link_strengh = 0.01;
    let link_length_ratio = 1.01;
    let diam_sqrd = diameter * diameter;
    unsafe {
        let particles_internal_2 = particles_internal as *mut [ParticleInternal];
        let particles_2 = particles as *mut [Particle];
        for (i, l) in links.iter().enumerate() {
            let p1 = &mut particles[l.a];
            let p2 = &mut (*particles_2)[l.b];
            if p1.live == 0 || p2.live == 0 {
                error("live link with dead particle");
                continue;
            }
            let pi1 = &mut particles_internal[p1.idx];
            let pi2 = &mut (*particles_internal_2)[p2.idx];
            alchemy_transfer(p1, p2, pi1, pi2);
            if p1.k.is_static() && p2.k.is_static() {
                continue;
            }
            let wa = wrap_around(p1.p, p2.p);
            links_js[i].p = p1.p + wa.d / 2.0;
            let d = wa.d_sqrd.sqrt();
            let factor = (diameter * link_length_ratio - d) * link_strengh;
            let n = normalize(wa.d, d);
            if wa.d_sqrd > diam_sqrd && !n.x.is_nan() && !n.y.is_nan() {
                pi1.dv.x -= n.x * factor;
                pi1.dv.y -= n.y * factor;
                pi2.dv.x += n.x * factor;
                pi2.dv.y += n.y * factor;
                pi1.direction.x -= wa.d.x;
                pi1.direction.y -= wa.d.y;
                pi2.direction.x += wa.d.x;
                pi2.direction.y += wa.d.y;
            }
        }
    }
}
impl Gravithrust {
    pub fn add_particle_internal(&mut self, p: Vector, k: Kind, sid: Option<usize>) -> usize {
        self.add_particle_internal_2(p, Vector::default(), k, sid)
    }
}
