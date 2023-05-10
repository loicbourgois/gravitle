use crate::alchemy_generated::process_alchemy;
use crate::kind::Kind;
use crate::particle::Particle;
use crate::particle::ParticleInternal;
use crate::particle::State;
pub fn transfer_from_to(
    p1: &Particle,
    p2: &Particle,
    pi1: &mut ParticleInternal,
    pi2: &mut ParticleInternal,
) {
    if p2.q1 < p2.k.capacity() && p1.q1 > 0 {
        pi1.new_state = Some(State {
            q1: p1.q1 - 1,
            q2: 0,
            kind: p1.k,
            live: p1.live,
        });
        pi2.new_state = Some(State {
            q1: p2.q1 + 1,
            kind: p2.k,
            q2: 0,
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
    if p2.q1 < p2.k.capacity() && p1.q1 > 0 {
        pi1.new_state = Some(State {
            q1: 0,
            kind: Kind::Default,
            live: 0,
            q2: 0,
        });
        pi2.new_state = Some(State {
            q1: p2.q1 + 1,
            kind: p2.k,
            live: 1,
            q2: 0,
        });
    }
}
pub fn harvest(
    _p1: &Particle,
    p2: &Particle,
    _pi1: &mut ParticleInternal,
    pi2: &mut ParticleInternal,
    new_kind: Kind,
) {
    if p2.q1 < p2.k.capacity() {
        pi2.new_state = Some(State {
            q1: p2.q1 + 1,
            kind: new_kind,
            live: p2.live,
            q2: 0,
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
    process_alchemy(p1, p2, pi1, pi2);
}
