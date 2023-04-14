use crate::grid::grid_id_position;
use crate::grid::Grid;
use crate::kind::Kind;
use crate::link::Link;
use crate::link::LinkJS;
use crate::log;
use crate::math::collision_response;
use crate::math::normalize;
use crate::math::normalize_2;
use crate::math::wrap_around;
use crate::math::Vector;
use crate::particle;
use crate::particle::Particle;
use crate::particle::ParticleInternal;
use crate::particle::Particles;
use crate::ship::Ship;
use crate::ship::ShipMore;
use rand::Rng;
struct ParticleModel {
    p: Vector,
    k: Kind,
    sid: Option<usize>,
}
pub fn add_particle(
    particles: &mut Vec<Particle>,
    particles_internal: &mut Vec<ParticleInternal>,
    p: Vector,
    k: Kind,
    sid: Option<usize>,
) -> usize {
    add_particle_2(particles, particles_internal, p, Vector::default(), k, sid)
}
pub fn add_particle_2(
    particles: &mut Vec<Particle>,
    particles_internal: &mut Vec<ParticleInternal>,
    position: Vector,
    velocity: Vector,
    k: Kind,
    sid: Option<usize>,
) -> usize {
    let pid = particles.len();
    particles.push(Particle {
        p: position,
        pp: position - velocity,
        v: velocity,
        m: 1.0,
        k,
        direction: Vector::default(),
        a: 0,
        idx: pid,
        grid_id: 0,
        e: 0,
    });
    particles_internal.push(ParticleInternal {
        dp: Vector::default(),
        dv: Vector::default(),
        direction: Vector::default(),
        sid,
        new_kind: vec![],
    });
    pid
}
pub fn add_particles(
    diameter: f32,
    particles: &mut Vec<Particle>,
    particles_internal: &mut Vec<ParticleInternal>,
) {
    let mut particles_to_add = vec![];
    for p1 in particles.iter() {
        if p1.k == Kind::Sun && rand::thread_rng().gen::<f32>() < -1.0 {
            particles_to_add.push(ParticleModel {
                p: Vector {
                    x: p1.p.x + rand::thread_rng().gen::<f32>() * diameter - diameter * 0.5,
                    y: p1.p.y + rand::thread_rng().gen::<f32>() * diameter - diameter * 0.5,
                },
                k: Kind::Armor,
                sid: None,
            });
        }
    }
    for x in &particles_to_add {
        add_particle(particles, particles_internal, x.p, x.k, x.sid);
    }
}
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
    particles: &mut Vec<Particle>,
    particles_internal: &mut [ParticleInternal],
    grid: &Grid,
    ships: &mut [Ship],
    ships_more: &[ShipMore],
) {
    let crdp = 0.01; // collision response delta (position)
    let crdv = 0.90; // collision response delta (velocity)
    let diameter_sqrd = diameter * diameter;
    unsafe {
        let particles_2 = particles as *mut Particles;
        for p1 in particles.iter_mut() {
            for ns in neighbours(&p1.p, grid) {
                for idx in ns {
                    let p2 = &mut (*particles_2)[*idx];
                    if p1.idx < p2.idx {
                        let wa = wrap_around(p1.p, p2.p);
                        if wa.d_sqrd < diameter_sqrd {
                            match (p1.k, p2.k) {
                                (Kind::Sun, Kind::ElectroField) => particles_internal[p2.idx]
                                    .new_kind
                                    .push(Kind::ElectroFieldPlasma),
                                (Kind::ElectroField, Kind::Sun) => particles_internal[p1.idx]
                                    .new_kind
                                    .push(Kind::ElectroFieldPlasma),
                                (Kind::ElectroFieldPlasma, Kind::PlasmaCollector) => {
                                    particles_internal[p1.idx].new_kind.push(Kind::ElectroField)
                                }
                                (Kind::PlasmaCollector, Kind::ElectroFieldPlasma) => {
                                    particles_internal[p2.idx].new_kind.push(Kind::ElectroField)
                                }
                                _ => {}
                            }
                            match particles_internal[p1.idx].sid {
                                Some(sid) => {
                                    if ships_more[sid].target_pid == Some(p2.idx) {
                                        ships[sid].on_target += 1;
                                    }
                                }
                                None => {}
                            }
                            match particles_internal[p2.idx].sid {
                                Some(sid) => {
                                    if ships_more[sid].target_pid == Some(p1.idx) {
                                        ships[sid].on_target += 1;
                                    }
                                }
                                None => {}
                            }
                            if particle::do_collision(p1) && particle::do_collision(p2) {
                                let cr = collision_response(&wa, p1, p2);
                                if !cr.x.is_nan() && !cr.y.is_nan() {
                                    {
                                        let d1 = &mut particles_internal[p1.idx];
                                        d1.dv.x += cr.x * crdv;
                                        d1.dv.y += cr.y * crdv;
                                        d1.dp.x -= wa.d.x * crdp;
                                        d1.dp.y -= wa.d.y * crdp;
                                    }
                                    {
                                        let d2 = &mut particles_internal[p2.idx];
                                        d2.dv.x -= cr.x * crdv;
                                        d2.dv.y -= cr.y * crdv;
                                        d2.dp.x += wa.d.x * crdp;
                                        d2.dp.y += wa.d.y * crdp;
                                    }
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
    for (i, l) in links.iter().enumerate() {
        let p1 = &particles[l.a];
        let p2 = &particles[l.b];
        let wa = wrap_around(p1.p, p2.p);
        links_js[i].p = p1.p + wa.d / 2.0;
        let d = wa.d_sqrd.sqrt();
        let factor = (diameter * link_length_ratio - d) * link_strengh;
        let n = normalize(wa.d, d);
        if wa.d_sqrd > diameter * diameter && !n.x.is_nan() && !n.y.is_nan() {
            {
                let d1 = &mut particles_internal[l.a];
                d1.dv.x -= n.x * factor;
                d1.dv.y -= n.y * factor;
                d1.direction.x -= wa.d.x;
                d1.direction.y -= wa.d.y;
            }
            {
                let d2 = &mut particles_internal[l.b];
                d2.dv.x += n.x * factor;
                d2.dv.y += n.y * factor;
                d2.direction.x += wa.d.x;
                d2.direction.y += wa.d.y;
            }
        }
    }
}
pub fn update_particles(
    diameter: f32,
    particles: &mut [Particle],
    particles_internal: &mut [ParticleInternal],
    booster_acceleration: f32,
) {
    for (pid, p1) in particles.iter_mut().enumerate() {
        let mut d1 = &mut particles_internal[pid];
        p1.direction = normalize_2(d1.direction);
        if !particle::is_static(p1) {
            p1.v.x += d1.dv.x;
            p1.v.y += d1.dv.y;
            p1.p.x += d1.dp.x;
            p1.p.y += d1.dp.y;
        }
        if p1.k == Kind::Booster && p1.a == 1 {
            p1.v.x -= d1.direction.x * booster_acceleration;
            p1.v.y -= d1.direction.y * booster_acceleration;
        }
        if p1.k == Kind::Ray {
            p1.e += 1;
            p1.e = p1.e.min(5_000);
        }
        match d1.new_kind.len() {
            0 => {}
            1 => {
                p1.k = d1.new_kind[0];
            }
            _ => log(&format!(
                "too many new kinds {:?} -> {:?}",
                p1.k, d1.new_kind
            )),
        }
        d1.dp.x = 0.0;
        d1.dp.y = 0.0;
        d1.dv.x = 0.0;
        d1.dv.y = 0.0;
        d1.direction.x = 0.0;
        d1.direction.y = 0.0;
        d1.new_kind.clear();
        p1.a = 0;
        p1.v.x = p1.v.x.max(-diameter * 0.5);
        p1.v.x = p1.v.x.min(diameter * 0.5);
        p1.v.y = p1.v.y.max(-diameter * 0.5);
        p1.v.y = p1.v.y.min(diameter * 0.5);
        p1.p.x = (10.0 + p1.p.x + p1.v.x) % 1.0;
        p1.p.y = (10.0 + p1.p.y + p1.v.y) % 1.0;
        p1.pp.x = p1.p.x - p1.v.x;
        p1.pp.y = p1.p.y - p1.v.y;
    }
}
