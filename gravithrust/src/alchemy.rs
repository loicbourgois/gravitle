use crate::alchemy_generated::process_alchemy_transfer;
use crate::particle::Particle;
use crate::particle::ParticleInternal;
use crate::particle::QuantityKind;
use crate::particle::State;
pub fn transfer_from_to(
    p1: &mut Particle,
    p2: &mut Particle,
    pi1: &mut ParticleInternal,
    pi2: &mut ParticleInternal,
    qk: QuantityKind,
) {
    let rate = (match qk {
        QuantityKind::Energy => 10_000,
        _ => 1,
    })
    .min(p1.quantity(qk))
    .min(p2.remaining_capacity(qk));
    if rate > 0 && pi1.new_state.is_none() && pi2.new_state.is_none() {
        p1.remove_quantity(qk, rate);
        p2.add_quantity(qk, rate);
        pi1.new_state = Some(State {
            live: p1.live,
        });
        pi2.new_state = Some(State {
            live: p1.live,
        });
    }
}
pub fn alchemy_transfer(
    p1: &mut Particle,
    p2: &mut Particle,
    pi1: &mut ParticleInternal,
    pi2: &mut ParticleInternal,
) {
    if p1.live == 0 || p2.live == 0 {
        return;
    }
    if pi1.new_state.is_some() || pi2.new_state.is_some() {
        return;
    }
    unsafe {
        let p1_pointer = p1 as *const Particle;
        let p2_pointer = p2 as *const Particle;
        let qks_1 = (*p1_pointer).qks();
        let qks_2 = (*p2_pointer).qks();
        for qk in qks_1 {
            if qks_2.contains(qk) {
                process_alchemy_transfer(p1, p2, pi1, pi2, *qk);
                process_alchemy_transfer(p2, p1, pi2, pi1, *qk);
            }
        }
    }
}
