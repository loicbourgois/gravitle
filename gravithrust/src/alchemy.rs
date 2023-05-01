use crate::kind::Kind;
use crate::log;
use crate::particle::Particle;
use crate::particle::ParticleInternal;
use crate::particle::State;
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
    match (p1.k, p1.a, p2.k) {
        // Energy transfer from core to booster|ray
        (Kind::Booster | Kind::Ray, _, Kind::Core) => {
            if p1.quantity < p1.k.capacity() && p2.quantity > 0 {
                pi1.new_state = Some(State {
                    quantity: p1.quantity + 1,
                    kind: p1.k,
                    live: p1.live,
                });
                pi2.new_state = Some(State {
                    quantity: p2.quantity - 1,
                    kind: p2.k,
                    live: p2.live,
                });
            }
        }
        // Harvest sun
        (Kind::Sun, _, Kind::ElectroField) => {
            if p2.quantity < p2.k.capacity() {
                pi2.new_state = Some(State {
                    quantity: p2.quantity + 1,
                    kind: Kind::PlasmaElectroField,
                    live: p2.live,
                });
            }
        }
        // Collect plasma
        (Kind::PlasmaCollector, 1, Kind::PlasmaElectroField) => {
            if p1.quantity < p1.k.capacity() && p2.quantity > 0 {
                pi1.new_state = Some(State {
                    quantity: p1.quantity + 1,
                    kind: Kind::PlasmaCollector,
                    live: 1,
                });
                pi2.new_state = Some(State {
                    quantity: 0,
                    kind: Kind::Default,
                    live: 0,
                });
            }
        }
        (Kind::PlasmaCollector, 1, Kind::PlasmaDepot) => {
            if p1.quantity < p1.k.capacity() && p2.quantity > 0 {
                log("aa");
                pi1.new_state = Some(State {
                    quantity: p1.quantity + 1,
                    kind: p1.k,
                    live: p1.live,
                });
                pi2.new_state = Some(State {
                    quantity: p2.quantity - 1,
                    kind: p2.k,
                    live: p2.live,
                });
            }
        }
        // Drop plasma
        (Kind::PlasmaCollector, 0, Kind::PlasmaDepot | Kind::PlasmaRefineryInput) => {
            if p1.quantity > 0 && p2.quantity < p2.k.capacity() {
                pi1.new_state = Some(State {
                    quantity: p1.quantity - 1,
                    kind: Kind::PlasmaCollector,
                    live: 1,
                });
                pi2.new_state = Some(State {
                    quantity: p2.quantity + 1,
                    kind: p2.k,
                    live: 1,
                });
            }
        }
        _ => {}
    }
}
