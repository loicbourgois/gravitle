use crate::kind::Kind;
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
    match (p1.k, p2.k, &pi1.new_state, &pi2.new_state) {
        (Kind::Core, Kind::Booster, None, None) => {
            if p1.volume > 0 && p2.volume < p2.k.capacity() {
                pi1.new_state = Some(State {
                    volume: p1.volume - 1,
                    kind: Kind::Core,
                    live: 1,
                });
                pi2.new_state = Some(State {
                    volume: p2.volume + 1,
                    kind: Kind::Booster,
                    live: 1,
                });
            }
        }
        (Kind::Core, Kind::Ray, None, None) => {
            if p1.volume > 0 && p2.volume < p2.k.capacity() {
                pi1.new_state = Some(State {
                    volume: p1.volume - 1,
                    kind: Kind::Core,
                    live: 1,
                });
                pi2.new_state = Some(State {
                    volume: p2.volume + 1,
                    kind: Kind::Ray,
                    live: 1,
                });
            }
        }
        (Kind::Sun, Kind::ElectroField, None, None) => {
            if p2.volume < p2.k.capacity() {
                pi2.new_state = Some(State {
                    volume: p2.volume + 1,
                    kind: Kind::PlasmaElectroField,
                    live: 1,
                });
            }
        }
        (Kind::PlasmaElectroField, Kind::PlasmaCollector, None, None) => {
            if p1.volume > 0 && p2.volume < p2.k.capacity() {
                pi1.new_state = Some(State {
                    volume: 0,
                    kind: Kind::Default,
                    live: 0,
                });
                pi2.new_state = Some(State {
                    volume: p2.volume + 1,
                    kind: Kind::PlasmaCollector,
                    live: 1,
                });
            }
        }
        (Kind::PlasmaCollector, Kind::PlasmaDepot, None, None) => {
            if p1.volume > 0 && p2.volume < p2.k.capacity() {
                pi1.new_state = Some(State {
                    volume: p1.volume - 1,
                    kind: Kind::PlasmaCollector,
                    live: 1,
                });
                pi2.new_state = Some(State {
                    volume: p2.volume + 1,
                    kind: Kind::PlasmaDepot,
                    live: 1,
                });
            }
        }
        _ => {}
    }
}
