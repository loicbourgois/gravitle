// use crate::alchemy::harvest;
// use crate::alchemy::transfer_and_delete;
use crate::alchemy::transfer_from_to;
use crate::kind::Kind;
use crate::particle::ParticleInternal;
use crate::particle::QuantityKind;
use crate::particle::State;
use crate::particle::Particle;
pub fn process_alchemy_transfer(
    p1: &mut Particle,
    p2: &mut Particle,
    pi1: &mut ParticleInternal,
    pi2: &mut ParticleInternal,
    qk: QuantityKind,
) {
    match (p1.k, p2.k, qk) {
        //__ALCHEMY_TRANSFER__//
        _ => {}
    }
}
pub fn alchemy_transform(p1: &mut Particle, pi1: &mut ParticleInternal) {
    if p1.live == 0 {
        return;
    }
    if pi1.new_state.is_some() {
        return;
    }
    match p1.k {
        //__ALCHEMY_TRANSFORM__//
        _ => {}
    }
}
impl Particle {
    pub fn qks(&self) -> &[QuantityKind] {
        match &self.k {
            //__QKS__//
        }
    }
}
