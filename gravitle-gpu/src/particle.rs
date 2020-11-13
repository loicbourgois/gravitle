use serde::{Deserialize, Serialize};
// particle id
#[allow(non_camel_case_types)]
pub type pid = usize;
// particle definition id
#[allow(non_camel_case_types)]
pub type pdid = usize;
#[derive(Clone, Serialize, Deserialize)]
pub struct ParticleDefinition {
    pub string_id: String,
    pub name: String,
    pub short_name: String,
    pub thrust: f32,
}
pub struct Particle {
    pub pid: pid,
    pub pdid: pdid,
}
