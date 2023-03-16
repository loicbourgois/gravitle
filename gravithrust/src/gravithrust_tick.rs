use crate::collision_response;
// use crate::cross;
// use crate::gravithrust::Gravithrust;
use crate::kind::Kind;
use crate::normalize;
use crate::normalize_2;
use crate::particle;
// use crate::ship_orientation;
// use crate::ship_position;
use crate::wrap_around;
use crate::Delta;
use crate::Link;
use crate::Particle;

pub fn compute_collision_responses(
    diameter: f32,
    particles: &mut Vec<Particle>,
    deltas: &mut Vec<Delta>,
) {
    let crdp = 0.01; // collision response delta (position)
    let crdv = 0.90; // collision response delta (velocity)
    let diameter_sqrd = diameter * diameter;
    for (i1, p1) in particles.iter().enumerate() {
        for (i2, p2) in particles.iter().enumerate() {
            if i1 < i2 {
                let wa = wrap_around(p1.p, p2.p);
                if wa.d_sqrd < diameter_sqrd {
                    let cr = collision_response(&wa, p1, p2);
                    if !cr.x.is_nan() && !cr.y.is_nan() {
                        {
                            let d1 = &mut deltas[i1];
                            d1.v.x += cr.x * crdv;
                            d1.v.y += cr.y * crdv;
                            d1.p.x -= wa.d.x * crdp;
                            d1.p.y -= wa.d.y * crdp;
                        }
                        {
                            let d2 = &mut deltas[i2];
                            d2.v.x -= cr.x * crdv;
                            d2.v.y -= cr.y * crdv;
                            d2.p.x += wa.d.x * crdp;
                            d2.p.y += wa.d.y * crdp;
                        }
                    }
                }
            }
        }
    }
}

pub fn compute_link_responses(
    diameter: f32,
    particles: &mut Vec<Particle>,
    deltas: &mut Vec<Delta>,
    links: &mut Vec<Link>,
) {
    let link_strengh = 0.001;
    let linkt_length_ratio = 1.01;
    for (_i, l) in links.iter().enumerate() {
        let p1 = &particles[l.a];
        let p2 = &particles[l.b];
        let wa = wrap_around(p1.p, p2.p);
        let d = wa.d_sqrd.sqrt();
        let factor = (diameter * linkt_length_ratio - d) * link_strengh;
        let n = normalize(wa.d, d);
        if wa.d_sqrd > diameter * diameter && !n.x.is_nan() && !n.y.is_nan() {
            {
                let d1 = &mut deltas[l.a];
                d1.v.x -= n.x * factor;
                d1.v.y -= n.y * factor;
                d1.direction.x -= wa.d.x;
                d1.direction.y -= wa.d.y;
            }
            {
                let d2 = &mut deltas[l.b];
                d2.v.x += n.x * factor;
                d2.v.y += n.y * factor;
                d2.direction.x += wa.d.x;
                d2.direction.y += wa.d.y;
            }
        }
    }
}

pub fn update_particles(diameter: f32, particles: &mut Vec<Particle>, deltas: &mut Vec<Delta>) {
    let booster_acceleration = 0.00005;
    for (i1, p1) in particles.iter_mut().enumerate() {
        let mut d1 = &mut deltas[i1];
        p1.direction = normalize_2(d1.direction);
        if !particle::is_static(p1) {
            p1.v.x += d1.v.x;
            p1.v.y += d1.v.y;
            p1.p.x += d1.p.x;
            p1.p.y += d1.p.y;
        }
        if p1.k == Kind::Booster && p1.a == 1 {
            p1.v.x -= d1.direction.x * booster_acceleration;
            p1.v.y -= d1.direction.y * booster_acceleration;
        }
        p1.a = 0;
        d1.p.x = 0.0;
        d1.p.y = 0.0;
        d1.v.x = 0.0;
        d1.v.y = 0.0;
        d1.direction.x = 0.0;
        d1.direction.y = 0.0;
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
