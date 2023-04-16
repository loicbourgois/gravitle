use crate::gravithrust::Gravithrust;
use crate::job::Action;
use crate::job::Condition;
use crate::kind::Kind;
use crate::log;
use crate::math::wrap_around;
use crate::particle::Particle;
use crate::ship::ShipMore;
pub fn plasma_volume_soft_capa(particles: &[Particle], ship_more: &ShipMore) -> (u32, u32) {
    let mut volume = 0;
    let mut capacity = 0;
    for pid in &ship_more.pids {
        let p = &particles[*pid];
        match p.k {
            Kind::PlasmaCargo | Kind::PlasmaCollector | Kind::PlasmaDepot => {
                volume += p.volume;
                capacity += p.k.soft_capacity();
            }
            _ => {}
        }
    }
    (volume, capacity)
}
fn all_conditions_ok(
    particles: &[Particle],
    ship_more: &ShipMore,
    conditions: &[Condition],
) -> bool {
    for condition in conditions {
        match condition {
            Condition::PlasmaStorageNotFull => {
                let (volume, capacity) = plasma_volume_soft_capa(particles, ship_more);
                if volume >= capacity {
                    return false;
                }
            }
            Condition::PlasmaStorageFull => {
                let (volume, capacity) = plasma_volume_soft_capa(particles, ship_more);
                if volume < capacity {
                    return false;
                }
            }
        }
    }
    true
}
impl Gravithrust {
    pub fn check_job(&mut self, sid: usize) {
        let mut ship_more = &mut self.ships_more[sid];
        let ship = &self.ships[sid];
        match &ship_more.job {
            Some(job) => {
                for task in &job.tasks {
                    if !all_conditions_ok(&self.particles, ship_more, &task.conditions) {
                        continue;
                    }
                    match task.action {
                        Action::CollectPlasmaElectroField => match ship_more.target_pid {
                            None => {
                                let mut dmin = 100.0;
                                let mut target_pid = None;
                                for p in &self.particles {
                                    if p.k == Kind::PlasmaElectroField {
                                        let wa = wrap_around(p.p, ship.p);
                                        if wa.d_sqrd < dmin {
                                            dmin = wa.d_sqrd;
                                            target_pid = Some(p.idx);
                                        }
                                    }
                                }
                                ship_more.target_pid = target_pid;
                                match target_pid {
                                    Some(pid) => {
                                        let p = &self.particles[pid];
                                        log(&format!("s#{} -> p#{}:{:?}", sid, p.idx, p.k));
                                    }
                                    None => {}
                                }
                            }
                            Some(target_pid) => {
                                if self.particles[target_pid].k != Kind::PlasmaElectroField {
                                    ship_more.target_pid = None;
                                }
                            }
                        },
                        Action::DeliverPlasma => match ship_more.target_pid {
                            None => {
                                for p in &self.particles {
                                    if p.k == Kind::PlasmaDepot {
                                        ship_more.target_pid = Some(p.idx);
                                        log(&format!("s#{} -> p#{}:{:?}", sid, p.idx, p.k));
                                        break;
                                    }
                                }
                            }
                            Some(target_pid) => {
                                if self.particles[target_pid].k != Kind::PlasmaDepot {
                                    ship_more.target_pid = None;
                                }
                            }
                        },
                        Action::ResetTarget => {
                            ship_more.target_pid = None;
                        }
                    }
                    break;
                }
            }
            None => {}
        }
    }
}
