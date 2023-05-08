use crate::kind::Kind;
// use crate::log;
use crate::particle::Particle;
use crate::particle::ParticleInternal;
use crate::particle::State;
pub fn transfer_from_to(
    p1: &Particle,
    p2: &Particle,
    pi1: &mut ParticleInternal,
    pi2: &mut ParticleInternal,
) {
    if p2.quantity < p2.k.capacity() && p1.quantity > 0 {
        pi1.new_state = Some(State {
            quantity: p1.quantity - 1,
            kind: p1.k,
            live: p1.live,
        });
        pi2.new_state = Some(State {
            quantity: p2.quantity + 1,
            kind: p2.k,
            live: p2.live,
        });
    }
}
pub fn transfer_and_delete(
    p1: &Particle,
    p2: &Particle,
    pi1: &mut ParticleInternal,
    pi2: &mut ParticleInternal,
) {
    if p2.quantity < p2.k.capacity() && p1.quantity > 0 {
        pi1.new_state = Some(State {
            quantity: 0,
            kind: Kind::Default,
            live: 0,
        });
        pi2.new_state = Some(State {
            quantity: p2.quantity + 1,
            kind: p2.k,
            live: 1,
        });
    }
}
pub fn alchemy(
    p1: &Particle,
    p2: &Particle,
    pi1: &mut ParticleInternal,
    pi2: &mut ParticleInternal,
) {
    if p1.live == 0 || p2.live == 0 {
        return;
    }
    if pi1.new_state.is_some() || pi2.new_state.is_some() {
        return;
    }
    match (p1.k, p1.a, p2.k, p2.a) {
        // Energy transfer from core to booster|ray
        (Kind::Core, _, Kind::Booster | Kind::ElectroFieldLauncher, _) => {
            transfer_from_to(p1, p2, pi1, pi2);
        }
        // Harvest sun
        (Kind::Sun, _, Kind::ElectroField, _) => {
            if p2.quantity < p2.k.capacity() {
                pi2.new_state = Some(State {
                    quantity: p2.quantity + 1,
                    kind: Kind::PlasmaElectroField,
                    live: p2.live,
                });
            }
        }
        // Collect electrofield plasma
        (Kind::PlasmaElectroField, _, Kind::PlasmaElectroFieldCollector, 1) => {
            transfer_and_delete(p1, p2, pi1, pi2);
        }
        // Collect raw plasma
        (Kind::PlasmaRawDepot, _, Kind::PlasmaRawCollector, 1) => {
            transfer_from_to(p1, p2, pi1, pi2);
        }
        // Drop plasma
        (Kind::PlasmaElectroFieldCollector, 0, Kind::PlasmaRawDepot, _) => {
            transfer_from_to(p1, p2, pi1, pi2);
        }
        // Drop
        (Kind::PlasmaRawCollector, 0, Kind::PlasmaRefineryInput, _) => {
            transfer_from_to(p1, p2, pi1, pi2);
        }
        _ => {}
    }
}
