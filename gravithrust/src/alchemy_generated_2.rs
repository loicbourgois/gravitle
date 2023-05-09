use crate::alchemy::harvest;
use crate::alchemy::transfer_and_delete;
use crate::alchemy::transfer_from_to;
use crate::kind::Kind;
use crate::particle::Particle;
use crate::particle::ParticleInternal;
pub fn process_alchemy(
    p1: &Particle,
    p2: &Particle,
    pi1: &mut ParticleInternal,
    pi2: &mut ParticleInternal,
) {
    match (p1.k, p1.a, p2.k, p2.a) {
            // Energy transfer from core to
            (Kind::Core, _, Kind::Booster, _) => {
                transfer_from_to(p1, p2, pi1, pi2);
            }
            (Kind::Core, _, Kind::ElectroFieldLauncher, _) => {
                transfer_from_to(p1, p2, pi1, pi2);
            }
            // Harvest sun into plasma_electro_field using electro_field
            (Kind::Sun, _, Kind::ElectroField, _) => {
                harvest(p1, p2, pi1, pi2, Kind::PlasmaElectroField);
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
