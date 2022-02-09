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
}

impl From<u32> for Kind {
    fn from(v: u32) -> Self {
        match v {
            x if x == Kind::Core as u32 => Kind::Core,
            x if x == Kind::Metal as u32 => Kind::Metal,
            x if x == Kind::Turbo as u32 => Kind::Turbo,
            x if x == Kind::Mouth as u32 => Kind::Mouth,
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
}
