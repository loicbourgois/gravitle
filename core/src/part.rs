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
}

impl From<u8> for Kind {
    fn from(v: u8) -> Self {
        match v {
            x if x == Kind::Core as u8 => Kind::Core,
            x if x == Kind::Metal as u8 => Kind::Metal,
            x if x == Kind::Turbo as u8 => Kind::Turbo,
            x if x == Kind::Mouth as u8 => Kind::Mouth,
            x if x == Kind::Energy as u8 => Kind::Energy,
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
