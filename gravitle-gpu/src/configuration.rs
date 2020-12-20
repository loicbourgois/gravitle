use crate::particle::ParticleDefinition;
use crate::Constants;
use crate::Vec2;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Clone, Serialize, Deserialize)]
pub struct Configuration {
    pub constants: Constants,
    pub initial_max_speed_per_s: f32,
    pub multiplier: f32,
    pub port: u32,
    pub address: String,
    pub initial_particle_count: usize,
    pub gpu_id: usize,
    pub serialize_unactive_particles: bool,
    pub update_client_data: bool,
    pub show_gpu_supported_features: bool,
    pub alchemy: AlchemyConfiguration,
    pub particle_definitions: HashMap<String, ParticleDefinition>,
    pub particles: Vec<ParticleConfiguration>,
    pub default_particle_type: String,
    pub display_engine_logs: bool,
    pub engine_logs_refresh: usize,
    pub min_particle_count: usize,
    pub durations_length: usize,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct AlchemyConfiguration {
    pub collisions: Vec<CollisionResponseDefinitionOuter>,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct CollisionResponseDefinitionOuter {
    pub particles: [ParticleType; 2],
    pub response: CollisionResponseDefinition,
}
#[derive(Clone, Serialize, Deserialize)]
pub enum CollisionResponseDefinition {
    Transform(CrdTransform),
    Link(CrdLink),
}
#[derive(Clone, Serialize, Deserialize)]
pub struct CrdTransform {
    pub particles: Vec<ParticleType>,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct CrdLink {
    pub strength: f32,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct ParticleConfiguration {
    pub r#type: String,
    pub x: f32,
    pub y: f32,
    pub velocity_per_s: Vec2,
}
type ParticleType = String;
