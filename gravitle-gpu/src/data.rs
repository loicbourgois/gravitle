use crate::configuration::CollisionResponseDefinition;
use crate::configuration::CrdLink;
use crate::particle::pdid;
use crate::particle::pid;
use crate::particle::Particle;
use crate::particle::ParticleDefinition;
use crate::CollisionCell;
use crate::Configuration;
use crate::GpuParticle;
//use crate::Vec2;
use crate::MAX_COLLISION_PER_PARTICLE;
use crate::MAX_COLLISION_TO_CHECK;
use crate::MAX_GRID_SIZE;
use crate::MAX_LINK_PER_PARTICLE;
use crate::MAX_PARTICLES_COUNT;
use crate::MAX_PARTICLE_DEFINITIONS;
use crate::PADDER_COUNT;
use arrayvec::ArrayVec;
use rand::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use vulkano::buffer::BufferUsage;
use vulkano::buffer::CpuAccessibleBuffer;
#[derive(Clone)]
pub enum CollisionResponseDefinitionInternal {
    Transform(CrdiTransform),
    Link(CrdLink),
}
#[derive(Clone)]
pub struct CrdiTransform {
    pub pdids: Vec<pdid>,
}
pub struct Data {
    pub inactive_particles: HashSet<usize>,
    pub active_particles: HashMap<pid, Particle>,
    pub gpu_buffer: std::sync::Arc<vulkano::buffer::CpuAccessibleBuffer<GpuData>>,
    pub particle_definitions: HashMap<String, ParticleDefinition>,
    pub pdid_to_string: HashMap<pdid, String>,
    pub string_to_pdid: HashMap<String, pdid>,
    pub collision_response_definitions: HashMap<(pdid, pdid), CollisionResponseDefinitionInternal>,
    pub configuration: Configuration,
    pub links: HashMap<(pdid, pdid), Link>,
}
pub struct Link {
    pub strength: f32,
}
pub struct GpuData {
    pub particles: [[GpuParticle; 2]; MAX_PARTICLES_COUNT],
    pub collision_grid: [[CollisionCell; MAX_GRID_SIZE]; MAX_GRID_SIZE],
    pub particle_definitions: [GpuParticleDefinition; MAX_PARTICLE_DEFINITIONS],
}
#[derive(Clone, Copy)]
pub struct GpuParticleDefinition {
    pub thrust: f32,
}
pub fn activate_particle(data: &mut Data, pdid: pdid) -> Option<pid> {
    if data.inactive_particles.len() == 0 {
        println!("not enough inactive particle");
        return None;
    }
    let pid: pid = *data.inactive_particles.iter().next().unwrap();
    data.inactive_particles.remove(&pid);
    {
        let gpu_buffer = &mut data.gpu_buffer.write().unwrap();
        let constants = data.configuration.constants;
        let mut rng = rand::thread_rng();
        let x = rng.gen::<f32>() * constants.width;
        let y = rng.gen::<f32>() * constants.height;
        let max_speed_per_tick =
            data.configuration.initial_max_speed_per_s * constants.delta_time_s;
        for i in 0..=1 {
            gpu_buffer.particles[pid][i] = GpuParticle {
                x: x,
                y: y,
                x_before: x + rng.gen::<f32>() * max_speed_per_tick - max_speed_per_tick * 0.5,
                y_before: y + rng.gen::<f32>() * max_speed_per_tick - max_speed_per_tick * 0.5,
                grid_x: ((x / constants.width * constants.grid_size as f32).abs() as u32)
                    .max(0)
                    .min(constants.grid_size - 1),
                grid_y: ((y / constants.height * constants.grid_size as f32).abs() as u32)
                    .max(0)
                    .min(constants.grid_size - 1),
                collisions_count: 0,
                collision_pids: [0; MAX_COLLISION_PER_PARTICLE],
                d: constants.default_diameter,
                mass: constants.default_mass,
                is_active: 1,
                velocity_x: 0.0,
                velocity_y: 0.0,
                momentum_x: 0.0,
                momentum_y: 0.0,
                pdid: pdid as u32,
                kinetic_energy: 0.0,
                padder: [0; PADDER_COUNT],
                //pdid: pdid as u32,
                link_count: 0,
                linked_pids: [0; MAX_LINK_PER_PARTICLE],
            };
        }
    }
    data.active_particles.insert(
        pid,
        Particle {
            pid: pid,
            pdid: pdid,
        },
    );
    return Some(pid);
}
pub fn create_default_particle(data: &mut Data) {
    let pdid_str: String = data.configuration.default_particle_type.clone();
    let pdid: pdid = *data.string_to_pdid.get(&pdid_str).unwrap();
    activate_particle(data, pdid);
}
pub fn load_data(
    configuration: &Configuration,
    device: std::sync::Arc<vulkano::device::Device>,
) -> Data {
    let mut inactive_particles = HashSet::new();
    for i in 0..MAX_PARTICLES_COUNT {
        inactive_particles.insert(i);
    }
    let constants = configuration.constants;
    let gpu_particles: [[GpuParticle; 2]; MAX_PARTICLES_COUNT] =
        get_random_particles(configuration, MAX_PARTICLES_COUNT)
            .into_iter()
            .collect::<ArrayVec<_>>()
            .into_inner()
            .unwrap_or_else(|_| unreachable!());
    let collision_cell = CollisionCell {
        count: 0,
        pids: [0; MAX_COLLISION_TO_CHECK],
    };
    let collision_grid: [[CollisionCell; MAX_GRID_SIZE]; MAX_GRID_SIZE] =
        [[collision_cell; MAX_GRID_SIZE]; MAX_GRID_SIZE];
    let mut gpu_particle_definitions: [GpuParticleDefinition; MAX_PARTICLE_DEFINITIONS] =
        [GpuParticleDefinition { thrust: 0.0 }; MAX_PARTICLE_DEFINITIONS];
    let host_cached = false;
    let mut pdid_counter = 0;
    let mut pdid_to_string = HashMap::new();
    let mut string_to_pdid = HashMap::new();
    for pd in configuration.particle_definitions.values() {
        pdid_to_string.insert(pdid_counter, pd.string_id.clone());
        string_to_pdid.insert(pd.string_id.clone(), pdid_counter);
        gpu_particle_definitions[pdid_counter] = GpuParticleDefinition { thrust: pd.thrust };
        pdid_counter += 1;
    }
    let gpu_buffer = CpuAccessibleBuffer::from_data(
        device,
        BufferUsage::all(),
        host_cached,
        GpuData {
            particles: gpu_particles,
            collision_grid: collision_grid,
            particle_definitions: gpu_particle_definitions,
        },
    )
    .expect("failed to create gpu buffer");
    let mut collision_response_definitions: HashMap<
        (pdid, pdid),
        CollisionResponseDefinitionInternal,
    > = HashMap::new();
    for crd_outer in &configuration.alchemy.collisions {
        let pdid_a = string_to_pdid.get(&crd_outer.particles[0]).unwrap();
        let pdid_b = string_to_pdid.get(&crd_outer.particles[1]).unwrap();
        match collision_response_definitions.get_mut(&(*pdid_a, *pdid_b)) {
            Some(_) => {
                panic!("problem with collision_response_definitions");
            }
            None => {
                let crdi = match &crd_outer.response {
                    CollisionResponseDefinition::Transform(crd) => {
                        let mut pdids: Vec<pdid> = Vec::new();
                        for str in &crd.particles {
                            pdids.push(*string_to_pdid.get(str).unwrap());
                        }
                        CollisionResponseDefinitionInternal::Transform(CrdiTransform {
                            pdids: pdids,
                        })
                    }
                    CollisionResponseDefinition::Link(crd) => {
                        CollisionResponseDefinitionInternal::Link(CrdLink {
                            strength: crd.strength,
                        })
                    }
                };
                collision_response_definitions.insert((*pdid_a, *pdid_b), crdi.clone());
                collision_response_definitions.insert((*pdid_b, *pdid_a), crdi);
            }
        }
    }
    let mut data = Data {
        inactive_particles: inactive_particles,
        active_particles: HashMap::new(),
        gpu_buffer: gpu_buffer,
        particle_definitions: configuration.particle_definitions.clone(),
        pdid_to_string: pdid_to_string,
        string_to_pdid: string_to_pdid,
        configuration: configuration.clone(),
        collision_response_definitions: collision_response_definitions,
        links: HashMap::new(),
    };
    for p in configuration.particles.iter() {
        match activate_particle_str(&mut data, p.r#type.clone()) {
            Some(pid) => {
                let gpu_buffer = &mut data.gpu_buffer.write().unwrap();
                for i in 0..=1 {
                    gpu_buffer.particles[pid][i].x = p.x;
                    gpu_buffer.particles[pid][i].y = p.y;
                    gpu_buffer.particles[pid][i].x_before =
                        p.x - p.velocity_per_s.x * constants.delta_time_s;
                    gpu_buffer.particles[pid][i].y_before =
                        p.y - p.velocity_per_s.y * constants.delta_time_s;
                    gpu_buffer.particles[pid][i].grid_x =
                        ((p.x / constants.width * constants.grid_size as f32).abs() as u32)
                            .max(0)
                            .min(constants.grid_size - 1);
                    gpu_buffer.particles[pid][i].grid_y =
                        ((p.y / constants.height * constants.grid_size as f32).abs() as u32)
                            .max(0)
                            .min(constants.grid_size - 1);
                }
            }
            None => {}
        }
    }
    for _ in configuration.particles.len()..configuration.initial_particle_count {
        let pdid_str = data.configuration.default_particle_type.clone();
        activate_particle_str(&mut data, pdid_str);
    }
    return data;
}
pub fn activate_particle_str(data: &mut Data, pdid_str: String) -> Option<pid> {
    let pdid: pdid = *data.string_to_pdid.get(&pdid_str).unwrap();
    return activate_particle(data, pdid);
}
pub fn deactivate_particle(data: &mut Data, pid: pid) {
    data.active_particles.remove(&pid);
    data.inactive_particles.insert(pid);
    let gpu_buffer = &mut data.gpu_buffer.write().unwrap();
    gpu_buffer.particles[pid][0].is_active = 0;
    gpu_buffer.particles[pid][1].is_active = 0;
}
pub fn create_link(data: &mut Data, pida: pid, pidb: pid, link: Link) {
    data.links.insert((pida, pidb), link);
    let gpu_buffer = &mut data.gpu_buffer.write().unwrap();
    let a = gpu_buffer.particles[pida][0].link_count as usize;
    let b = gpu_buffer.particles[pidb][0].link_count as usize;
    for i in 0..=1 {
        gpu_buffer.particles[pida][i].linked_pids[a] = pidb as u32;
        gpu_buffer.particles[pida][i].link_count += 1;
        gpu_buffer.particles[pidb][i].linked_pids[b] = pida as u32;
        gpu_buffer.particles[pidb][i].link_count += 1;
    }
}
fn get_random_particles(
    configuration: &Configuration,
    total_count: usize,
) -> Vec<[GpuParticle; 2]> {
    let mut gpu_particles = Vec::new();
    let mut rng = rand::thread_rng();
    let constants = configuration.constants;
    for _ in 0..total_count {
        let x = rng.gen::<f32>() * constants.width;
        let y = rng.gen::<f32>() * constants.height;
        let max_speed_per_tick = configuration.initial_max_speed_per_s * constants.delta_time_s;
        let gpu_particle = GpuParticle {
            x: x,
            y: y,
            x_before: x + rng.gen::<f32>() * max_speed_per_tick - max_speed_per_tick * 0.5,
            y_before: y + rng.gen::<f32>() * max_speed_per_tick - max_speed_per_tick * 0.5,
            grid_x: ((x / constants.width * constants.grid_size as f32).abs() as u32)
                .max(0)
                .min(constants.grid_size - 1),
            grid_y: ((y / constants.height * constants.grid_size as f32).abs() as u32)
                .max(0)
                .min(constants.grid_size - 1),
            collisions_count: 0,
            collision_pids: [0; MAX_COLLISION_PER_PARTICLE],
            d: constants.default_diameter,
            mass: constants.default_mass,
            is_active: 0,
            velocity_x: 0.0,
            velocity_y: 0.0,
            momentum_x: 0.0,
            momentum_y: 0.0,
            kinetic_energy: 0.0,
            padder: [0; PADDER_COUNT],
            link_count: 0,
            linked_pids: [0; MAX_LINK_PER_PARTICLE],
            //pdid: 0,
            pdid: 0,
        };
        gpu_particles.push([gpu_particle, gpu_particle]);
    }
    return gpu_particles;
}
pub fn update_grid_coord(data: &mut Data, pid: pid) {
    for i in 0..=1 {
        let mut gpu_buffer = data.gpu_buffer.write().unwrap();
        gpu_buffer.particles[pid][i].grid_x = ((gpu_buffer.particles[pid][i].x
            / data.configuration.constants.width
            * data.configuration.constants.grid_size as f32)
            .abs() as u32)
            .max(0)
            .min(data.configuration.constants.grid_size - 1);
        gpu_buffer.particles[pid][i].grid_y = ((gpu_buffer.particles[pid][i].y
            / data.configuration.constants.height
            * data.configuration.constants.grid_size as f32)
            .abs() as u32)
            .max(0)
            .min(data.configuration.constants.grid_size - 1)
    }
}
