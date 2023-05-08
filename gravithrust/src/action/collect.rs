use crate::kind::Kind;
use crate::log;
use crate::math::wrap_around;
use crate::particle::Particle;
use crate::ship::Ship;
use crate::ship::ShipMore;
pub fn collect(s: &Ship, sm: &mut ShipMore, particles: &mut [Particle], kind: Kind) {
    match sm.target_pid {
        None => {
            let mut dmin = std::f32::INFINITY;
            let mut target_pid: Option<usize> = None;
            for p in particles.iter() {
                if p.k == kind && p.quantity > 0 {
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
                    for pid_2 in &sm.pids {
                        let p2 = &mut particles[*pid_2];
                        match (kind, p2.k) {
                            (Kind::PlasmaElectroField, Kind::PlasmaElectroFieldCollector) => {
                                p2.a = 1;
                            }
                            (Kind::PlasmaRawDepot, Kind::PlasmaRawCollector) => {
                                p2.a = 1;
                            }
                            _ => {}
                        }
                    }
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
