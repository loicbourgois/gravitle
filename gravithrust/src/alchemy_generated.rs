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
        // transfer energy core booster
        (Kind::Core, _, Kind::Booster, _) => {
            transfer_from_to(p1, p2, pi1, pi2);
        }
        // transfer energy core electro_field_launcher
        (Kind::Core, _, Kind::ElectroFieldLauncher, _) => {
            transfer_from_to(p1, p2, pi1, pi2);
        }
        // transfer energy core heat_launcher
        (Kind::Core, _, Kind::HeatLauncher, _) => {
            transfer_from_to(p1, p2, pi1, pi2);
        }
        // harvest matter sun electro_field plasma_electro_field
        (Kind::Sun, _, Kind::ElectroField, _) => {
            harvest(p1, p2, pi1, pi2, Kind::PlasmaElectroField);
        }
        // harvest matter ice heat water
        (Kind::Ice, _, Kind::Heat, _) => {
            harvest(p1, p2, pi1, pi2, Kind::Water);
        }
        // transfer_and_delete matter plasma_electro_field plasma_electro_field_collector
        (Kind::PlasmaElectroField, _, Kind::PlasmaElectroFieldCollector, _) => {
            transfer_and_delete(p1, p2, pi1, pi2);
        }
        // collect matter plasma_raw_depot plasma_raw_collector
        (Kind::PlasmaRawDepot, _, Kind::PlasmaRawCollector, _) => {
            transfer_from_to(p1, p2, pi1, pi2);
        }
        // drop matter plasma_electro_field_collector plasma_raw_depot
        (Kind::PlasmaElectroFieldCollector, _, Kind::PlasmaRawDepot, _) => {
            transfer_from_to(p1, p2, pi1, pi2);
        }
        // drop matter plasma_raw_collector plasma_refinery_input
        (Kind::PlasmaRawCollector, _, Kind::PlasmaRefineryInput, _) => {
            transfer_from_to(p1, p2, pi1, pi2);
        }

        // transfer water_droplet water luciole
        (Kind::Water, _, Kind::Luciole, _) => {
            transfer_from_to(p1, p2, pi1, pi2);
        }
        // transfer iron_ore iron_asteroid iron_ore_collector
        (Kind::IronAsteroid, _, Kind::IronOreCollector, _) => {
            transfer_from_to(p1, p2, pi1, pi2);
        }
        // transfer iron_ore iron_ore_collector iron_ore_cargo
        (Kind::IronOreCollector, _, Kind::IronOreCargo, _) => {
            transfer_from_to(p1, p2, pi1, pi2);
        }
        // transfer iron_ore iron_ore_cargo iron_ore_collector
        (Kind::IronOreCargo, _, Kind::IronOreCollector, _) => {
            transfer_from_to(p1, p2, pi1, pi2);
        }
        // transfer coal coal_asteroid coal_collector
        (Kind::CoalAsteroid, _, Kind::CoalCollector, _) => {
            transfer_from_to(p1, p2, pi1, pi2);
        }
        // transfer coal coal_collector coal_cargo
        (Kind::CoalCollector, _, Kind::CoalCargo, _) => {
            transfer_from_to(p1, p2, pi1, pi2);
        }
        // transfer coal coal_cargo coal_collector
        (Kind::CoalCargo, _, Kind::CoalCollector, _) => {
            transfer_from_to(p1, p2, pi1, pi2);
        }
        // transfer iron_ore iron_ore_collector iron_furnace
        (Kind::IronOreCollector, _, Kind::IronFurnace, _) => {
            transfer_from_to(p1, p2, pi1, pi2);
        }
        // transfer coal coal_collector iron_furnace
        (Kind::CoalCollector, _, Kind::IronFurnace, _) => {
            transfer_from_to(p1, p2, pi1, pi2);
        }
        // transfer iron iron_furnace iron_collector
        (Kind::IronFurnace, _, Kind::IronCollector, _) => {
            transfer_from_to(p1, p2, pi1, pi2);
        }
        // transfer iron_gangue iron_furnace iron_gangue_collector
        (Kind::IronFurnace, _, Kind::IronGangueCollector, _) => {
            transfer_from_to(p1, p2, pi1, pi2);
        }
        // transfer heat iron_furnace generator
        (Kind::IronFurnace, _, Kind::Generator, _) => {
            transfer_from_to(p1, p2, pi1, pi2);
        }
        // transfer energy generator battery
        (Kind::Generator, _, Kind::Battery, _) => {
            transfer_from_to(p1, p2, pi1, pi2);
        }
        // transfer ice ice_asteroid ice_collector
        (Kind::IceAsteroid, _, Kind::IceCollector, _) => {
            transfer_from_to(p1, p2, pi1, pi2);
        }
        // transfer ice ice_collector ice_cargo
        (Kind::IceCollector, _, Kind::IceCargo, _) => {
            transfer_from_to(p1, p2, pi1, pi2);
        }
        // transfer ice ice_cargo ice_collector
        (Kind::IceCargo, _, Kind::IceCollector, _) => {
            transfer_from_to(p1, p2, pi1, pi2);
        }
        // transfer ice ice_collector ice_melter
        (Kind::IceCollector, _, Kind::IceMelter, _) => {
            transfer_from_to(p1, p2, pi1, pi2);
        }
        // transfer water ice_melter generator
        (Kind::IceMelter, _, Kind::Generator, _) => {
            transfer_from_to(p1, p2, pi1, pi2);
        }
        _ => {}
    }
}
