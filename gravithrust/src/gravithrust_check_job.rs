use crate::action::collect;
use crate::action::deliver_closest;
use crate::action::deliver_less_quantity;
use crate::gravithrust::Gravithrust;
use crate::job::Action;
use crate::job::Condition;
use crate::kind::Kind;
use crate::particle::Particle;
use crate::particle::QuantityKind;
use crate::ship::ShipMore;
use rand::Rng;
pub fn remaining_capacity(ship_more: &ShipMore, particles: &[Particle], qk: QuantityKind) -> u32 {
    let mut rc = 0;
    for pid in &ship_more.pids {
        let p = &particles[*pid];
        if p.qks().contains(&qk) {
            rc += p.remaining_capacity(qk);
        }
    }
    rc
}
pub fn quantity(ship_more: &ShipMore, particles: &[Particle], qk: QuantityKind) -> u32 {
    let mut q = 0;
    for pid in &ship_more.pids {
        let p = &particles[*pid];
        if p.qks().contains(&qk) {
            q += p.quantity(qk);
        }
    }
    q
}
fn all_conditions_ok(
    particles: &[Particle],
    ship_more: &ShipMore,
    conditions: &[Condition],
) -> bool {
    let mut rng = rand::rng();
    for condition in conditions {
        let r = match condition {
            Condition::CoalStorageEmpty => quantity(ship_more, particles, QuantityKind::Coal) == 0,
            Condition::CoalStorageFull => {
                remaining_capacity(ship_more, particles, QuantityKind::Coal) == 0
            }
            Condition::IronOreStorageEmpty => {
                quantity(ship_more, particles, QuantityKind::IronOre) == 0
            }
            Condition::IronOreStorageFull => {
                remaining_capacity(ship_more, particles, QuantityKind::IronOre) == 0
            }
            Condition::EnergyStorageEmpty => {
                quantity(ship_more, particles, QuantityKind::Energy) == 0
            }
            Condition::EnergyStorageFull => {
                remaining_capacity(ship_more, particles, QuantityKind::Energy) == 0
            }
            Condition::Random1Per1000 => rng.random::<f32>() < 0.001,
            Condition::Random1Per10 => rng.random::<f32>() < 0.1,
            Condition::Random1Per100 => rng.random::<f32>() < 0.01,
        };
        if !r {
            return r;
        }
    }
    true
}
impl Gravithrust {
    pub fn check_job(&mut self, sid: usize) {
        let ship_more = &mut self.ships_more[sid];
        let ship = &self.ships[sid];
        match &ship_more.job {
            Some(job) => {
                for task in &job.tasks {
                    if !all_conditions_ok(&self.particles, ship_more, &task.conditions) {
                        continue;
                    }
                    match task.action {
                        Action::CollectCoal => {
                            collect(
                                ship,
                                ship_more,
                                &mut self.particles,
                                Kind::CoalAsteroid,
                                QuantityKind::Coal,
                            );
                        }
                        Action::DeliverCoal => {
                            deliver_closest(
                                ship,
                                ship_more,
                                &mut self.particles,
                                Kind::CoalDepot,
                                QuantityKind::Coal,
                            );
                        }

                        Action::CollectEnergy => {
                            collect(
                                ship,
                                ship_more,
                                &mut self.particles,
                                Kind::EnergyDepot,
                                QuantityKind::Energy,
                            );
                        }
                        Action::DeliverEnergy => {
                            deliver_less_quantity(
                                ship_more,
                                &mut self.particles,
                                Kind::Battery,
                                QuantityKind::Energy,
                            );
                        }

                        Action::CollectIronOre => {
                            collect(
                                ship,
                                ship_more,
                                &mut self.particles,
                                Kind::IronAsteroid,
                                QuantityKind::IronOre,
                            );
                        }
                        Action::DeliverIronOre => {
                            deliver_closest(
                                ship,
                                ship_more,
                                &mut self.particles,
                                Kind::IronOreDepot,
                                QuantityKind::IronOre,
                            );
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
