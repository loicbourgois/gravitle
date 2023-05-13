use crate::kind::Kind;
use crate::log;
use crate::math::wrap_around;
use crate::particle::Particle;
use crate::particle::QuantityKind;
use crate::ship::Ship;
use crate::ship::ShipMore;
use rand::seq::SliceRandom;
use rand::thread_rng;
pub fn deliver_closest(
    s: &Ship,
    sm: &mut ShipMore,
    particles: &mut [Particle],
    kind: Kind,
    qk: QuantityKind,
) {
    match sm.target_pid {
        None => {
            let mut dmin = std::f32::INFINITY;
            let mut target_pid: Option<usize> = None;
            for p in particles.iter() {
                if p.k == kind && p.remaining_capacity(qk) > 0 {
                    let wa = wrap_around(p.p, s.p);
                    if wa.d_sqrd < dmin {
                        dmin = wa.d_sqrd;
                        target_pid = Some(p.idx);
                    }
                }
            }
            sm.target_pid = target_pid;
            match target_pid {
                Some(pid) => {
                    let p = &particles[pid];
                    log(&format!(
                        "s#{} -> p#{}:{:?} [{} {}]",
                        sm.sid, p.idx, p.k, p.p.x, p.p.y
                    ));
                }
                None => {}
            }
        }
        Some(target_pid) => {
            if particles[target_pid].k != kind {
                sm.target_pid = None;
            }
        }
    }
}
pub fn deliver_less_quantity(
    s: &Ship,
    sm: &mut ShipMore,
    particles: &mut [Particle],
    kind: Kind,
    qk: QuantityKind,
) {
    match sm.target_pid {
        None => {
            let mut min = std::u32::MAX;
            let mut target_pids = Vec::new();
            for p in particles.iter() {
                if p.k == kind && p.remaining_capacity(qk) > 0 {
                    let x = p.quantity(qk);
                    if x == min {
                        target_pids.push(p.idx)
                    } else if x < min {
                        target_pids.clear();
                        target_pids.push(p.idx);
                        min = x;
                    }
                }
            }
            sm.target_pid = target_pids.choose(&mut thread_rng()).copied();
            match sm.target_pid {
                Some(pid) => {
                    let p = &particles[pid];
                    log(&format!(
                        "s#{} -> p#{}:{:?} [{} {}]",
                        sm.sid, p.idx, p.k, p.p.x, p.p.y
                    ));
                }
                None => {}
            }
        }
        Some(target_pid) => {
            if particles[target_pid].k != kind {
                sm.target_pid = None;
            }
        }
    }
}
