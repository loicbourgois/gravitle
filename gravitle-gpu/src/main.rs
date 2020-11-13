#![deny(warnings)]
mod configuration;
mod cs;
mod data;
mod particle;
mod point;
mod server;
mod vector;
use crate::configuration::Configuration;
use crate::data::activate_particle;
use crate::data::create_default_particle;
use crate::data::create_link;
use crate::data::deactivate_particle;
use crate::data::load_data;
use crate::data::update_grid_coord;
use crate::data::CollisionResponseDefinitionInternal;
use crate::data::Link;
use crate::particle::pdid;
use crate::particle::pid;
use crate::particle::ParticleDefinition;
use crate::vector::Vector;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::mem::size_of;
use std::sync::{Arc, RwLock};
use std::thread;
//use std::time::Duration;
use std::time::SystemTime;
use vulkano::command_buffer::AutoCommandBufferBuilder;
use vulkano::command_buffer::CommandBuffer;
use vulkano::descriptor::descriptor_set::PersistentDescriptorSet;
use vulkano::descriptor::PipelineLayoutAbstract;
use vulkano::device::Device;
use vulkano::device::DeviceExtensions;
use vulkano::device::Features;
use vulkano::instance::Instance;
use vulkano::instance::InstanceExtensions;
use vulkano::instance::PhysicalDevice;
use vulkano::pipeline::ComputePipeline;
use vulkano::sync::GpuFuture;
//#[allow(non_camel_case_types)]
//type pid = usize;
const MAX_COLLISION_PER_PARTICLE: usize = 64;
const MAX_COLLISION_TO_CHECK: usize = 1024;
const MAX_PARTICLES_COUNT: usize = 1024 * 64;
const MAX_GRID_SIZE: usize = 128;
const MAX_LINK_PER_PARTICLE: usize = 32;
const PADDER_COUNT: usize = 2;
const MAX_PARTICLE_DEFINITIONS: usize = 64;
#[derive(Clone, Copy)]
//#[allow(dead_code)]
//#[repr(align(1))]
pub struct GpuParticle {
    pub pdid: u32,
    pub link_count: u32,
    pub collision_pids: [u32; MAX_COLLISION_PER_PARTICLE],
    pub linked_pids: [u32; MAX_LINK_PER_PARTICLE],
    pub velocity_x: f32,
    pub velocity_y: f32,
    pub momentum_x: f32,
    pub momentum_y: f32,
    pub is_active: u32,
    pub d: f32,
    pub x: f32,
    pub y: f32,
    pub x_before: f32,
    pub y_before: f32,
    pub mass: f32,
    pub kinetic_energy: f32,
    pub grid_x: u32,
    pub grid_y: u32,
    pub collisions_count: u32,
    pub padder: [u32; PADDER_COUNT],
    //pub pdid: u32,
}
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Constants {
    pub width: f32,
    pub height: f32,
    pub delta_time_s: f32,
    pub grid_size: u32,
    pub default_diameter: f32,
    pub world_size: f32,
    pub collision_push_rate: f32,
    pub default_mass: f32,
    pub gravity: Vec2,
}
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}
#[allow(dead_code)]
struct PushConstants {
    gravity: Vec2,
    i_source: u32,
    i_target: u32,
    width: f32,
    height: f32,
    grid_size: u32,
    delta_time_s: f32,
    collision_push_rate: f32,
}
#[derive(Serialize, Deserialize)]
struct ParticleClientData {
    x: f32,
    y: f32,
    d: f32,
    a: bool,
    pdid: pdid,
}
#[derive(Serialize, Deserialize)]
struct ClientData {
    particles: Vec<ParticleClientData>,
    constants: Constants,
    tick: usize,
    momentum: Vec2,
    kinetic_energy: f32,
    absolute_momentum: Vec2,
    average_duration: u128,
    particle_definitions: HashMap<String, ParticleDefinition>,
    pdid_to_string: HashMap<pdid, String>,
}
#[derive(Copy, Clone)]
pub struct CollisionCell {
    pub count: u32,
    pub pids: [u32; MAX_COLLISION_TO_CHECK],
}
fn main() {
    println!("Blooper");
    let configuration_str: String = env::var("blooper_configuration")
        .unwrap()
        .replace("\\\"", "\"");
    let configuration: Configuration = serde_json::from_str(&configuration_str).unwrap();
    let instance =
        Instance::new(None, &InstanceExtensions::none(), None).expect("failed to create instance");
    let physical_devices = PhysicalDevice::enumerate(&instance);
    println!("Available devices");
    for (i, physical_device) in physical_devices.clone().enumerate() {
        println!("  {} - {}", i, physical_device.name());
        if configuration.show_gpu_supported_features {
            println!("    {:?}", physical_device.supported_features());
        }
    }
    let physical_device_id = configuration.gpu_id;
    let physical_device = physical_devices.collect::<Vec<PhysicalDevice>>()[physical_device_id];
    println!("using physical device {}", physical_device.name());
    println!("Families");
    for family in physical_device.queue_families() {
        println!("  #{:?}", family.id());
        println!("    queues: {:?}", family.queues_count());
        println!("    supports_compute: {:?}", family.supports_compute());
    }
    let family_id = 0;
    println!("using family #{}", family_id);
    let extensions = DeviceExtensions {
        khr_storage_buffer_storage_class: true,
        ..DeviceExtensions::none()
    };
    let features = Features { ..Features::none() };
    let (device, mut queues) = {
        Device::new(
            physical_device,
            &features,
            &extensions,
            [(physical_device.queue_families().next().unwrap(), 0.5)]
                .iter()
                .cloned(),
        )
        .expect("failed to create device")
    };
    println!("loaded extensions: {:#?}", device.loaded_extensions());
    let queue = queues.next().unwrap();
    // todo: const
    let local_size_x = 64;
    let work_groups_count: u32 = MAX_PARTICLES_COUNT as u32 / local_size_x;
    let particles_size = size_of::<[[GpuParticle; 2]; MAX_PARTICLES_COUNT]>();
    let collision_grid_size = size_of::<[[CollisionCell; MAX_GRID_SIZE]; MAX_GRID_SIZE]>();
    let stack_size = particles_size * 8 + collision_grid_size * 4 + 100_000;
    println!("size: {}", stack_size);
    let thread_builder = thread::Builder::new();
    let handler = thread_builder
        .stack_size(stack_size)
        .spawn(move || {
            let constants = Constants {
                world_size: configuration.constants.world_size * configuration.multiplier,
                width: configuration.constants.width * configuration.multiplier,
                height: configuration.constants.height * configuration.multiplier,
                default_diameter: configuration.constants.default_diameter
                    * configuration.multiplier,
                ..configuration.constants
            };
            let mut data = load_data(&configuration, device.clone());
            let shader = cs::Shader::load(device.clone()).expect("failed to create shader module");
            let compute_pipeline = Arc::new(
                ComputePipeline::new(device.clone(), &shader.main_entry_point(), &())
                    .expect("failed to create compute pipeline"),
            );
            let layout = compute_pipeline.layout().descriptor_set_layout(0).unwrap();
            let set = Arc::new(
                PersistentDescriptorSet::start(layout.clone())
                    .add_buffer(data.gpu_buffer.clone())
                    .unwrap()
                    .build()
                    .unwrap(),
            );
            let mut tick = 0;
            let client_data = "".to_string();
            let client_data_lock = Arc::new(RwLock::new(client_data));
            //
            // thread client
            //
            {
                let client_data_lock_clone = Arc::clone(&client_data_lock);
                let configuration_clone = configuration.clone();
                thread::spawn(move || {
                    server::handle_websocket(client_data_lock_clone, configuration_clone);
                });
            }
            let client_data_lock_clone = Arc::clone(&client_data_lock);
            let mut durations: Vec<u128> = Vec::new();
            let mut durations_fix_nan: Vec<u128> = Vec::new();
            let mut durations_collision_grid: Vec<u128> = Vec::new();
            let mut durations_gpu_compute: Vec<u128> = Vec::new();
            let mut durations_collision_response: Vec<u128> = Vec::new();
            let mut durations_client_data: Vec<u128> = Vec::new();
            loop {
                let start_time = SystemTime::now();
                let i_source: usize = tick % 2;
                let i_target: usize = (i_source as usize + 1) % 2;
                //
                if data.active_particles.len() < data.configuration.min_particle_count {
                    create_default_particle(&mut data);
                }
                // fix nan
                {
                    let start_time_fix_nan = SystemTime::now();
                    let mut particles_to_deactivate: HashSet<pid> = HashSet::new();
                    {
                        let gpu_buffer_read = data.gpu_buffer.read().unwrap();
                        for pid in data.active_particles.keys() {
                            let p_gpu = gpu_buffer_read.particles[*pid];
                            if !p_gpu[i_source].x.is_finite() || !p_gpu[i_source].y.is_finite() {
                                particles_to_deactivate.insert(*pid);
                            }
                        }
                    }
                    for pid in particles_to_deactivate {
                        deactivate_particle(&mut data, pid);
                        println!("fix nan for {}", pid);
                    }
                    match SystemTime::now().duration_since(start_time_fix_nan) {
                        Ok(t) => durations_fix_nan.push(t.as_micros()),
                        Err(_) => println!("error getting elapsed time"),
                    }
                }
                // collision grid
                {
                    let start_time_collisison_grid = SystemTime::now();
                    let mut buffer_write = data.gpu_buffer.write().unwrap();
                    for i in 0..constants.grid_size as usize {
                        for j in 0..constants.grid_size as usize {
                            buffer_write.collision_grid[i][j].count = 0;
                        }
                    }
                    for pid in data.active_particles.keys() {
                        let p = buffer_write.particles[*pid][i_source];
                        let i = p.grid_x as usize;
                        let j = p.grid_y as usize;
                        let c = { buffer_write.collision_grid[i][j].count as usize };
                        if c < MAX_COLLISION_TO_CHECK {
                            buffer_write.collision_grid[i][j].pids[c] = *pid as u32;
                        } else {
                            //
                        }
                        buffer_write.collision_grid[i][j].count += 1;
                    }
                    match SystemTime::now().duration_since(start_time_collisison_grid) {
                        Ok(t) => durations_collision_grid.push(t.as_micros()),
                        Err(_) => println!("error getting elapsed time"),
                    }
                }
                // gpu compute
                {
                    let start_time_gpu_compute = SystemTime::now();
                    let push_constants = PushConstants {
                        i_source: i_source as u32,
                        i_target: i_target as u32,
                        grid_size: constants.grid_size,
                        width: constants.width,
                        height: constants.height,
                        delta_time_s: constants.delta_time_s,
                        collision_push_rate: constants.collision_push_rate,
                        gravity: constants.gravity,
                    };
                    let mut builder =
                        AutoCommandBufferBuilder::new(device.clone(), queue.family()).unwrap();
                    builder
                        .dispatch(
                            [work_groups_count, 1, 1],
                            compute_pipeline.clone(),
                            set.clone(),
                            push_constants,
                        )
                        .unwrap();
                    let command_buffer = builder.build().unwrap();
                    let finished = command_buffer.execute(queue.clone()).unwrap();
                    finished
                        .then_signal_fence_and_flush()
                        .unwrap()
                        .wait(None)
                        .unwrap();

                    match SystemTime::now().duration_since(start_time_gpu_compute) {
                        Ok(t) => durations_gpu_compute.push(t.as_micros()),
                        Err(_) => println!("error getting elapsed time"),
                    }
                }
                // collision response
                let start_time_collision_response = SystemTime::now();
                struct ParticleConfigurationVerlet {
                    pub pdid: pdid,
                    pub x: f32,
                    pub y: f32,
                    pub x_before: f32,
                    pub y_before: f32,
                }
                let mut particles_to_create = Vec::new();
                let mut particles_to_deactivate: HashSet<pid> = HashSet::new();
                let mut links_to_create: HashMap<(pid, pid), Link> = HashMap::new();
                {
                    let mut buffer_write = data.gpu_buffer.write().unwrap();
                    for pid_a in data.active_particles.keys() {
                        let pa = data.active_particles.get(pid_a).unwrap();
                        let mut c = buffer_write.particles[*pid_a][i_source].collisions_count;
                        if c > MAX_COLLISION_PER_PARTICLE as u32 {
                            c = MAX_COLLISION_PER_PARTICLE as u32;
                        }
                        for i in 0..c {
                            let pas = &buffer_write.particles[*pid_a][i_source];
                            let pid_b = pas.collision_pids[i as usize] as usize;
                            match data.active_particles.get(&pid_b) {
                                Some(pb) => {
                                    let pbs = &buffer_write.particles[pid_b][i_source];
                                    match data
                                        .collision_response_definitions
                                        .get(&(pa.pdid, pb.pdid))
                                    {
                                        Some(crd_) => match crd_ {
                                            CollisionResponseDefinitionInternal::Transform(
                                                crdi,
                                            ) => match crdi.pdids.len() {
                                                1 => {
                                                    particles_to_deactivate.insert(*pid_a);
                                                    particles_to_deactivate.insert(pid_b);
                                                    let x = (pas.x + pbs.x) * 0.5;
                                                    let y = (pas.y + pbs.y) * 0.5;
                                                    particles_to_create.push(
                                                        ParticleConfigurationVerlet {
                                                            pdid: crdi.pdids[0],
                                                            x: x,
                                                            y: y,
                                                            x_before: x,
                                                            y_before: y,
                                                        },
                                                    );
                                                    // caution
                                                    break;
                                                }
                                                2 => {
                                                    particles_to_deactivate.insert(*pid_a);
                                                    particles_to_deactivate.insert(pid_b);
                                                    particles_to_create.push(
                                                        ParticleConfigurationVerlet {
                                                            pdid: crdi.pdids[0],
                                                            x: pas.x,
                                                            y: pas.y,
                                                            x_before: pas.x_before,
                                                            y_before: pas.y_before,
                                                        },
                                                    );
                                                    particles_to_create.push(
                                                        ParticleConfigurationVerlet {
                                                            pdid: crdi.pdids[1],
                                                            x: pbs.x,
                                                            y: pbs.y,
                                                            x_before: pbs.x_before,
                                                            y_before: pbs.y_before,
                                                        },
                                                    );
                                                    // caution
                                                    // avoid transforming a particle twice, but could have sides effect
                                                    // also, only works for low pid
                                                    // pid_b can be Transformed multiple times
                                                    break;
                                                }
                                                n => println!(
                                                    "Transform not supported for {} output",
                                                    n
                                                ),
                                            },
                                            CollisionResponseDefinitionInternal::Link(
                                                crdi_link,
                                            ) => {
                                                match data.links.get(&(pa.pid, pb.pid)) {
                                                    Some(_) => {
                                                        // do nothing
                                                    }
                                                    None => {
                                                        links_to_create.insert(
                                                            (pa.pid, pb.pid),
                                                            Link {
                                                                strength: crdi_link.strength,
                                                            },
                                                        );
                                                        // caution
                                                        break;
                                                    }
                                                }
                                            }
                                        },
                                        None => {
                                            let v = Vector::new_2(pas.x, pas.y, pbs.x, pbs.y);
                                            let distance_centers = v.length();
                                            let radiuses = (pas.d * 0.5) + (pbs.d * 0.5);
                                            let delta = radiuses - distance_centers;
                                            let delta_vector = v.normalized().multiplied(delta);
                                            buffer_write.particles[*pid_a][i_target].x -=
                                                delta_vector.x * constants.collision_push_rate;
                                            buffer_write.particles[*pid_a][i_target].y -=
                                                delta_vector.y * constants.collision_push_rate;
                                            buffer_write.particles[pid_b][i_target].x +=
                                                delta_vector.x * constants.collision_push_rate;
                                            buffer_write.particles[pid_b][i_target].y +=
                                                delta_vector.y * constants.collision_push_rate;
                                        }
                                    }
                                }
                                None => {
                                    println!("cannot find pb");
                                }
                            }
                        }
                    }
                }
                for ((pid_a, pid_b), crdlink) in links_to_create {
                    create_link(&mut data, pid_a, pid_b, crdlink);
                }
                for pid in particles_to_deactivate {
                    deactivate_particle(&mut data, pid);
                }
                for p in particles_to_create {
                    match activate_particle(&mut data, p.pdid) {
                        Some(pid) => {
                            for i in 0..=1 {
                                {
                                    let mut buffer_write = data.gpu_buffer.write().unwrap();
                                    buffer_write.particles[pid][i].x = p.x;
                                    buffer_write.particles[pid][i].y = p.y;
                                    buffer_write.particles[pid][i].x_before = p.x_before;
                                    buffer_write.particles[pid][i].y_before = p.y_before;
                                }
                                update_grid_coord(&mut data, pid);
                            }
                        }
                        None => {}
                    }
                }
                match SystemTime::now().duration_since(start_time_collision_response) {
                    Ok(t) => durations_collision_response.push(t.as_micros()),
                    Err(_) => println!("error getting elapsed time"),
                }
                //
                tick += 1;
                //
                for _ in configuration.durations_length..durations.len() {
                    durations.remove(0);
                }
                let average_duration = mean_u128(&durations);
                let average_duration_fix_nan = {
                    for _ in configuration.durations_length..durations_fix_nan.len() {
                        durations_fix_nan.remove(0);
                    }
                    mean_u128(&durations_fix_nan)
                };
                for _ in configuration.durations_length..durations_gpu_compute.len() {
                    durations_gpu_compute.remove(0);
                }
                let average_duration_gpu_compute = mean_u128(&durations_gpu_compute);
                for _ in configuration.durations_length..durations_collision_grid.len() {
                    durations_collision_grid.remove(0);
                }
                let average_duration_collision_grid = mean_u128(&durations_collision_grid);
                for _ in configuration.durations_length..durations_collision_response.len() {
                    durations_collision_response.remove(0);
                }
                let average_duration_collision_response = mean_u128(&durations_collision_response);
                for _ in configuration.durations_length..durations_client_data.len() {
                    durations_client_data.remove(0);
                }
                let average_duration_client_data = mean_u128(&durations_client_data);
                //
                // write client data
                //
                let start_time_client_data = SystemTime::now();
                if configuration.update_client_data {
                    let buffer_read = data.gpu_buffer.read().unwrap();
                    let mut particles_client: Vec<ParticleClientData> = Vec::new();
                    let mut total_momentum = Vec2 { x: 0.0, y: 0.0 };
                    let mut absolute_momentum = Vec2 { x: 0.0, y: 0.0 };
                    let mut kinetic_energy = 0.0;
                    for (pid, p) in data.active_particles.iter() {
                        let p_gpu = buffer_read.particles[*pid];
                        particles_client.push(ParticleClientData {
                            x: p_gpu[i_target].x,
                            y: p_gpu[i_target].y,
                            d: p_gpu[i_target].d,
                            a: p_gpu[i_target].is_active == 1,
                            pdid: p.pdid,
                        });
                        total_momentum.x += p_gpu[i_target].momentum_x;
                        total_momentum.y += p_gpu[i_target].momentum_y;
                        absolute_momentum.x += p_gpu[i_target].momentum_x.abs();
                        absolute_momentum.y += p_gpu[i_target].momentum_y.abs();
                        kinetic_energy += p_gpu[i_target].kinetic_energy;
                    }
                    if configuration.serialize_unactive_particles {
                        for pid in &data.inactive_particles {
                            let p_gpu = buffer_read.particles[*pid];
                            particles_client.push(ParticleClientData {
                                x: p_gpu[i_target].x,
                                y: p_gpu[i_target].y,
                                d: p_gpu[i_target].d,
                                a: p_gpu[i_target].is_active == 1,
                                // todo: use Nan for inactive particle
                                // inactive particles can't have a pdid
                                pdid: 0,
                            });
                        }
                    }
                    let client_data = ClientData {
                        constants: constants,
                        particles: particles_client,
                        tick: tick,
                        momentum: total_momentum,
                        kinetic_energy: kinetic_energy,
                        absolute_momentum: absolute_momentum,
                        average_duration: average_duration,
                        particle_definitions: data.particle_definitions.clone(),
                        pdid_to_string: data.pdid_to_string.clone(),
                    };
                    *(client_data_lock_clone.write().unwrap()) =
                        serde_json::to_string(&client_data).unwrap().to_string();
                }
                match SystemTime::now().duration_since(start_time_client_data) {
                    Ok(t) => durations_client_data.push(t.as_micros()),
                    Err(_) => println!("error getting elapsed time"),
                }
                //
                // engine logs
                //
                if configuration.display_engine_logs
                    && tick % configuration.engine_logs_refresh == 0
                {
                    println!("#{}", tick);
                    println!("  active_particles: {}", data.active_particles.len());
                    println!("  links:            {}", data.links.len());
                    println!("  duration");
                    println!("    fix nan:            {}μs", average_duration_fix_nan);
                    println!(
                        "    collision grid:     {}μs",
                        average_duration_collision_grid
                    );
                    println!("    gpu compute:        {}μs", average_duration_gpu_compute);
                    println!(
                        "    collision response: {}μs",
                        average_duration_collision_response
                    );
                    println!("    client data:        {}μs", average_duration_client_data);
                    let sum = average_duration_fix_nan
                        + average_duration_collision_grid
                        + average_duration_gpu_compute
                        + average_duration_collision_response
                        + average_duration_client_data;
                    if sum < average_duration {
                        println!("    other:              {}μs", average_duration - sum);
                    } else {
                        println!("    other:              0μs");
                    }
                    println!("    total:              {}μs", average_duration);
                }
                match SystemTime::now().duration_since(start_time) {
                    Ok(t) => durations.push(t.as_micros()),
                    Err(_) => println!("error getting elapsed time"),
                }
            }
        })
        .unwrap();
    handler.join().unwrap();
}
fn mean_u128(v: &Vec<u128>) -> u128 {
    let sum: u128 = Iterator::sum(v.iter());
    if v.len() > 0 {
        sum / (v.len() as u128)
    } else {
        0
    }
}

/*
let x = (pas.x + pbs.x) * 0.5;
let y = (pas.y + pbs.y) * 0.5;
let dv_a = Vector::new_2(
    pas.x_before,
    pas.y_before,
    pas.x,
    pas.y,
);
let dv_b = Vector::new_2(
    pbs.x_before,
    pbs.y_before,
    pbs.x,
    pbs.y,
);
let mut dv = dv_a.clone();
dv.remove(&dv_b);
let relative_speed = dv.length();
let a_to_b =
    Vector::new_2(pas.x, pas.y, pbs.x, pbs.y);
let normal = a_to_b.get_normal().normalized();
let x0 = x + normal.x * 0.5;
let y0 = y + normal.y * 0.5;
let x1 = x - normal.x * 0.5;
let y1 = y - normal.y * 0.5;
particles_to_create.push(
    ParticleConfigurationVerlet {
        pdid: crdi.pdids[0],
        x: x0,
        y: y0,
        x_before: x0
            - normal.x * relative_speed * 0.5,
        y_before: y0
            - normal.y * relative_speed * 0.5,
    },
);
particles_to_create.push(
    ParticleConfigurationVerlet {
        pdid: crdi.pdids[1],
        x: x1,
        y: y1,
        x_before: x1
            + normal.x * relative_speed * 0.5,
        y_before: y1
            + normal.y * relative_speed * 0.5,
    },
);*/
