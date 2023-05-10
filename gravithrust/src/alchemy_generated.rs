// use crate::alchemy::harvest;
// use crate::alchemy::transfer_and_delete;
use crate::alchemy::transfer_from_to;
use crate::kind::Kind;
use crate::particle::Particle;
use crate::particle::ParticleInternal;
use crate::particle::QuantityKind;
use crate::particle::State;
pub fn process_alchemy_transfer(
    p1: &mut Particle,
    p2: &mut Particle,
    pi1: &mut ParticleInternal,
    pi2: &mut ParticleInternal,
    qk: QuantityKind,
) {
    match (p1.k, p2.k, qk) {
        // transfer energy core booster
        (Kind::Core, Kind::Booster, QuantityKind::Energy) => {
            transfer_from_to(p1, p2, pi1, pi2, QuantityKind::Energy);
        }
        // transfer energy core electro_field_launcher
        (Kind::Core, Kind::ElectroFieldLauncher, QuantityKind::Energy) => {
            transfer_from_to(p1, p2, pi1, pi2, QuantityKind::Energy);
        }
        // transfer energy core heat_launcher
        (Kind::Core, Kind::HeatLauncher, QuantityKind::Energy) => {
            transfer_from_to(p1, p2, pi1, pi2, QuantityKind::Energy);
        }
        // transfer water_droplet water luciole
        (Kind::Water, Kind::Luciole, QuantityKind::WaterDroplet) => {
            transfer_from_to(p1, p2, pi1, pi2, QuantityKind::WaterDroplet);
        }
        // transfer iron_ore iron_asteroid iron_ore_collector
        (Kind::IronAsteroid, Kind::IronOreCollector, QuantityKind::IronOre) => {
            transfer_from_to(p1, p2, pi1, pi2, QuantityKind::IronOre);
        }
        // transfer iron_ore iron_ore_collector iron_ore_cargo
        (Kind::IronOreCollector, Kind::IronOreCargo, QuantityKind::IronOre) => {
            transfer_from_to(p1, p2, pi1, pi2, QuantityKind::IronOre);
        }
        // transfer iron_ore iron_ore_cargo iron_ore_collector
        (Kind::IronOreCargo, Kind::IronOreCollector, QuantityKind::IronOre) => {
            transfer_from_to(p1, p2, pi1, pi2, QuantityKind::IronOre);
        }
        // transfer coal coal_asteroid coal_input
        (Kind::CoalAsteroid, Kind::CoalInput, QuantityKind::Coal) => {
            transfer_from_to(p1, p2, pi1, pi2, QuantityKind::Coal);
        }
        // transfer coal coal_input coal_cargo
        (Kind::CoalInput, Kind::CoalCargo, QuantityKind::Coal) => {
            transfer_from_to(p1, p2, pi1, pi2, QuantityKind::Coal);
        }
        // transfer coal coal_cargo coal_output
        (Kind::CoalCargo, Kind::CoalOutput, QuantityKind::Coal) => {
            transfer_from_to(p1, p2, pi1, pi2, QuantityKind::Coal);
        }
        // transfer coal coal_output coal_depot
        (Kind::CoalOutput, Kind::CoalDepot, QuantityKind::Coal) => {
            transfer_from_to(p1, p2, pi1, pi2, QuantityKind::Coal);
        }
        // transfer iron_ore iron_asteroid iron_ore_input
        (Kind::IronAsteroid, Kind::IronOreInput, QuantityKind::IronOre) => {
            transfer_from_to(p1, p2, pi1, pi2, QuantityKind::IronOre);
        }
        // transfer iron_ore iron_ore_input iron_ore_cargo
        (Kind::IronOreInput, Kind::IronOreCargo, QuantityKind::IronOre) => {
            transfer_from_to(p1, p2, pi1, pi2, QuantityKind::IronOre);
        }
        // transfer iron_ore iron_ore_cargo iron_ore_output
        (Kind::IronOreCargo, Kind::IronOreOutput, QuantityKind::IronOre) => {
            transfer_from_to(p1, p2, pi1, pi2, QuantityKind::IronOre);
        }
        // transfer iron_ore iron_ore_output iron_ore_depot
        (Kind::IronOreOutput, Kind::IronOreDepot, QuantityKind::IronOre) => {
            transfer_from_to(p1, p2, pi1, pi2, QuantityKind::IronOre);
        }
        // transfer iron_ore iron_ore_depot iron_furnace
        (Kind::IronOreDepot, Kind::IronFurnace, QuantityKind::IronOre) => {
            transfer_from_to(p1, p2, pi1, pi2, QuantityKind::IronOre);
        }
        // transfer iron_ore iron_ore_collector iron_ore_depot
        (Kind::IronOreCollector, Kind::IronOreDepot, QuantityKind::IronOre) => {
            transfer_from_to(p1, p2, pi1, pi2, QuantityKind::IronOre);
        }
        // transfer coal coal_collector coal_depot
        (Kind::CoalCollector, Kind::CoalDepot, QuantityKind::Coal) => {
            transfer_from_to(p1, p2, pi1, pi2, QuantityKind::Coal);
        }
        // transfer coal coal_depot iron_furnace
        (Kind::CoalDepot, Kind::IronFurnace, QuantityKind::Coal) => {
            transfer_from_to(p1, p2, pi1, pi2, QuantityKind::Coal);
        }
        // transfer iron iron_furnace iron_collector
        (Kind::IronFurnace, Kind::IronCollector, QuantityKind::Iron) => {
            transfer_from_to(p1, p2, pi1, pi2, QuantityKind::Iron);
        }
        // transfer iron_gangue iron_furnace iron_gangue_collector
        (Kind::IronFurnace, Kind::IronGangueCollector, QuantityKind::IronGangue) => {
            transfer_from_to(p1, p2, pi1, pi2, QuantityKind::IronGangue);
        }
        // transfer heat iron_furnace generator
        (Kind::IronFurnace, Kind::Generator, QuantityKind::Heat) => {
            transfer_from_to(p1, p2, pi1, pi2, QuantityKind::Heat);
        }
        // transfer heat iron_furnace heat_collector
        (Kind::IronFurnace, Kind::HeatCollector, QuantityKind::Heat) => {
            transfer_from_to(p1, p2, pi1, pi2, QuantityKind::Heat);
        }
        // transfer energy generator battery
        (Kind::Generator, Kind::Battery, QuantityKind::Energy) => {
            transfer_from_to(p1, p2, pi1, pi2, QuantityKind::Energy);
        }
        // transfer ice ice_asteroid ice_collector
        (Kind::IceAsteroid, Kind::IceCollector, QuantityKind::Ice) => {
            transfer_from_to(p1, p2, pi1, pi2, QuantityKind::Ice);
        }
        // transfer ice ice_collector ice_cargo
        (Kind::IceCollector, Kind::IceCargo, QuantityKind::Ice) => {
            transfer_from_to(p1, p2, pi1, pi2, QuantityKind::Ice);
        }
        // transfer ice ice_cargo ice_collector
        (Kind::IceCargo, Kind::IceCollector, QuantityKind::Ice) => {
            transfer_from_to(p1, p2, pi1, pi2, QuantityKind::Ice);
        }
        // transfer ice ice_collector ice_melter
        (Kind::IceCollector, Kind::IceMelter, QuantityKind::Ice) => {
            transfer_from_to(p1, p2, pi1, pi2, QuantityKind::Ice);
        }
        // transfer water ice_melter generator
        (Kind::IceMelter, Kind::Generator, QuantityKind::Water) => {
            transfer_from_to(p1, p2, pi1, pi2, QuantityKind::Water);
        }
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
        // transform ice_asteroid=->1*ice
        Kind::IceAsteroid => {
            if p1.remaining_capacity(QuantityKind::Ice) >= 1 {
                p1.add_quantity(QuantityKind::Ice, 1);
                pi1.new_state = Some(State {
                    live: p1.live,
                });
            }
        }
        // transform iron_asteroid=->1*iron_ore
        Kind::IronAsteroid => {
            if p1.remaining_capacity(QuantityKind::IronOre) >= 1 {
                p1.add_quantity(QuantityKind::IronOre, 1);
                pi1.new_state = Some(State {
                    live: p1.live,
                });
            }
        }
        // transform coal_asteroid=->1*coal
        Kind::CoalAsteroid => {
            if p1.remaining_capacity(QuantityKind::Coal) >= 1 {
                p1.add_quantity(QuantityKind::Coal, 1);
                pi1.new_state = Some(State {
                    live: p1.live,
                });
            }
        }
        // transform booster=->10*energy
        Kind::Booster => {
            if p1.remaining_capacity(QuantityKind::Energy) >= 10 {
                p1.add_quantity(QuantityKind::Energy, 10);
                pi1.new_state = Some(State {
                    live: p1.live,
                });
            }
        }
        _ => {}
    }
}
impl Particle {
    pub fn qks(&self) -> &[QuantityKind] {
        match &self.k {
            Kind::Invalid => &[],
            Kind::Default => &[],
            Kind::Armor => &[],
            Kind::Core => &[QuantityKind::Energy],
            Kind::Booster => &[QuantityKind::Energy],
            Kind::Sun => &[],
            Kind::Light => &[],
            Kind::Plant => &[],
            Kind::Metal => &[],
            Kind::Depot => &[],
            Kind::Target => &[],
            Kind::ElectroFieldLauncher => &[QuantityKind::Energy],
            Kind::Cargo => &[],
            Kind::Plasma => &[],
            Kind::Field => &[],
            Kind::Anchor => &[],
            Kind::SunCore => &[],
            Kind::ElectroField => &[],
            Kind::PlasmaElectroField => &[],
            Kind::PlasmaCargo => &[],
            Kind::PlasmaElectroFieldCollector => &[],
            Kind::PlasmaRawDepot => &[],
            Kind::PlasmaRawCollector => &[],
            Kind::PlasmaRefineryInput => &[],
            Kind::PlasmaRefineryOutput => &[],
            Kind::Static => &[],
            Kind::Ice => &[],
            Kind::Water => &[QuantityKind::WaterDroplet],
            Kind::Heat => &[],
            Kind::HeatLauncher => &[QuantityKind::Energy],
            Kind::Generator => &[
                QuantityKind::Energy,
                QuantityKind::Water,
                QuantityKind::Heat,
            ],
            Kind::Fuel => &[],
            Kind::Electricity => &[],
            Kind::IronFurnace => &[
                QuantityKind::IronGangue,
                QuantityKind::Heat,
                QuantityKind::IronOre,
                QuantityKind::Coal,
                QuantityKind::Iron,
            ],
            Kind::CoalCargo => &[QuantityKind::Coal],
            Kind::CoalAsteroid => &[QuantityKind::Coal],
            Kind::CoalCollector => &[QuantityKind::Coal],
            Kind::IronCargo => &[],
            Kind::IronCollector => &[QuantityKind::Iron],
            Kind::IronAsteroid => &[QuantityKind::IronOre],
            Kind::IronOreCargo => &[QuantityKind::IronOre],
            Kind::IronOreCollector => &[QuantityKind::IronOre],
            Kind::Luciole => &[QuantityKind::WaterDroplet],
            Kind::Vers => &[],
            Kind::IceMelter => &[QuantityKind::Ice, QuantityKind::Water],
            Kind::IceCollector => &[QuantityKind::Ice],
            Kind::IceCargo => &[QuantityKind::Ice],
            Kind::IceAsteroid => &[QuantityKind::Ice],
            Kind::Battery => &[QuantityKind::Energy],
            Kind::IronGangueCollector => &[QuantityKind::IronGangue],
            Kind::CoalDepot => &[QuantityKind::Coal],
            Kind::IronOreDepot => &[QuantityKind::IronOre],
            Kind::HeatCollector => &[QuantityKind::Heat],
            Kind::CoalOutput => &[QuantityKind::Coal],
            Kind::CoalInput => &[QuantityKind::Coal],
            Kind::IronOreInput => &[QuantityKind::IronOre],
            Kind::IronOreOutput => &[QuantityKind::IronOre],
        }
    }
}
