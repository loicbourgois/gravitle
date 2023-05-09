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
        // transfer core booster
        (Kind::Core, _, Kind::Booster, _) => {
            transfer_from_to(p1, p2, pi1, pi2);
        }
        // transfer core electro_field_launcher
        (Kind::Core, _, Kind::ElectroFieldLauncher, _) => {
            transfer_from_to(p1, p2, pi1, pi2);
        }
        // transfer core heat_launcher
        (Kind::Core, _, Kind::HeatLauncher, _) => {
            transfer_from_to(p1, p2, pi1, pi2);
        }
        // harvest sun electro_field plasma_electro_field
        (Kind::Sun, _, Kind::ElectroField, _) => {
            harvest(p1, p2, pi1, pi2, Kind::PlasmaElectroField);
        }
        // harvest ice heat water
        (Kind::Ice, _, Kind::Heat, _) => {
            harvest(p1, p2, pi1, pi2, Kind::Water);
        }
        // transfer_and_delete plasma_electro_field plasma_electro_field_collector
        (Kind::PlasmaElectroField, _, Kind::PlasmaElectroFieldCollector, _) => {
            transfer_and_delete(p1, p2, pi1, pi2);
        }
        // collect plasma_raw_depot plasma_raw_collector
        (Kind::PlasmaRawDepot, _, Kind::PlasmaRawCollector, _) => {
            transfer_from_to(p1, p2, pi1, pi2);
        }
        // drop plasma_electro_field_collector plasma_raw_depot
        (Kind::PlasmaElectroFieldCollector, _, Kind::PlasmaRawDepot, _) => {
            transfer_from_to(p1, p2, pi1, pi2);
        }
        // drop plasma_raw_collector plasma_refinery_input
        (Kind::PlasmaRawCollector, _, Kind::PlasmaRefineryInput, _) => {
            transfer_from_to(p1, p2, pi1, pi2);
        }

        _ => {}
    }
}
