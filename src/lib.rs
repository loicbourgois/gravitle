#[macro_use] mod utils;
mod link;
mod intersection;
mod collision;
mod link_intersection;
mod link_to_create;
mod particle;
mod trajectory;
mod wrap_around;
mod point;
mod segment;
mod vector;
mod particles_to_push;

extern crate web_sys;
extern crate wasm_bindgen;

use std::cmp;

use wasm_bindgen::prelude::*;

use link::Link;
use intersection::Intersection;
use link_intersection::LinkIntersection;
use particle::Particle;
use collision::Collision;
use trajectory::Trajectory;
use particle::ParticleCollisionBehavior;
use link_to_create::LinkToCreate;
use wrap_around::WrapAround;
use segment::Segment;
use point::Point;
use vector::Vector;
use particles_to_push::ParticlesToPush;

use std::fmt;
use std::mem;

use line_intersection::{LineInterval};
use geo::{Line};

//
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
//
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

//
// Collision happens when two particles collide
//
enum CollisionBehavior {
    DoNothing,
    DestroyParticles,
    CreateLink,
    MergeParticles,
    PushParticles
}

//
// Implement convertion function for Algorithm
//
impl CollisionBehavior {

    //
    // Convert from String to CollisionBehavior
    //
    fn from_string(value: String) -> CollisionBehavior {
        match value.as_ref() {
            "do-nothing" => CollisionBehavior::DoNothing,
            "destroy-particles" => CollisionBehavior::DestroyParticles,
            "create-link" => CollisionBehavior::CreateLink,
            "merge-particles" => CollisionBehavior::MergeParticles,
            "push-particles" => CollisionBehavior::PushParticles,
            _ => {
                console_error!("Unknown CollisionBehavior : {}", value);
                panic!("Unknown CollisionBehavior : {}", value);
            }
        }
    }

    //
    // Convert CollisionBehavior to String
    //
    fn as_string(&self) -> String {
        match self {
            CollisionBehavior::DoNothing => "do-nothing".to_string(),
            CollisionBehavior::DestroyParticles => "destroy-particles".to_string(),
            CollisionBehavior::CreateLink => "create-link".to_string(),
            CollisionBehavior::MergeParticles => "merge-particles".to_string(),
            CollisionBehavior::PushParticles => "push-particles".to_string()
        }
    }
}

//
// Intersection happens when a moving particle and a link intersect
//
enum IntersectionBehavior {
    DoNothing,
    DestroyParticle,
    DestroyLink
}

//
// IntersectionBehavior
//
impl IntersectionBehavior {

    //
    // Convert from String to IntersectionBehavior
    //
    fn from_string(value: String) -> IntersectionBehavior {
        match value.as_ref() {
            "do-nothing" => IntersectionBehavior::DoNothing,
            "destroy-particle" => IntersectionBehavior::DestroyParticle,
            "destroy-link" => IntersectionBehavior::DestroyLink,
            _ => {
                console_error!("Unknown IntersectionBehavior : {}", value);
                panic!("Unknown IntersectionBehavior : {}", value);
            }
        }
    }
}

//
// Link intersection happens when two links intersect
//
enum LinkIntersectionBehavior {
    DoNothing,
    DestroyLinks
}

//
// IntersectionBehavior
//
impl LinkIntersectionBehavior {

    //
    // Convert from String to IntersectionBehavior
    //
    fn from_string(value: String) -> LinkIntersectionBehavior {
        match value.as_ref() {
            "do-nothing" => LinkIntersectionBehavior::DoNothing,
            "destroy-links" => LinkIntersectionBehavior::DestroyLinks,
            _ => {
                console_error!("Unknown LinkIntersectionBehavior : {}", value);
                panic!("Unknown LinkIntersectionBehavior : {}", value);
            }
        }
    }
}

//
// Wrap around happens when a moving particle goe outside the universe
// and teleports on the other edge.
//
enum WrapAroundBehavior {
    DoNothing,
    DestroyLinks
}

//
// WrapAroundBehavior
//
impl WrapAroundBehavior {

    //
    // Convert from String to IntersectionBehavior
    //
    fn from_string(value: String) -> WrapAroundBehavior {
        match value.as_ref() {
            "do-nothing" => WrapAroundBehavior::DoNothing,
            "destroy-links" => WrapAroundBehavior::DestroyLinks,
            _ => {
                console_error!("Unknown WrapAroundBehavior : {}", value);
                panic!("Unknown WrapAroundBehavior : {}", value);
            }
        }
    }
}

//
// Algorithm used to advance by one step
//
enum Algorithm {
    Euler,
    Verlet
}

//
// Implement convertion function for Algorithm
//
impl Algorithm {

    //
    // Convert from String to Algorithm
    //
    fn from_string(value: String) -> Algorithm {
        match value.as_ref() {
            "euler" => Algorithm::Euler,
            "verlet" => Algorithm::Verlet,
            _ => {
                console_error!("Unknown Algorithm : {}", value);
                panic!("Unknown Algorithm : {}", value);
            }
        }
    }
}

//
// Represents a Universe
// A Universe is made of Particles and Links
//
#[wasm_bindgen]
pub struct Universe {
    width: f64,
    height: f64,
    step: u32,
    delta_time: f64,
    gravitational_constant: f64,
    particles: Vec<Particle>,
    particle_counter: u32,
    algorithm: Algorithm,
    minimal_distance_for_gravity: f64,
    links: Vec<Link>,
    intersections: Vec<Intersection>,
    collisions: Vec<Collision>,
    wrap_arounds: Vec<WrapAround>,
    link_intersections: Vec<LinkIntersection>,
    tick_durations: Vec<f64>,
    tick_average_duration: f64,
    particle_index_to_links_indexes: Vec<Vec<usize>>,
    intersection_behavior: IntersectionBehavior,
    collision_behavior: CollisionBehavior,
    link_intersection_behavior: LinkIntersectionBehavior,
    wrap_around_behavior: WrapAroundBehavior,
    links_to_create: Vec<LinkToCreate>,
    links_to_destroy_indexes: Vec<usize>,
    particles_to_destroy_indexes: Vec<usize>,
    particles_to_disable_indexes: Vec<usize>,
    stabiliser_power: i32,
    stabilise_positions_enabled: bool,
    trajectories: Vec<Trajectory>,
    default_link_length: f64,
    default_link_strengh: f64,
    default_link_thrust_force: f64,
    drag_coefficient: f64,
    wrap_around: bool,
    fixed_clone_count: bool,
    max_link_history: usize,
    particles_to_push: Vec<ParticlesToPush>,
    default_push_force: f64
}

//
// Public methods for Universe, exported to JavaScript
//
#[wasm_bindgen]
impl Universe {

    //
    // Create a new Universe
    //
    pub fn new() -> Universe {
        utils::set_panic_hook();
        Universe {
            width: 10.0,
            height: 10.0,
            step: 0,
            delta_time: 0.01,
            particles: Vec::new(),
            gravitational_constant: 667.4,
            particle_counter: 0,
            algorithm: Algorithm::Verlet,
            minimal_distance_for_gravity: 1.0,
            links: Vec::new(),
            intersections: Vec::new(),
            collisions: Vec::new(),
            link_intersections: Vec::new(),
            wrap_arounds: Vec::new(),
            tick_durations: Vec::new(),
            tick_average_duration: 0.0,
            particle_index_to_links_indexes: Vec::new(),
            intersection_behavior: IntersectionBehavior::DoNothing,
            collision_behavior: CollisionBehavior::DoNothing,
            link_intersection_behavior: LinkIntersectionBehavior::DoNothing,
            wrap_around_behavior: WrapAroundBehavior::DoNothing,
            links_to_create: Vec::new(),
            links_to_destroy_indexes: Vec::new(),
            particles_to_destroy_indexes: Vec::new(),
            particles_to_disable_indexes: Vec::new(),
            stabiliser_power: 10,
            stabilise_positions_enabled: false,
            trajectories: Vec::new(),
            default_link_length: 10.0,
            default_link_strengh: 100.0,
            default_link_thrust_force: 100.0,
            drag_coefficient: 0.5,
            wrap_around: true,
            fixed_clone_count: true,
            max_link_history: 1000,
            particles_to_push: Vec::new(),
            default_push_force: 100.0
        }
    }

    //
    // Load Universe from a JSON String
    //
    pub fn load_from_json(&mut self, json_string: String) {
        let json_parsed = &json::parse(&json_string).unwrap();
        self.minimal_distance_for_gravity =
            json_parsed["minimal_distance_for_gravity"].as_f64().unwrap_or(self.minimal_distance_for_gravity);
        self.width = json_parsed["width"].as_f64().unwrap_or(self.width);
        self.height = json_parsed["height"].as_f64().unwrap_or(self.height);
        self.delta_time = json_parsed["delta_time"].as_f64().unwrap_or(self.delta_time);
        self.stabiliser_power = json_parsed["stabiliser_power"].as_i32().unwrap_or(self.stabiliser_power);
        self.gravitational_constant = json_parsed["gravitational_constant"].as_f64().unwrap_or(self.gravitational_constant);
        self.default_link_length = json_parsed["default_link_length"].as_f64().unwrap_or(self.default_link_length);
        self.default_link_strengh = json_parsed["default_link_strengh"].as_f64().unwrap_or(self.default_link_strengh);
        self.default_link_thrust_force = json_parsed["default_link_thrust_force"].as_f64().unwrap_or(self.default_link_thrust_force);
        self.drag_coefficient = json_parsed["drag_coefficient"].as_f64().unwrap_or(self.drag_coefficient);
        self.wrap_around = json_parsed["wrap_around"].as_bool().unwrap_or(self.wrap_around);
        self.fixed_clone_count = json_parsed["fixed_clone_count"].as_bool().unwrap_or(self.fixed_clone_count);
        self.set_default_push_force(json_parsed["default_push_force"].as_f64().unwrap_or(self.default_push_force));
        self.stabilise_positions_enabled = json_parsed["stabilise_positions_enabled"]
            .as_bool()
            .unwrap_or(self.stabilise_positions_enabled);
        self.set_algorithm_from_string(json_parsed["algorithm"].to_string());
        self.set_collision_behavior_from_string(json_parsed["collision_behavior"].to_string());
        self.set_intersection_behavior_from_string(json_parsed["intersection_behavior"].to_string());
        self.set_link_intersection_behavior_from_string(json_parsed["link_intersection_behavior"].to_string());
        self.set_wrap_around_behavior_from_string(json_parsed["wrap_around_behavior"].to_string());
        self.set_particles_json((&json_parsed["particles"]).to_string());
        self.set_links_json((&json_parsed["links"]).to_string());
    }

    //
    // Advance the Universe by one step
    //
    pub fn tick(&mut self) {
        //
        // Setup permormance analysis
        //
        let t1 = Universe::now();
        //
        // Perform actual computations
        //
        self.particles_to_push.clear();
        self.particles_to_destroy_indexes.clear();
        self.particles_to_disable_indexes.clear();
        self.links_to_destroy_indexes.clear();
        self.links_to_create.clear();
        self.wrap_arounds.clear();
        
        self.add_gravity_forces();
        self.add_link_forces();
        self.add_links_thrust_forces();
        self.add_drag_forces();
        self.update_acceleration();
        self.reset_forces();
        match self.algorithm {
            Algorithm::Euler => {
                self.update_speed_euler(self.get_delta_time());
                self.update_position_euler(self.get_delta_time());
            },
            Algorithm::Verlet => {
                self.update_position_verlet(self.get_delta_time());
                self.update_speed_verlet(self.get_delta_time());
            }
        }
        self.compute_tick_collisions();
        self.treat_collisions();
        self.treat_particles_to_push();
        if self.wrap_around {
            self.compute_wrap_arounds();
            self.treat_wrap_arounds();
        } else {
            // Do nothing
        }
        if self.stabilise_positions_enabled {
            self.stabilise_positions();
        } else {
            // Do nothing
        }
        self.update_momentum();
        self.update_trajectories();
        self.update_links_coordinates();
        self.update_links_coordinates_cycled();
        self.compute_tick_intersections();
        self.compute_tick_link_intersections();
        self.treat_intersections();
        self.treat_link_intersections();
        self.create_links();
        self.disable_particles();
        self.destroy_particles();
        self.destroy_links();
        self.update_links_states();
        self.step += 1;
        //
        // Permormance analysis
        //
        let t2 = Universe::now();
        let duration = t2 - t1;
        self.tick_durations.push(duration);
        if self.tick_durations.len() > (1.0 / self.delta_time) as usize {
            self.tick_durations.drain(0..1);
        }
        let mut tick_average_duration = 0.0;
        for duration in self.tick_durations.iter() {
            tick_average_duration += duration;
        }
        tick_average_duration /= self.tick_durations.len() as f64;
        self.tick_average_duration = tick_average_duration;
    }

    //
    // Reset the Universe
    //
    pub fn reset(&mut self) {
        self.step = 0;
        self.reset_particles();
        self.reset_links();
    }

    //
    // Reset particles and set them from json
    //
    pub fn set_particles_json(&mut self, json_string: String ) {
        self.reset_particles();
        let particles_data = &json::parse(&json_string).unwrap();
        for i in 0..particles_data.len() {
            let particle_json_string = & particles_data[i];
            self.add_particle_json(particle_json_string.to_string());
        }
    }

    //
    // Reset links and set them from json
    //
    pub fn set_links_json(&mut self, json_string: String ) {
        self.reset_links();
        let json_parsed = &json::parse(&json_string).unwrap();
        for i in 0..json_parsed.len() {
            let p1_id = json_parsed[i]["p1_index"].as_usize().unwrap();
            let p2_id = json_parsed[i]["p2_index"].as_usize().unwrap();
            self.add_link(p1_id, p2_id);
            let last_index = self.links.len() - 1;
            let json_string = json_parsed[i].to_string();
            self.links[last_index].load_from_json(json_string);
        }
    }

    //
    // Add a new particle and initialise it with the given JSON
    //
    pub fn add_particle_json(&mut self, json_string: String) {
        self.add_particle();
        let last_index = self.particles.len() - 1;
        self.particles[last_index].load_from_json(json_string);
    }

    //
    // Setter for algorithm
    //
    pub fn set_algorithm_from_string(&mut self, algorithm_string: String) {
        if algorithm_string != "null" {
            self.algorithm = Algorithm::from_string(algorithm_string);
        } else {
            // Do nothing
        }
    }

    //
    // Setter for collision_behavior
    //
    pub fn set_collision_behavior_from_string(&mut self, collision_behavior_string: String) {
        if collision_behavior_string != "null" {
            self.collision_behavior = CollisionBehavior::from_string(collision_behavior_string);
        } else {
            // Do nothing
        }
    }

    //
    // Setter for intersection_behavior
    //
    pub fn set_intersection_behavior_from_string(&mut self, intersection_behavior_string: String) {
        if intersection_behavior_string != "null" {
            self.intersection_behavior = IntersectionBehavior::from_string(intersection_behavior_string);
        } else {
            // Do nothing
        }
    }

    //
    // Setter for link_intersection_behavior
    //
    pub fn set_link_intersection_behavior_from_string(&mut self, link_intersection_behavior_string: String) {
        if link_intersection_behavior_string != "null" {
            self.link_intersection_behavior = LinkIntersectionBehavior::from_string(
                link_intersection_behavior_string
            );
        } else {
            // Do nothing
        }
    }

    //
    // Setter for wrap_around_behavior
    //
    pub fn set_wrap_around_behavior_from_string(&mut self, wrap_around_behavior_string: String) {
        if wrap_around_behavior_string != "null" {
            self.wrap_around_behavior = WrapAroundBehavior::from_string(
                wrap_around_behavior_string
            );
        } else {
            // Do nothing
        }
    }

    //
    // Setter for wrap_around
    //
    pub fn set_wrap_around(&mut self, wrap_around: bool) {
        self.wrap_around = wrap_around;
    }

    //
    // Setter for minimal_distance_for_gravity
    //
    pub fn set_minimal_distance_for_gravity(&mut self, minimal_distance_for_gravity: f64) {
        self.minimal_distance_for_gravity = minimal_distance_for_gravity;
    }

    //
    // Setter for default_link_length
    //
    pub fn set_default_link_length(&mut self, default_link_length: f64) {
        self.default_link_length = default_link_length;
    }

    //
    // Setter for default_link_strengh
    //
    pub fn set_default_link_strengh(&mut self, default_link_strengh: f64) {
        self.default_link_strengh = default_link_strengh;
    }

    //
    // Setter for default_link_thrust_force
    //
    pub fn set_default_link_thrust_force(&mut self, default_link_thrust_force: f64) {
        self.default_link_thrust_force = default_link_thrust_force;
    }

    //
    // Setter for drag_coefficient
    //
    pub fn set_drag_coefficient(&mut self, drag_coefficient: f64) {
        self.drag_coefficient = drag_coefficient;
    }

    //
    // Setter for stabilise_positions_enabled
    //
    pub fn set_stabilise_positions_enabled(&mut self, stabilise_positions_enabled: bool) {
        self.stabilise_positions_enabled = stabilise_positions_enabled;
    }

    //
    // Setter for stabiliser_power
    //
    pub fn set_stabiliser_power(&mut self, stabiliser_power: i32) {
        self.stabiliser_power = stabiliser_power;
    }

    //
    // Setter for gravitational_constant
    //
    pub fn set_gravitational_constant(&mut self, gravitational_constant: f64) {
        self.gravitational_constant = gravitational_constant;
    }

    //
    // Setter for default_push_force
    //
    pub fn set_default_push_force(&mut self, default_push_force: f64) {
        self.default_push_force = default_push_force
    }

    //
    // Setter for gravitational_constant
    //
    pub fn set_width(&mut self, width: f64) {
        self.width = width;
    }

    //
    // Setter for gravitational_constant
    //
    pub fn set_height(&mut self, height: f64) {
        self.height = height;
    }

    //
    // Setter for delta_time
    //
    pub fn set_delta_time(&mut self, delta_time: f64) {
        self.delta_time = delta_time;
    }

    //
    // Setter for fixed_clone_count
    //
    pub fn set_fixed_clone_count(&mut self, fixed_clone_count: bool) {
        self.fixed_clone_count = fixed_clone_count;
    }

    //
    // Returns a String representation of the Universe
    //
    pub fn get_infos(&self) -> String {
        self.to_string()
    }

    //
    // Get delta_time for the Universe in milliseconds
    //
    pub fn get_delta_time_milli(&self) -> f64 {
        self.delta_time * 1000.0
    }

    //
    // Get delta_time for the Universe in seconds
    //
    pub fn get_delta_time(&self) -> f64 {
        self.delta_time
    }

    //
    // Return a pointer to the Vec of Particles
    //
    pub fn get_particles(&self) -> *const Particle {
        self.particles.as_ptr()
    }

    //
    // Return number of Particles in the Universe
    //
    pub fn get_particles_count(&self) -> usize {
        self.particles.len()
    }

    //
    // Return a pointer to the Vec of Links
    //
    pub fn get_links(&self) -> *const Link {
        self.links.as_ptr()
    }

    //
    // Return number of Links in the Universe
    //
    pub fn get_links_count(&self) -> usize {
        self.links.len()
    }

    //
    // Returns width of the Universe
    //
    pub fn get_width(&self) -> f64 {
        self.width
    }

    //
    // Returns height of the Universe
    //
    pub fn get_height(&self) -> f64 {
        self.height
    }

    //
    // Returns a pointer to intersections
    //
    pub fn get_intersections(&self) -> *const Intersection {
        self.intersections.as_ptr()
    }

    //
    // Returns the number of intersections
    //
    pub fn get_intersections_count(&self) -> usize {
        self.intersections.len()
    }

    //
    // Return the size of an Intersection
    //
    pub fn get_intersection_size(&self) -> usize {
        mem::size_of::<Intersection>()
    }

    //
    // Return a specific intersection
    //
    pub fn get_intersection(&self, id: usize) -> Intersection {
        self.intersections[id]
    }

    //
    // Returns the number of trajectories
    //
    pub fn get_trajectories_count(&self) -> usize {
        self.trajectories.len()
    }

    //
    // Return a specific trajectory
    //
    pub fn get_trajectory(&self, id: usize) -> Trajectory {
        self.trajectories[id].clone()
    }

    //
    // Getter for trajectories
    //
    pub fn get_trajectories_position_at_period(&self, period: usize) -> Vec<f64> {
        let mut positions = Vec::new();
        for trajectory in self.trajectories.iter() {
            let mut trajectory_positions = trajectory.get_positions_at_period_as_f64s(period);
            positions.append(&mut trajectory_positions);
        }
        positions
    }

    //
    // Getter for number of particle getting disabled in the tick
    //
    pub fn get_particles_to_disable_indexes_length(&self) -> usize {
        self.particles_to_disable_indexes.len()
    }

    //
    // Get a grid representing the gravitational forces exerced
    // throughout the Universe
    //
    pub fn get_gravitational_grid(&self, width: usize, height: usize) -> Vec<f64> {
        let mass = 1.0;
        let mut grid = vec![0.0; width*height];
        let w64 = width as f64;
        let h64 = height as f64;
        for i in 0..width {
            let x = (self.width / w64 ) * i as f64
                + (self.width / w64 ) / 2.0 - self.width / 2.0;
            for j in 0..height {
                let y = (self.height / h64 ) * j as f64
                    + (self.height / h64 ) / 2.0 - self.height / 2.0;
                grid[i*width + j] =
                    self.get_absolute_gravitational_field_at(x, y, mass);
            }
        }
        grid
    }

    //
    // Get a grid representing the gravitational forces exerced
    // throughout the Universe.
    // Values are squared and normalized for smoother feel.
    //
    pub fn get_gravitational_grid_squared_normalized(
        &self,
        width: usize,
        height: usize
    ) -> Vec<f64> {
        let mut grid = self.get_gravitational_grid(width, height);
        let mut max = -9999.0;
        let mut min = 9999.0;
        for i in 0..grid.len() {
            grid[i] = grid[i].sqrt();
            if grid[i] < min {
                min = grid[i];
            } else {
                // Do nothing
            }
            if grid[i] > max {
                max = grid[i];
            } else {
                // Do nothing
            }
        }
        for i in 0..grid.len() {
            grid[i] =(grid[i] - min) / (max-min);
        }
        grid
    }

    //
    // Get links coordinates used for drawing.
    // Different from real in Universe coordinates.
    // Useful in case of links wraping around the edge of the Universe.
    //
    pub fn get_links_coordinates_to_draw(&self) -> Vec<f64> {
        let mut coordinates = Vec::new();
        if self.wrap_around {
            for link in self.links.iter() {
                if link.is_enabled() {
                    let cycle_deltas = Particle::get_cycle_deltas(
                        self.particles[link.get_p1_index()],
                        self.particles[link.get_p2_index()]
                    );
                    if cycle_deltas[0] == 0 && cycle_deltas[1] == 0 {
                        coordinates.extend_from_slice(& link.get_coordinates());
                    } else {
                        coordinates.extend_from_slice(& link.get_coordinates_cycled());
                    }
                } else {
                    // Do nothing
                }
            }
        } else {
            for link in self.links.iter() {
                if link.is_enabled() {
                    coordinates.extend_from_slice(&link.get_coordinates());
                } else {
                    // Do nothing
                }
            }
        }
        coordinates
    }

    //
    // Get links' coordinates for links whose thrusting is activated.
    // Used for drawing.
    // Different from real in Universe coordinates.
    // Useful in case of links wraping around the edge of the Universe.
    //
    pub fn get_thrusting_links_coordinates_to_draw(&self) -> Vec<f64> {
        let mut coordinates = Vec::new();
        if self.wrap_around {
            for link in self.links.iter() {
                if link.is_enabled() && link.is_thrust_activated() {
                    let cycle_deltas = Particle::get_cycle_deltas(
                        self.particles[link.get_p1_index()],
                        self.particles[link.get_p2_index()]
                    );
                    if cycle_deltas[0] == 0 && cycle_deltas[1] == 0 {
                        coordinates.extend_from_slice(& link.get_coordinates());
                    } else {
                        coordinates.extend_from_slice(& link.get_coordinates_cycled());
                    }
                } else {
                    // Do nothing
                }
            }
        } else {
            for link in self.links.iter() {
                if link.is_enabled() && link.is_thrust_activated() {
                    coordinates.extend_from_slice(&link.get_coordinates());
                } else {
                    // Do nothing
                }
            }
        }
        coordinates
    }

    //
    // Returns links states as Vector of f64
    //
    // Data is structured as follow :
    //  x1 at step n
    //  y1 at step n
    //  x2 at step n
    //  y2 at step n
    //  x1 at step (n - 1)
    //  y1 at step (n - 1)
    //  x2 at step (n - 1)
    //  y2 at step (n - 1)
    //  is_thrusting, as a positive or negative f64
    //  normalized_age, as a f64 in range [0.0, 1.0],
    //      where 0.0 correspond to the current step
    //      and 1.0 correspond to the history length
    //
    pub fn get_links_states (
        &self,
        history_length: usize,
        period: usize
    ) -> Vec<f64> {
        let mut data = Vec::new();
        for link in self.links.iter() {
            let history = link.get_states(history_length, period);
            let mut age = 0.0;
            let mut i = 0;
            let len = cmp::min(history.len(), history.len() - 1);
            while i < len {
                let state = history[i];
                let state_2 = history[i+1];
                let normalized_age = age / history_length as f64;
                let data_tmp = [
                    state.cycled_coordinates[0].x1,
                    state.cycled_coordinates[0].y1,
                    state.cycled_coordinates[0].x2,
                    state.cycled_coordinates[0].y2,
                    state_2.cycled_coordinates[0].x1,
                    state_2.cycled_coordinates[0].y1,
                    state_2.cycled_coordinates[0].x2,
                    state_2.cycled_coordinates[0].y2,
                    if state.thrust_activated { 1.0 } else {0.0},
                    normalized_age,
                    state.cycled_coordinates[1].x1,
                    state.cycled_coordinates[1].y1,
                    state.cycled_coordinates[1].x2,
                    state.cycled_coordinates[1].y2,
                    state_2.cycled_coordinates[1].x1,
                    state_2.cycled_coordinates[1].y1,
                    state_2.cycled_coordinates[1].x2,
                    state_2.cycled_coordinates[1].y2,
                    if state.thrust_activated { 1.0 } else {-1.0},
                    normalized_age
                ];
                data.extend_from_slice(&data_tmp);
                age = age + period as f64;
                i += 1;
            }
        }
        data
    }

    //
    // Return particles coordinates.
    //
    pub fn get_particles_data_to_draw(&self) -> Vec<f64> {
        let mut data = Vec::new();
        for particle in self.particles.iter() {
            if particle.is_enabled() {
                data.extend_from_slice(&particle.get_coordinates());
                data.push(particle.get_radius());
            } else {
                // Do nothing
            }
        }
        data
    }

    //
    // Activate thrust for the given links
    //
    pub fn activate_thrust_for_links(&mut self, link_indexes: Vec<usize>) {
        for link_index in link_indexes {
            if link_index < self.links.len() {
                self.links[link_index].activate_thrust();
            } else {
                console_warning!("Cannot activate thrust : link #{} doesn't exist.", link_index);
            }
        }
    }

    //
    // Deactivate thrust for the given links
    //
    pub fn deactivate_thrust_for_links(&mut self, link_indexes: Vec<usize>) {
        for link_index in link_indexes {
            if link_index < self.links.len() {
                self.links[link_index].deactivate_thrust();
            } else {
                console_warning!("Cannot deactivate thrust : link #{} doesn't exist.", link_index);
            }
        }
    }

    //
    // Getter for step
    //
    pub fn get_step(&self) -> u32 {
        self.step
    }

    //
    // Given two particle indexes, try to find the corresponding link index.
    // Returns None if no link could be found.
    //
    pub fn get_link_index_from_particles_indexes(&self, particle_index_1: usize, particle_index_2: usize) -> Option<usize> {
        for (link_index, link) in self.links.iter().enumerate() {
            if link.has_particle(particle_index_1) && link.has_particle(particle_index_2) {
                return Some(link_index);
            } else {
                // Do nothing
            }
        }
        return None;
    }

    //
    // Get both x and y coordinates of all particles in a single Vec.
    //
    pub fn get_particle_coordinates(&self) -> Vec<f64> {
        let mut coordinates = vec![0.0; self.particles.len() * 2];
        for (particle_index, particle) in self.particles.iter().enumerate() {
            let i = particle_index * 2;
            coordinates[i] = particle.get_x();
            coordinates[i+1] = particle.get_y();
        }
        coordinates
    }

    //
    // Get all 4 coordinates for a link given its index
    //
    pub fn get_link_coordinates_for_link(&self, link_index: usize) -> Vec<f64> {
        self.links[link_index].get_coordinates().to_vec()
    }
}

//
// Private methods
//
impl Universe {

    //
    // Update speed using Euler method
    //
    fn update_speed_euler(&mut self, delta_time: f64) {
        for particle in &mut self.particles {
            if particle.is_enabled() {
                particle.update_speed_euler(delta_time);
            } else {
                // Do nothing
            }
        }
    }

    //
    // Update speed for Verlet mode
    //
    fn update_speed_verlet(&mut self, delta_time: f64) {
        for particle in &mut self.particles {
            if particle.is_enabled() {
                particle.update_speed_verlet(delta_time);
            } else {
                // Do nothing
            }
        }
    }

    //
    // Update position using Euler method
    //
    // See Wikipedia for explanation
    // https://en.wikipedia.org/wiki/Euler_method
    //
    fn update_position_euler(&mut self, delta_time: f64) {
        for particle in &mut self.particles {
            if particle.is_enabled() {
                particle.update_position_euler(delta_time);
            } else {
                // Do nothing
            }
        }
    }

    //
    // Update position using Verlet Integration
    //
    // See wikipedia for explanation
    // https://en.wikipedia.org/wiki/Verlet_integration#Verlet_integration_(without_velocities)
    //
    fn update_position_verlet(&mut self, delta_time: f64) {
        for particle in &mut self.particles {
            if particle.is_enabled() {
                particle.update_position_verlet(delta_time);
            } else {
                // Do nothing
            }
        }
    }

    //
    // Reset forces for the tick
    //
    fn reset_forces(&mut self) {
        for particle in &mut self.particles {
            if particle.is_enabled() {
                particle.reset_forces();
            } else {
                // Do nothing
            }
        }
    }

    //
    // Add gravity forces
    //
    fn add_gravity_forces (&mut self) {
        if self.wrap_around {
            self.add_gravity_forces_wrap_around();
        } else {
            self.add_gravity_forces_no_wrap_around();
        }
    }

    //
    // Add gravity forces when wrap_around is disabled
    //
    fn add_gravity_forces_no_wrap_around (&mut self) {
        let particles = self.particles.clone();
        for particle in &mut self.particles {
            if particle.is_enabled() {
                particle.add_gravity_forces(
                    & particles,
                    self.gravitational_constant,
                    self.minimal_distance_for_gravity
                );
            } else {
                // Do nothing
            }
        }
    }

    //
    // Add gravity forces when wrap_around is enabled
    //
    fn add_gravity_forces_wrap_around (&mut self) {
        let particles = self.particles.clone();
        for particle in &mut self.particles {
            if particle.is_enabled() {
                if self.fixed_clone_count {
                    particle.add_gravity_forces_wrap_around_fixed_clone_count(
                        & particles,
                        self.gravitational_constant,
                        self.width,
                        self.height,
                        self.minimal_distance_for_gravity
                    );
                } else {
                    particle.add_gravity_forces_wrap_around_variable_clone_count(
                        & particles,
                        self.gravitational_constant,
                        self.width,
                        self.height,
                        self.minimal_distance_for_gravity
                    );
                }
            } else {
                // Do nothing
            }
        }
    }

    //
    // Add forces from links
    //
    // Uses Hooke's law
    // See https://en.wikipedia.org/wiki/Hooke's_law#Formal_definition
    //
    fn add_link_forces(&mut self) {
        for link in self.links.iter() {
            if link.is_enabled() {
                let p1_index = link.get_p1_index();
                let p2_index = link.get_p2_index();
                let p1 = & self.particles[p1_index];
                let p2 = & self.particles[p2_index];
                if p1.is_enabled() && p2.is_enabled() {
                    let forces = if self.wrap_around {
                        Particle::get_link_forces_wrap_around_enabled(p1, p2, link, self.width, self.height)
                    } else {
                        Particle::get_link_forces_wrap_around_disabled(p1, p2, link)
                    };
                    let force_x = forces.x * 0.5;
                    let force_y = forces.y * 0.5;
                    self.particles[p1_index].add_force(Vector{x: force_x, y: force_y});
                    self.particles[p2_index].add_force(Vector{x:-force_x, y: -force_y});
                } else {
                    // Do nothing
                }
            } else {
                // Do nothing
            }
        }
    }

    //
    // Add thrust forces generated by links to particles
    //
    fn add_links_thrust_forces(&mut self) {
        for link in self.links.iter() {
            if link.is_enabled() && link.is_thrust_activated() {
                let p1_index = link.get_p1_index();
                let p2_index = link.get_p2_index();
                let coordinates_cycled = link.get_coordinates_cycled();
                let force_option = link.get_thrust_forces(coordinates_cycled);
                match force_option {
                    Some(force) => {
                        self.particles[p1_index].add_force(force);
                        self.particles[p2_index].add_force(force);
                    },
                    None => {
                        // Do nothing
                    }
                }
            } else {
                // Do nothing
            }
        }
    }

    //
    // Add drag forces
    //
    // Uses a simplified version of the Drag equation
    // Only takes into account a drag coefficient and the velocity of particles
    //
    // See https://en.wikipedia.org/wiki/Drag_equation
    //
    fn add_drag_forces(&mut self) {
        for particle in &mut self.particles {
            if particle.is_enabled() {
                particle.add_drag_forces(self.drag_coefficient);
            } else {
                // Do nothing
            }
        }
    }

    //
    // Update acceleration
    //
    fn update_acceleration(&mut self) {
        for particle in &mut self.particles {
            if particle.is_enabled() {
                particle.update_acceleration();
            } else {
                // Do nothing
            }
        }
    }

    //
    // Recenter particles if they got outside the Universe
    // and wrap_around is enabled
    //
    fn compute_wrap_arounds(&mut self) {
        let x_max = self.width * 0.5;
        let x_min = -x_max;
        let y_max = self.height * 0.5;
        let y_min = -y_max;
        for (particle_index, particle) in self.particles.iter().enumerate() {
            let point = particle.get_coordinates_as_point();
            if particle.is_enabled() {
                if point.x < x_min || point.x > x_max
                        || point.y < y_min || point.y > y_max {
                    self.wrap_arounds.push(WrapAround::new(particle_index));
                } else {
                    // Do nothing
                }
            } else {
                // Do nothing
            }
        }
    }

    //
    // Treat wrap arounds
    //
    fn treat_wrap_arounds(& mut self) {
        for wrap_around in self.wrap_arounds.iter() {
            let particle_index = wrap_around.get_particle_index();
            self.particles[particle_index].wrap_around(
                self.width,
                self.height
            );
            match & self.wrap_around_behavior {
                WrapAroundBehavior::DoNothing => {
                    // Do nothing
                },
                WrapAroundBehavior::DestroyLinks => {
                    let mut links_to_destroy_indexes = self.particle_index_to_links_indexes[particle_index].clone();
                    self.links_to_destroy_indexes.append(&mut links_to_destroy_indexes);
                }
            }
        }
    }

    //
    // Stabilize positions by removing last decimals for each coordinate
    // Useful for conserving symetries for a longer period
    //
    fn stabilise_positions(&mut self) {
        let stabiliser = (10.0_f64).powi(self.stabiliser_power);
        for particle in &mut self.particles {
            particle.stabilise_position(stabiliser);
        }
    }

    //
    // Update each particle's momentum
    //
    fn update_momentum(&mut self) {
        for particle in &mut self.particles {
            particle.update_momentum();
        }
    }

    //
    // Update trajectories
    //
    fn update_trajectories(&mut self) {
        let step = self.step;
        for index in 0..self.trajectories.len() {
            if self.particles[index].is_fixed() || !self.particles[index].is_enabled() {
                // Do nothing
            } else {
                let coordinates = self.particles[index].get_coordinates_as_point();
                self.trajectories[index].add_position(
                    coordinates.x,
                    coordinates.y,
                    step
                );
            }
        }
    }

    //
    // Update links coordinates by coppying particles coordinates to links
    //
    fn update_links_coordinates(&mut self) {
        for link in &mut self.links {
            let p1_coordinates = self.particles[link.get_p1_index()].get_coordinates_as_point();
            let p2_coordinates = self.particles[link.get_p2_index()].get_coordinates_as_point();
            link.set_coordinates(
                p1_coordinates.x,
                p1_coordinates.y,
                p2_coordinates.x,
                p2_coordinates.y
            );
        }
    }

    //
    // Update links coordinates cycled
    //
    fn update_links_coordinates_cycled(&mut self) {
        for link in &mut self.links {
            let p1_index = link.get_p1_index();
            let p2_index = link.get_p2_index();
            let cycle_deltas = Particle::get_cycle_deltas(
                self.particles[p1_index],
                self.particles[p2_index]
            );
            let dx = (cycle_deltas[0] - link.get_initial_cycle_delta_x()) as f64 * self.width;
            let dy = (cycle_deltas[1] - link.get_initial_cycle_delta_y()) as f64 * self.height;
            let coordinates = link.get_coordinates();
            let coordinates_cycled = [
                coordinates[0], coordinates[1],
                coordinates[2] + dx, coordinates[3] + dy,
                coordinates[0] - dx, coordinates[1] - dy,
                coordinates[2], coordinates[3]
            ];
            link.set_coordinates_cycled(coordinates_cycled);
        }
    }

    //
    // List intersections happening during the current tick
    //
    fn compute_tick_intersections(& mut self) {
        self.intersections.clear();
        for (link_index, link) in self.links.iter().enumerate() {
            for (particle_index, particle) in self.particles.iter().enumerate() {
                if self.links[link_index].has_particle(particle_index)
                    || particle.is_enabled() == false
                    || self.links[link_index].is_enabled() == false
                {
                    // Do nothing
                } else {
                    let mut intersection_found = false;
                    let link_segments = link.get_cycled_coordinates_as_segments();
                    for link_segment in link_segments.iter() {
                        match Universe::approximate_segment_circle_intersection(
                            link_segment,
                            particle.get_coordinates_as_point(),
                            particle.get_radius()
                        ) {
                            Some(intersection_point) => {
                                self.intersections.push(
                                    Intersection::new(
                                        intersection_point.x,
                                        intersection_point.y,
                                        link_index,
                                        particle_index
                                    )
                                );
                                intersection_found = true;
                                break;
                            },
                            None => {
                                // Do nothing
                            }
                        }
                    }
                    if intersection_found == false {
                        for link_segment in link_segments.iter() {
                            match Universe::get_segments_intersection(
                                & particle.get_coordinates_as_segment(),
                                link_segment
                            ) {
                                Some(intersection_point) => {
                                    self.intersections.push(
                                        Intersection::new(
                                            intersection_point.x,
                                            intersection_point.y,
                                            link_index,
                                            particle_index
                                        )
                                    );
                                },
                                None => {
                                    // NTD
                                }
                            }
                        }
                    } else {
                        // Do nothing
                    }
                }
            }
        }
    }

    //
    // List collisions happening during the current tick
    // Uses a posteriori approach.
    //
    // TODO : implement a continuous a priori approach to collision detection
    // source : https://ericleong.me/research/circle-circle/#dynamic-static-circle-collision-response
    //
    fn compute_tick_collisions(& mut self) {
        self.collisions.clear();
        for (i, particle_1) in self.particles.iter().enumerate() {
            for (j, particle_2) in self.particles.iter().enumerate() {
                if i < j {
                    let particles_collide = Particle::particles_collide(particle_1, particle_2);
                    if particles_collide {
                        self.collisions.push(Collision::new(i, j));
                    } else {
                        // NTD
                    }
                } else {
                    // Do nothing
                }
            }
        }
    }

    //
    // List link intersections happening during the current tick
    //
    fn compute_tick_link_intersections (& mut self) {
        self.link_intersections.clear();
        for (i, link_1) in self.links.iter().enumerate() {
            for (j, link_2) in self.links.iter().enumerate() {
                if j > i {
                    if link_1.is_enabled() && link_2.is_enabled() {
                        let p1_1_index = link_1.get_p1_index();
                        let p1_2_index = link_1.get_p2_index();
                        let p2_1_index = link_2.get_p1_index();
                        let p2_2_index = link_2.get_p2_index();
                        if p1_1_index != p2_1_index
                                && p1_1_index != p2_2_index
                                && p1_2_index != p2_1_index
                                && p1_2_index != p2_2_index {
                            let link_1_segments = link_1.get_cycled_coordinates_as_segments();
                            let link_2_segments = link_2.get_cycled_coordinates_as_segments();
                            let links_intersection_options = [
                                Universe::get_segments_intersection(
                                    & link_1_segments[0],
                                    & link_2_segments[0]
                                ),
                                Universe::get_segments_intersection(
                                    & link_1_segments[0],
                                    & link_2_segments[1]
                                ),
                                Universe::get_segments_intersection(
                                    & link_1_segments[1],
                                    & link_2_segments[0]
                                ),
                                Universe::get_segments_intersection(
                                    & link_1_segments[1],
                                    & link_2_segments[1]
                                ),
                            ];
                            for links_intersection_option in links_intersection_options.iter() {
                                match links_intersection_option {
                                    Some(links_intersection) => {
                                        self.link_intersections.push(LinkIntersection::new(
                                            links_intersection.x, links_intersection.y, i, j
                                        ));
                                    },
                                    None => {
                                        // Do nothing
                                    }
                                }
                            }
                        } else {
                            // Do nothing
                        }
                    } else {
                        // Do nothing
                    }
                } else {
                    // Do nothing
                }
            }
        }
    }

    //
    // Treat detected intersections using the specified intersection behavior
    //
    fn treat_intersections(& mut self) {
        match & self.intersection_behavior {
            IntersectionBehavior::DoNothing => {
                // NTD
            },
            IntersectionBehavior::DestroyLink => {
                // List links to destroy
                for intersection in self.intersections.iter() {
                    self.links_to_destroy_indexes.push((*intersection).get_link_index());
                }
            },
            IntersectionBehavior::DestroyParticle => {
                for intersection in self.intersections.iter() {
                    let particle_index = (*intersection).get_particle_index();
                    self.particles_to_destroy_indexes.push(particle_index);
                    let mut links_to_destroy_indexes = self.particle_index_to_links_indexes[particle_index].clone();
                    self.links_to_destroy_indexes.append(&mut links_to_destroy_indexes);
                }
            }
        }
    }

    //
    // Treat detected collisions using the specified collision behavior
    //
    fn treat_collisions(& mut self) {
        for collision in self.collisions.iter() {
            let p1_index = (*collision).get_particle_1_index();
            let p2_index = (*collision).get_particle_2_index();
            let p1_collision_behavior = self.particles[p1_index].get_collision_behavior();
            let p2_collision_behavior = self.particles[p2_index].get_collision_behavior();
            match (&self.collision_behavior, p1_collision_behavior, p2_collision_behavior) {
                (CollisionBehavior::DoNothing, ParticleCollisionBehavior::DoNothing, ParticleCollisionBehavior::DoNothing) => {
                    // NTD
                },
                (CollisionBehavior::DoNothing, ParticleCollisionBehavior::DoNothing, ParticleCollisionBehavior::DisableSelf) => {
                    self.particles_to_disable_indexes.push(p2_index);
                },
                (CollisionBehavior::DoNothing, ParticleCollisionBehavior::DoNothing, ParticleCollisionBehavior::DestroySelf) => {
                    self.particles_to_destroy_indexes.push(p2_index);
                },
                (CollisionBehavior::DoNothing, ParticleCollisionBehavior::DisableSelf, ParticleCollisionBehavior::DoNothing) => {
                    self.particles_to_disable_indexes.push(p1_index);
                },
                (CollisionBehavior::DoNothing, ParticleCollisionBehavior::DisableSelf, ParticleCollisionBehavior::DisableSelf) => {
                    self.particles_to_disable_indexes.push(p1_index);
                    self.particles_to_disable_indexes.push(p2_index);
                },
                (CollisionBehavior::DestroyParticles, ParticleCollisionBehavior::DoNothing, ParticleCollisionBehavior::DoNothing) => {
                    self.particles_to_destroy_indexes.push(p1_index);
                    self.particles_to_destroy_indexes.push(p2_index);
                },
                (CollisionBehavior::CreateLink, ParticleCollisionBehavior::DoNothing, ParticleCollisionBehavior::DoNothing) => {
                    self.links_to_create.push(LinkToCreate::new(p1_index, p2_index));
                },
                (CollisionBehavior::PushParticles, ParticleCollisionBehavior::DoNothing, ParticleCollisionBehavior::DoNothing) => {
                    let collision_point_option = Particle::particles_collision_point(
                        &self.particles[p1_index],
                        &self.particles[p2_index]
                    );
                    match collision_point_option {
                        Some(collision_point) => {
                            self.particles_to_push.push(ParticlesToPush {
                                particle_1_index: p1_index,
                                particle_2_index: p2_index,
                                collision_point: collision_point
                            });
                        },
                        None => {
                            // Do nothing
                        }
                    }
                },
                (collision_behavior, p1_collision_behavior, p2_collision_behavior) => {
                    console_error!("Treat collision for ({}, {}, {}) not implemented",
                        collision_behavior.as_string(),
                        p1_collision_behavior.as_string(),
                        p2_collision_behavior.as_string()
                    );
                }
            }
        }
    }

    //
    // Treat detected link intersections using the specified behavior
    //
    fn treat_link_intersections(&mut self) {
        for link_intersection in self.link_intersections.iter() {
            match &self.link_intersection_behavior {
                LinkIntersectionBehavior::DoNothing => {
                    // Do nothing
                },
                LinkIntersectionBehavior::DestroyLinks => {
                    self.links_to_destroy_indexes.push(link_intersection.get_link_1_index());
                    self.links_to_destroy_indexes.push(link_intersection.get_link_2_index());
                }
            }
        }
    }

    //
    // Treat cases where we need to push particles away from each other
    // because they are colliding.
    // source : https://www.gamasutra.com/view/feature/131424/pool_hall_lessons_fast_accurate_.php?page=3
    //
    fn treat_particles_to_push(&mut self) {
        for particle in &mut self.particles {
            particle.reset_move_by();
        }
        for push in self.particles_to_push.iter() {
            let p1 = self.particles[push.particle_1_index];
            let p2 = self.particles[push.particle_2_index];
            let p1_to_p2 = Vector::new(
                & self.particles[push.particle_1_index].get_coordinates_as_point(),
                & self.particles[push.particle_2_index].get_coordinates_as_point()
            );
            let n = p1_to_p2.normalized();
            let a1 = Vector::dot(self.particles[push.particle_1_index].get_speed_as_vector(), n);
            let a2 = Vector::dot(self.particles[push.particle_2_index].get_speed_as_vector(), n);
            let optimized_p = (2.0 * (a1 - a2)) / (p1.mass + p2.mass);
            let dv1 = n.multiplied(- optimized_p * p2.mass);
            let dv2 = n.multiplied(optimized_p * p1.mass);
            let dp1 = dv1.multiplied(self.get_delta_time());
            let dp2 = dv2.multiplied(self.get_delta_time());
            if p1.can_move() {
                self.particles[push.particle_1_index].add_to_move_by(dp1);
            } else {
                // Do nothing
            }
            if p2.can_move() {
                self.particles[push.particle_2_index].add_to_move_by(dp2);
            } else {
                // Do nothing
            }
        }
        for particle in &mut self.particles {
            particle.apply_move_by();
        }
        for push in self.particles_to_push.iter() {
            let p1 = self.particles[push.particle_1_index];
            let p2 = self.particles[push.particle_2_index];
            let radiuses_length = p1.get_radius() + p2.get_radius();
            let p1_to_p2 = Vector::new(
                & self.particles[push.particle_1_index].get_coordinates_as_point(),
                & self.particles[push.particle_2_index].get_coordinates_as_point()
            );
            let p1_to_p2_length = p1_to_p2.length();
            if p1_to_p2_length < radiuses_length {
                let delta_direction = p1_to_p2.normalized();
                let delta_length = radiuses_length - p1_to_p2_length;
                let delta = delta_direction.multiplied(delta_length);
                match (
                    self.particles[push.particle_1_index].can_move(),
                    self.particles[push.particle_2_index].can_move()
                ) {
                    (true, true) => {
                        self.particles[push.particle_1_index].translate(delta.multiplied(-0.5));
                        self.particles[push.particle_2_index].translate(delta.multiplied(0.5));
                    },
                    (true, false) => {
                        self.particles[push.particle_1_index].translate(delta.multiplied(-1.0));
                    },
                    (false, true) => {
                        self.particles[push.particle_2_index].translate(delta.multiplied(1.0));
                    },
                    (false, false) => {
                        // Do nothing
                    }
                };
            } else {
                // Do nothing
            }
        }
    }

    //
    // Reset particles
    //
    fn reset_particles(&mut self) {
        self.particles.clear();
        self.trajectories.clear();
        self.particle_index_to_links_indexes.clear();
    }

    //
    // Reset links
    //
    fn reset_links(&mut self) {
        self.links.clear();
        for indexes in &mut self.particle_index_to_links_indexes {
            indexes.clear();
        }
    }

    //
    // Create links if they do not already exist
    //
    fn create_links(& mut self) {
        //self.links_to_create.sort();
        //self.links_to_create.dedup();
        //
        for link_to_create in self.links_to_create.clone().iter() {
            let particle_1_index = link_to_create.get_particle_1_index();
            let particle_2_index = link_to_create.get_particle_2_index();
            if self.link_between_particles_exists(particle_1_index, particle_2_index) {
                // Do nothing
            } else {
                self.add_link(particle_1_index, particle_2_index);
            }
        }
    }

    //
    // Destroy links
    //
    fn destroy_links(& mut self) {
        // Remove duplicates
        self.links_to_destroy_indexes.sort();
        self.links_to_destroy_indexes.dedup();
        // Destroy links
        for link_to_destroy_index in self.links_to_destroy_indexes.iter().rev() {
            self.links.remove(*link_to_destroy_index);
            let tmp_particle_index_to_links_indexes = self.particle_index_to_links_indexes.clone();
            for (i, links_indexes) in tmp_particle_index_to_links_indexes.iter().enumerate() {
                for (j, link_index) in links_indexes.iter().enumerate().rev() {
                    if *link_index == *link_to_destroy_index {
                        self.particle_index_to_links_indexes[i].remove(j);
                    } else if *link_index > *link_to_destroy_index {
                        self.particle_index_to_links_indexes[i][j] -= 1;
                    } else {
                        // NTD
                    }
                }
            }
            
        }
    }

    //
    // Disable particles programmed to be disabled
    //
    fn disable_particles(& mut self) {
        // Remove duplicates
        self.particles_to_disable_indexes.sort();
        self.particles_to_disable_indexes.dedup();
        // Disable particles
        for particle_to_disable_index in self.particles_to_disable_indexes.iter().rev() {
            self.particles[*particle_to_disable_index].disable();
        }
    }

    //
    // Destroy particles
    //
    fn destroy_particles(& mut self) {
        // Remove duplicates
        self.particles_to_destroy_indexes.sort();
        self.particles_to_destroy_indexes.dedup();
        // Destroy particles
        for particle_to_destroy_index in self.particles_to_destroy_indexes.iter().rev() {
            // Add links to destroy
            let mut links_to_destroy_indexes = self.particle_index_to_links_indexes[*particle_to_destroy_index].clone();
            self.links_to_destroy_indexes.append(&mut links_to_destroy_indexes);
            // Destroy particles
            self.particles.remove(*particle_to_destroy_index);
            self.trajectories.remove(*particle_to_destroy_index);
            // Destroy references
            for (i, link) in self.links.clone().iter().enumerate() {
                let p1_index = link.get_p1_index();
                if p1_index > *particle_to_destroy_index {
                    self.links[i].decrease_p1_index();
                } else if p1_index == *particle_to_destroy_index {
                    console_warning!("Link #{} with reference to destroyed particle #{}",
                        i,
                        *particle_to_destroy_index
                    );
                } else {
                    // NTD
                }
                let p2_index = link.get_p2_index();
                if p2_index > *particle_to_destroy_index {
                    self.links[i].decrease_p2_index();
                } else if p2_index == *particle_to_destroy_index {
                    console_warning!("Link #{} with reference to destroyed particle #{}",
                        i,
                        *particle_to_destroy_index
                    );
                } else {
                    // NTD
                }
            }
        }
    }

    //
    // Check whether or not a link already exists between two particles
    //
    fn link_between_particles_exists(& self, p1_index: usize, p2_index: usize) -> bool {
        for link in & self.links {
            if (link.get_p1_index() == p1_index && link.get_p2_index() == p2_index)
                    || (link.get_p1_index() == p2_index && link.get_p2_index() == p1_index
            ) {
                return true;
            } else {
                // Do nothing
            }
        }
        false
    }

    //
    // Helper method to find if two segments intersect
    //
    fn get_segments_intersection(s1: & Segment, s2: & Segment) -> Option<Point> {
        let line_segment_1 = LineInterval::line_segment(
            Line::new(
                geo::Point::new(s1.x1, s1.y1),
                geo::Point::new(s1.x2, s1.y2)
            )
        );
        let line_segment_2 = LineInterval::line_segment(
            Line::new(
                geo::Point::new(s2.x1, s2.y1),
                geo::Point::new(s2.x2, s2.y2)
            )
        );
        match line_segment_1.relate(&line_segment_2).unique_intersection() {
            Some(intersection) => {
                Some(Point{x: intersection.x(), y: intersection.y()})
            },
            None    => {
                None
            }
        }
    }

    //
    // Returns an approximation of the intersection between a segment and a circle.
    //
    fn approximate_segment_circle_intersection(
            segment: & Segment,
            circle_center: Point,
            circle_radius: f64
    ) -> Option<Point> {
        let closest_point_option = Universe::get_closest_point_on_segment(&circle_center, &segment);
        match closest_point_option {
            Some(closest_point) => {
                let distance = Point::get_distance_2(&closest_point, &circle_center);
                if distance <= circle_radius {
                    Some(closest_point)
                } else {
                    None
                }
            },
            None => {
                None
            }
        }
    }

    //
    // Returns the point closest to p.
    // This point belongs to the segment s.
    //
    fn get_closest_point_on_segment(p: & Point, s: & Segment) -> Option<Point> {
        let x_delta = s.x2 - s.x1;
        let y_delta = s.y2 - s.y1;
        if x_delta == 0.0 && y_delta == 0.0 {
            return None;
        } else {
            // Do nothing
        }
        let u = ((p.x - s.x1) * x_delta + (p.y - s.y1) * y_delta) / (x_delta * x_delta + y_delta * y_delta);
        let closest_point = if u < 0.0 {
            Point {
                x: s.x1,
                y: s.y1
            }
        } else if u > 1.0 {
            Point {
                x: s.x2,
                y: s.y2
            }
        } else {
            Point {
                x: s.x1 + u * x_delta,
                y: s.y1 + u * y_delta
            }
        };
        return Some(closest_point);
    }

    //
    // Add a Particle to the Universe
    //
    // Add the particle itself
    // Add an empty trajectory for the particle
    // Add an empty list of links indexes for the particle.
    //
    fn add_particle(&mut self) {
        self.particles.push(Particle::new(
            self.particle_counter
        ));
        self.trajectories.push(Trajectory::new(
            self.particle_counter
        ));
        self.particle_index_to_links_indexes.push(Vec::new());
        self.particle_counter += 1;
    }

    //
    // Add a Link to the Universe
    //
    fn add_link(&mut self, p1_index: usize, p2_index: usize) {
        if p1_index >= self.particles.len() {
            console_error!("Cannot add link : particle #{} does not exist", p1_index);
        } else if p2_index >= self.particles.len() {
            console_error!("Cannot add link : particle #{} does not exist", p2_index);
        } else if p1_index >= self.particle_index_to_links_indexes.len() {
            console_error!("Cannot add link : particle_index_to_links_indexes #{} does not exist", p1_index);
        } else if p2_index >= self.particle_index_to_links_indexes.len() {
            console_error!("Cannot add link : particle_index_to_links_indexes #{} does not exist", p2_index);
        } else {
            let cycle_deltas = Particle::get_cycle_deltas(
                self.particles[p1_index],
                self.particles[p2_index]
            );
            self.links.push(Link::new(
                p1_index,
                p2_index,
                self.default_link_length,
                self.default_link_strengh,
                self.default_link_thrust_force,
                cycle_deltas[0],
                cycle_deltas[1]
            ));
            self.particle_index_to_links_indexes[p1_index].push(self.links.len() - 1);
            self.particle_index_to_links_indexes[p2_index].push(self.links.len() - 1);
        }
    }

    //
    // Get absolute forces exerced at a point in the Universe
    // by all the particles in that Universe
    //
    fn get_absolute_gravitational_field_at(&self, x: f64, y: f64, mass: f64) -> f64 {
        let mut field = (0.0, 0.0);
        for particle in self.particles.iter() {
            if particle.is_enabled() {
                let particle_gravitational_forces;
                match (self.wrap_around, self.fixed_clone_count) {
                    (true, true) => {
                        particle_gravitational_forces = particle.get_gravitational_force_wrap_around_fixed_clone_count(
                            x, y, mass,
                            self.width, self.height,
                            self.minimal_distance_for_gravity,
                            self.gravitational_constant
                        );
                    },
                    (true, false) => {
                        particle_gravitational_forces = particle.get_gravitational_force_wrap_around_variable_clone_count(
                            x, y, mass,
                            self.width, self.height,
                            self.minimal_distance_for_gravity,
                            self.gravitational_constant
                        );
                    },
                    (false, _) => {
                        particle_gravitational_forces = particle.get_gravitational_force(
                            x, y, mass,
                            self.minimal_distance_for_gravity,
                            self.gravitational_constant
                        );
                    }
                };
                field.0 += particle_gravitational_forces.x;
                field.1 += particle_gravitational_forces.y;
            } else {
                // Do nothing
            }
        }
        let a = field.0 * field.0 + field.1 * field.1;
        a.sqrt()
    }

    //
    // Prepend the latests links states
    //
    fn update_links_states(&mut self) {
        for link in &mut self.links {
            link.prepend_current_state(self.max_link_history, self.step);
        }
    }

    //
    // Helper function to get the time from a web browser
    //
    fn now() -> f64 {
        web_sys::window()
            .expect("should have a Window")
            .performance()
            .expect("should have a Performance")
            .now()
    }

    //
    // Returns the total momentum of the particles in the universe
    //
    fn get_momentum(& self) -> Vector {
        let mut momentum = Vector{x:0.0, y:0.0};
        for particle in self.particles.iter() {
            let particle_momentum = particle.get_momentum();
            momentum.x += particle_momentum.x;
            momentum.y += particle_momentum.y;
        }
        momentum
    }
}

//
// Implements the Display trait for Universe
//
impl fmt::Display for Universe {

    //
    // Format the Universe as a String
    //
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if writeln!(f, "Size : {} x {}", self.width, self.height).is_ok() {
            // NTD
        } else {
            console_error!("Could not write");
        }
        if writeln!(f, "Step : {}", self.step).is_ok() {
            // NTD
        } else {
            console_error!("Could not write");
        }
        if writeln!(f, "Tick : {:.2} ms", self.tick_average_duration).is_ok() {
            // NTD
        } else {
            console_error!("Could not write");
        }
        if writeln!(f, "Particles : {:.2}", self.particles.len()).is_ok() {
            // NTD
        } else {
            console_error!("Could not write");
        }
        if writeln!(f, "Links : {:.2}", self.links.len()).is_ok() {
            // NTD
        } else {
            console_error!("Could not write");
        }
        let momentum = self.get_momentum();
        if writeln!(f, "Momentum :\n\t{}{:.15}\n\t{}{:.15}",
            if momentum.x >= 0.0 { "+" } else { "" },
            momentum.x,
            if momentum.y >= 0.0 { "+" } else { "" },
            momentum.y
        ).is_ok() {
            // NTD
        } else {
            console_error!("Could not write");
        }
        Ok(())
    }
}

//
// Unit tests
//
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_get_absolute_gravitational_field_at() {
        let universe = Universe::new();
        let field = universe.get_absolute_gravitational_field_at(-2.0, 3.0, 1.0);
        assert_eq!(field, 0.0);
    }
}
