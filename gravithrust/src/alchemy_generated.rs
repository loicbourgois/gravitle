use crate::alchemy::transfer_from_to;
use crate::kind::Kind;
use crate::particle::ParticleInternal;
use crate::particle::QuantityKind;
use crate::particle::State;
use crate::particle::Particle;
#[allow(clippy::too_many_lines)]
pub fn process_alchemy_transfer(
    p1: &mut Particle,
    p2: &mut Particle,
    pi1: &mut ParticleInternal,
    pi2: &mut ParticleInternal,
    qk: QuantityKind,
) {
    match (p1.k, p2.k, qk) {
        // transfer ice ice_asteroid ice_collector
(Kind::IceAsteroid, Kind::IceCollector, QuantityKind::Ice) => {
  transfer_from_to(p1, p2, pi1, pi2, QuantityKind::Ice);
}
  // transfer ice ice_cargo ice_collector
(Kind::IceCargo, Kind::IceCollector, QuantityKind::Ice) => {
  transfer_from_to(p1, p2, pi1, pi2, QuantityKind::Ice);
}
  // transfer ice ice_collector ice_cargo
(Kind::IceCollector, Kind::IceCargo, QuantityKind::Ice) => {
  transfer_from_to(p1, p2, pi1, pi2, QuantityKind::Ice);
}
  // transfer ice ice_collector ice_melter
(Kind::IceCollector, Kind::IceMelter, QuantityKind::Ice) => {
  transfer_from_to(p1, p2, pi1, pi2, QuantityKind::Ice);
}
  // transfer water water luciole
(Kind::Water, Kind::Luciole, QuantityKind::Water) => {
  transfer_from_to(p1, p2, pi1, pi2, QuantityKind::Water);
}
  // transfer water ice_melter water_collector
(Kind::IceMelter, Kind::WaterCollector, QuantityKind::Water) => {
  transfer_from_to(p1, p2, pi1, pi2, QuantityKind::Water);
}
  // transfer water water_collector generator
(Kind::WaterCollector, Kind::Generator, QuantityKind::Water) => {
  transfer_from_to(p1, p2, pi1, pi2, QuantityKind::Water);
}
  // transfer water water flower
(Kind::Water, Kind::Flower, QuantityKind::Water) => {
  transfer_from_to(p1, p2, pi1, pi2, QuantityKind::Water);
}
  // transfer nectar flower luciole
(Kind::Flower, Kind::Luciole, QuantityKind::Nectar) => {
  transfer_from_to(p1, p2, pi1, pi2, QuantityKind::Nectar);
}
  // transfer coal coal_collector coal_depot
(Kind::CoalCollector, Kind::CoalDepot, QuantityKind::Coal) => {
  transfer_from_to(p1, p2, pi1, pi2, QuantityKind::Coal);
}
  // transfer coal coal_depot iron_furnace
(Kind::CoalDepot, Kind::IronFurnace, QuantityKind::Coal) => {
  transfer_from_to(p1, p2, pi1, pi2, QuantityKind::Coal);
}
  // transfer coal coal_depot coal_depot
(Kind::CoalDepot, Kind::CoalDepot, QuantityKind::Coal) => {
  transfer_from_to(p1, p2, pi1, pi2, QuantityKind::Coal);
}
  // transfer coal coal_asteroid coal_collector
(Kind::CoalAsteroid, Kind::CoalCollector, QuantityKind::Coal) => {
  transfer_from_to(p1, p2, pi1, pi2, QuantityKind::Coal);
}
  // transfer coal coal_collector coal_cargo
(Kind::CoalCollector, Kind::CoalCargo, QuantityKind::Coal) => {
  transfer_from_to(p1, p2, pi1, pi2, QuantityKind::Coal);
}
  // transfer coal coal_cargo coal_collector
(Kind::CoalCargo, Kind::CoalCollector, QuantityKind::Coal) => {
  transfer_from_to(p1, p2, pi1, pi2, QuantityKind::Coal);
}
  // transfer iron_ore iron_asteroid iron_ore_collector
(Kind::IronAsteroid, Kind::IronOreCollector, QuantityKind::IronOre) => {
  transfer_from_to(p1, p2, pi1, pi2, QuantityKind::IronOre);
}
  // transfer iron_ore iron_ore_collector iron_ore_depot
(Kind::IronOreCollector, Kind::IronOreDepot, QuantityKind::IronOre) => {
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
  // transfer iron_ore iron_ore_depot iron_furnace
(Kind::IronOreDepot, Kind::IronFurnace, QuantityKind::IronOre) => {
  transfer_from_to(p1, p2, pi1, pi2, QuantityKind::IronOre);
}
  // transfer iron_ore iron_ore_depot iron_ore_depot
(Kind::IronOreDepot, Kind::IronOreDepot, QuantityKind::IronOre) => {
  transfer_from_to(p1, p2, pi1, pi2, QuantityKind::IronOre);
}
  // transfer iron iron_furnace iron_collector
(Kind::IronFurnace, Kind::IronCollector, QuantityKind::Iron) => {
  transfer_from_to(p1, p2, pi1, pi2, QuantityKind::Iron);
}
  // transfer iron_gangue iron_furnace iron_gangue_collector
(Kind::IronFurnace, Kind::IronGangueCollector, QuantityKind::IronGangue) => {
  transfer_from_to(p1, p2, pi1, pi2, QuantityKind::IronGangue);
}
  // transfer heat heat_collector generator
(Kind::HeatCollector, Kind::Generator, QuantityKind::Heat) => {
  transfer_from_to(p1, p2, pi1, pi2, QuantityKind::Heat);
}
  // transfer heat iron_furnace heat_collector
(Kind::IronFurnace, Kind::HeatCollector, QuantityKind::Heat) => {
  transfer_from_to(p1, p2, pi1, pi2, QuantityKind::Heat);
}
  // transfer energy generator energy_depot
(Kind::Generator, Kind::EnergyDepot, QuantityKind::Energy) => {
  transfer_from_to(p1, p2, pi1, pi2, QuantityKind::Energy);
}
  // transfer energy energy_collector battery
(Kind::EnergyCollector, Kind::Battery, QuantityKind::Energy) => {
  transfer_from_to(p1, p2, pi1, pi2, QuantityKind::Energy);
}
  // transfer energy energy_collector energy_cargo
(Kind::EnergyCollector, Kind::EnergyCargo, QuantityKind::Energy) => {
  transfer_from_to(p1, p2, pi1, pi2, QuantityKind::Energy);
}
  // transfer energy energy_cargo energy_collector
(Kind::EnergyCargo, Kind::EnergyCollector, QuantityKind::Energy) => {
  transfer_from_to(p1, p2, pi1, pi2, QuantityKind::Energy);
}
  // transfer energy energy_cargo energy_cargo
(Kind::EnergyCargo, Kind::EnergyCargo, QuantityKind::Energy) => {
  transfer_from_to(p1, p2, pi1, pi2, QuantityKind::Energy);
}
  // transfer energy core booster
(Kind::Core, Kind::Booster, QuantityKind::Energy) => {
  transfer_from_to(p1, p2, pi1, pi2, QuantityKind::Energy);
}
  // transfer energy battery ice_melter
(Kind::Battery, Kind::IceMelter, QuantityKind::Energy) => {
  transfer_from_to(p1, p2, pi1, pi2, QuantityKind::Energy);
}
  // transfer energy battery booster
(Kind::Battery, Kind::Booster, QuantityKind::Energy) => {
  transfer_from_to(p1, p2, pi1, pi2, QuantityKind::Energy);
}
  // transfer energy energy_depot energy_collector
(Kind::EnergyDepot, Kind::EnergyCollector, QuantityKind::Energy) => {
  transfer_from_to(p1, p2, pi1, pi2, QuantityKind::Energy);
}
  // transfer energy core energy_cargo
(Kind::Core, Kind::EnergyCargo, QuantityKind::Energy) => {
  transfer_from_to(p1, p2, pi1, pi2, QuantityKind::Energy);
}
  // transfer energy core energy_collector
(Kind::Core, Kind::EnergyCollector, QuantityKind::Energy) => {
  transfer_from_to(p1, p2, pi1, pi2, QuantityKind::Energy);
}
  // transfer energy battery battery
(Kind::Battery, Kind::Battery, QuantityKind::Energy) => {
  transfer_from_to(p1, p2, pi1, pi2, QuantityKind::Energy);
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
    pi1.new_state = Some(State {live: p1.live});
  }
},
  // transform iron_asteroid=->1*iron_ore
Kind::IronAsteroid => {
  if p1.remaining_capacity(QuantityKind::IronOre) >= 1 {
    p1.add_quantity(QuantityKind::IronOre, 1);
    pi1.new_state = Some(State {live: p1.live});
  }
},
  // transform coal_asteroid=->1*coal
Kind::CoalAsteroid => {
  if p1.remaining_capacity(QuantityKind::Coal) >= 1 {
    p1.add_quantity(QuantityKind::Coal, 1);
    pi1.new_state = Some(State {live: p1.live});
  }
},
  // transform core=->1000*energy
Kind::Core => {
  if p1.remaining_capacity(QuantityKind::Energy) >= 1000 {
    p1.add_quantity(QuantityKind::Energy, 1000);
    pi1.new_state = Some(State {live: p1.live});
  }
},
  // transform flower=5*water->1*nectar
Kind::Flower => {
  if p1.quantity(QuantityKind::Water) >= 5&&p1.remaining_capacity(QuantityKind::Nectar) >= 1 {
    p1.remove_quantity(QuantityKind::Water, 5);
p1.add_quantity(QuantityKind::Nectar, 1);
    pi1.new_state = Some(State {live: p1.live});
  }
},
        _ => {}
    }
}
impl Particle {
    pub fn qks(&self) -> &[QuantityKind] {
        match &self.k {
            Kind::IronCollector => &[QuantityKind::Iron],
Kind::Luciole => &[QuantityKind::Water,QuantityKind::Nectar],
Kind::CoalAsteroid => &[QuantityKind::Coal],
Kind::IceMelter => &[QuantityKind::Water,QuantityKind::Ice,QuantityKind::Energy],
Kind::PlasmaRefineryOutput => &[],
Kind::IronOreCollector => &[QuantityKind::IronOre],
Kind::PlasmaRefineryInput => &[],
Kind::IronOreCargo => &[QuantityKind::IronOre],
Kind::IronOreDepot => &[QuantityKind::IronOre],
Kind::IceCollector => &[QuantityKind::Ice],
Kind::ElectroFieldLauncher => &[],
Kind::Battery => &[QuantityKind::Energy],
Kind::IronAsteroid => &[QuantityKind::IronOre],
Kind::Core => &[QuantityKind::Energy],
Kind::Flower => &[QuantityKind::Nectar,QuantityKind::Water],
Kind::Sun => &[],
Kind::Light => &[],
Kind::HeatCollector => &[QuantityKind::Heat],
Kind::Target => &[],
Kind::IronFurnace => &[QuantityKind::Iron,QuantityKind::IronOre,QuantityKind::IronGangue,QuantityKind::Heat,QuantityKind::Coal],
Kind::PlasmaRawDepot => &[],
Kind::WaterCollector => &[QuantityKind::Water],
Kind::PlasmaRawCollector => &[],
Kind::EnergyCargo => &[QuantityKind::Energy],
Kind::SunCore => &[],
Kind::Armor => &[],
Kind::IceAsteroid => &[QuantityKind::Ice],
Kind::IceCargo => &[QuantityKind::Ice],
Kind::CoalDepot => &[QuantityKind::Coal],
Kind::EnergyDepot => &[QuantityKind::Energy],
Kind::PlasmaCargo => &[],
Kind::PlasmaElectroFieldCollector => &[],
Kind::EnergyCollector => &[QuantityKind::Energy],
Kind::Anchor => &[],
Kind::CoalCargo => &[QuantityKind::Coal],
Kind::Static => &[],
Kind::Booster => &[QuantityKind::Energy],
Kind::CoalCollector => &[QuantityKind::Coal],
Kind::IronGangueCollector => &[QuantityKind::IronGangue],
Kind::Cargo => &[],
Kind::Water => &[QuantityKind::Water],
Kind::Generator => &[QuantityKind::Energy,QuantityKind::Water,QuantityKind::Heat],
        }
    }
}

