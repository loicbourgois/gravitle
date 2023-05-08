use crate::action::collect;
use crate::action::deliver;
use crate::gravithrust::Gravithrust;
use crate::job::Action;
use crate::job::Condition;
use crate::kind::Kind;
use crate::particle::Particle;
use crate::ship::ShipMore;
pub fn quantity_soft_capa(
    particles: &[Particle],
    ship_more: &ShipMore,
    kinds: &[Kind],
) -> (u32, u32) {
    let mut quantity = 0;
    let mut capacity = 0;
    for pid in &ship_more.pids {
        let p = &particles[*pid];
        if kinds.contains(&p.k) {
            quantity += p.quantity;
            capacity += p.k.soft_capacity();
        }
    }
    (quantity, capacity)
}
fn all_conditions_ok(
    particles: &[Particle],
    ship_more: &ShipMore,
    conditions: &[Condition],
) -> bool {
    for condition in conditions {
        let r = match condition {
            Condition::PlasmaStorageNotFull => {
                let (quantity, capacity) = quantity_soft_capa(
                    particles,
                    ship_more,
                    &[
                        Kind::PlasmaCargo,
                        Kind::PlasmaRawCollector,
                        Kind::PlasmaRawDepot,
                        Kind::PlasmaElectroFieldCollector,
                    ],
                );
                quantity < capacity
            }
            Condition::PlasmaStorageFull => {
                let (quantity, capacity) = quantity_soft_capa(
                    particles,
                    ship_more,
                    &[
                        Kind::PlasmaCargo,
                        Kind::PlasmaRawCollector,
                        Kind::PlasmaRawDepot,
                        Kind::PlasmaElectroFieldCollector,
                    ],
                );
                quantity >= capacity
            }
        };
        if !r {
            return r;
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
                        Action::CollectPlasmaElectroField => {
                            collect(
                                ship,
                                ship_more,
                                &mut self.particles,
                                Kind::PlasmaElectroField,
                            );
                        }
                        Action::DeliverPlasmaDepot => {
                            deliver(ship_more, &mut self.particles, Kind::PlasmaRawDepot);
                        }
                        Action::CollectPlasmaDepot => {
                            collect(ship, ship_more, &mut self.particles, Kind::PlasmaRawDepot);
                        }
                        Action::DeliverPlasmaRefineryIn => {
                            deliver(ship_more, &mut self.particles, Kind::PlasmaRefineryInput);
                        }
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
