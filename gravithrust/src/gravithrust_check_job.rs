use crate::action::collect;
use crate::action::deliver;
use crate::gravithrust::Gravithrust;
use crate::job::Action;
use crate::job::Condition;
use crate::kind::Kind;
use crate::particle::Particle;
use crate::particle::QuantityKind;
use crate::ship::ShipMore;
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
            _ => false,
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
                            deliver(
                                ship,
                                ship_more,
                                &mut self.particles,
                                Kind::CoalDepot,
                                QuantityKind::Coal,
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
                            deliver(
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
                        // Action::LaunchElectroField => self.launch_electro_field(sid),
                        _ => {}
                    }
                    break;
                }
            }
            None => {}
        }
    }
    // pub fn launch_electro_field(&mut self, sid: usize) {
    //     let ship_more = &self.ships_more[sid];
    //     let ship = &self.ships[sid];
    //     // match (ship_more.anchor_pid, ship_more.target_pid) {
    //     //     (Some(anchor_pid), Some(target_pid)) => {
    //     //         let anchor = self.particles[anchor_pid].p;
    //     //         let target = self.particles[target_pid].p;
    //     //         let uu = normalize_2(wrap_around(ship.p, target).d);
    //     //         let anchor_delta = wrap_around(ship.p, anchor);
    //     //         let ray_particle = &mut self.particles[ship_more.pids[0]];
    //     //         if anchor_delta.d_sqrd < 0.001
    //     //             && ray_particle.q1 >= ray_particle.k.capacity()
    //     //             && angle(uu, normalize_2(ray_particle.direction)).abs() < 0.01
    //     //         {
    //     //             ray_particle.q1 = 0;
    //     //             let aa = ray_particle.p + ray_particle.direction * self.diameter * 1.75;
    //     //             self.add_particle_internal_2(
    //     //                 aa,
    //     //                 uu * self.diameter * 0.01,
    //     //                 Kind::ElectroField,
    //     //                 None,
    //     //             );
    //     //         }
    //     //     }
    //     //     _ => {}
    //     // }
    // }
}
