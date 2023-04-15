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
    match (p1.k, p2.k, &pi1.new_state, &pi2.new_state) {
        (Kind::Sun, Kind::ElectroField, None, None) => {
            if p2.volume < p2.k.capacity() {
                pi2.new_state = Some(State {
                    volume: p2.volume + 1,
                    kind: Kind::ElectroFieldPlasma,
                });
            }
        }
        (Kind::ElectroFieldPlasma, Kind::PlasmaCollector, None, None) => {
            if p1.volume > 0 && p2.volume < p2.k.capacity() {
                pi1.new_state = Some(State {
                    volume: p1.volume - 1,
                    kind: Kind::ElectroField,
                });
                pi2.new_state = Some(State {
                    volume: p2.volume + 1,
                    kind: Kind::PlasmaCollector,
                });
            }
        }
        (Kind::PlasmaCollector, Kind::PlasmaDepot, None, None) => {
            if p1.volume > 0 && p2.volume < p2.k.capacity() {
                pi1.new_state = Some(State {
                    volume: p1.volume - 1,
                    kind: Kind::PlasmaCollector,
                });
                pi2.new_state = Some(State {
                    volume: p2.volume + 1,
                    kind: Kind::PlasmaDepot,
                });
            }
        }
        _ => {}
    }
}
