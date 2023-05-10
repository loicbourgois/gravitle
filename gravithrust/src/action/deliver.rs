use crate::kind::Kind;
use crate::log;
use crate::math::wrap_around;
use crate::particle::Particle;
use crate::particle::QuantityKind;
use crate::ship::Ship;
use crate::ship::ShipMore;
pub fn deliver(
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
                    log(&format!("s#{} -> p#{}:{:?}", sm.sid, p.idx, p.k));
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
