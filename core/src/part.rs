use crate::point::Point;
use crate::Float;
use serde::Deserialize;
use serde::Serialize;
use std::u32;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(into = "u32")]
pub enum Kind {
    Invalid = 0,
    Core = 1,
    Metal = 2,
    Turbo = 3,
    Mouth = 4,
    Energy = 5,
    Muscle = 6,
    Grip = 7,
    Eye = 8,
}

pub const BLOOP_KIND_COUNT: u8 = 8;

pub const BLOOP_KINDS: [Kind; BLOOP_KIND_COUNT as usize] = [
    Kind::Invalid,
    Kind::Core,
    Kind::Metal,
    Kind::Mouth,
    Kind::Muscle,
    Kind::Grip,
    Kind::Turbo,
    Kind::Eye,
];

pub fn bloop_dna_to_kind(i: u8) -> Kind {
    let i2 = i % BLOOP_KIND_COUNT;
    match i2 {
        0 => Kind::Invalid,
        1 => Kind::Core,
        2 => Kind::Metal,
        3 => Kind::Mouth,
        4 => Kind::Muscle,
        5 => Kind::Grip,
        6 => Kind::Turbo,
        7 => Kind::Eye,
        _ => panic!("bloop_dna_to_kind error")
    }
}

pub fn kind_to_bloop_dna(i: Kind) -> u8 {
    match i {
        Kind::Invalid => 0,
         Kind::Core => 1,
        Kind::Metal => 2,
        Kind::Mouth => 3,
        Kind::Muscle =>4,
        Kind::Grip => 5,
        Kind::Turbo => 6,
        Kind::Eye => 7,
        _ => panic!("kind_to_bloop_dna error")
    }
}

impl From<u8> for Kind {
    fn from(v: u8) -> Self {
        match v {
            x if x == Kind::Core as u8 => Kind::Core,
            x if x == Kind::Metal as u8 => Kind::Metal,
            x if x == Kind::Turbo as u8 => Kind::Turbo,
            x if x == Kind::Mouth as u8 => Kind::Mouth,
            x if x == Kind::Energy as u8 => Kind::Energy,
            x if x == Kind::Muscle as u8 => Kind::Muscle,
            x if x == Kind::Grip as u8 => Kind::Grip,
            x if x == Kind::Eye as u8 => Kind::Eye,
            _ => Kind::Invalid
        }
    }
}

impl From<u32> for Kind {
    fn from(v: u32) -> Self {
        match v {
            x if x == Kind::Core as u32 => Kind::Core,
            x if x == Kind::Metal as u32 => Kind::Metal,
            x if x == Kind::Turbo as u32 => Kind::Turbo,
            x if x == Kind::Mouth as u32 => Kind::Mouth,
            x if x == Kind::Energy as u32 => Kind::Energy,
            x if x == Kind::Muscle as u32 => Kind::Muscle,
            x if x == Kind::Grip as u32 => Kind::Grip,
            x if x == Kind::Eye as u32 => Kind::Eye,
            _ => Kind::Invalid
        }
    }
}

impl From<Kind> for u32 {
    fn from(v: Kind) -> u32 {
        match v {
            Kind::Invalid => 0,
            Kind::Core => 1,
            Kind::Metal => 2,
            Kind::Turbo => 3,
            Kind::Mouth => 4,
            Kind::Energy => 5,
            Kind::Muscle => 6,
            Kind::Grip => 7,
            Kind::Eye => 8,
        }
    }
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Part {
    pub d: Float,
    pub m: Float,
    pub p: Point,
    pub pp: Point,
    pub kind: Kind,
    pub energy: Float,
    pub activity: Float,
    pub uuid: u128,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
