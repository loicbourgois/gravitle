use crate::kind::Kind;
use crate::log;
use crate::particle::Particle;
use crate::ship::ShipMore;
pub fn deliver(sm: &mut ShipMore, particles: &mut [Particle], kind: Kind) {
    match sm.target_pid {
        None => {
            let mut target_pid = None;
            for p in particles.iter() {
                if p.k == kind {
                    target_pid = Some(p.idx);
                    sm.target_pid = target_pid;
                    log(&format!("s#{} -> p#{}:{:?}", sm.sid, p.idx, p.k));
                    break;
                }
            }
            match target_pid {
                Some(_) => {
                    for pid_2 in &sm.pids {
                        let p2 = &mut particles[*pid_2];
                        match (kind, p2.k) {
                            (
                                Kind::PlasmaElectroField
                                | Kind::PlasmaDepot
                                | Kind::PlasmaRefineryInput,
                                Kind::PlasmaCollector,
                            ) => {
                                p2.a = 0;
                            }
                            _ => {}
                        }
                    }
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
