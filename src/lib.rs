#[macro_use] mod utils;
mod link;
mod intersection;
mod collision;
mod link_intersection;
mod particle;
mod trajectory;

extern crate web_sys;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

use link::Link;
use intersection::Intersection;
use link_intersection::LinkIntersection;
use particle::Particle;
use collision::Collision;
use trajectory::Trajectory;
use particle::ParticleCollisionBehavior;

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
    MergeParticles
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
            CollisionBehavior::MergeParticles => "merge-particles".to_string()
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
    link_intersections: Vec<LinkIntersection>,
    tick_durations: Vec<f64>,
    tick_average_duration: f64,
    particle_index_to_links_indexes: Vec<Vec<usize>>,
    intersection_behavior: IntersectionBehavior,
    collision_behavior: CollisionBehavior,
    link_intersection_behavior: LinkIntersectionBehavior,
    links_to_destroy_indexes: Vec<usize>,
    particles_to_destroy_indexes: Vec<usize>,
    particles_to_disable_indexes: Vec<usize>,
    links_to_create: Vec<(usize, usize)>,
    stabiliser_power: i32,
    stabilise_positions_enabled: bool,
    trajectories: Vec<Trajectory>,
    default_link_length: f64,
    default_link_strengh: f64,
    drag_coefficient: f64,
    wrap_around: bool
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
            tick_durations: Vec::new(),
            tick_average_duration: 0.0,
            particle_index_to_links_indexes: Vec::new(),
            intersection_behavior: IntersectionBehavior::DoNothing,
            collision_behavior: CollisionBehavior::DoNothing,
            link_intersection_behavior: LinkIntersectionBehavior::DoNothing,
            links_to_destroy_indexes: Vec::new(),
            particles_to_destroy_indexes: Vec::new(),
            particles_to_disable_indexes: Vec::new(),
            links_to_create: Vec::new(),
            stabiliser_power: 10,
            stabilise_positions_enabled: false,
            trajectories: Vec::new(),
            default_link_length: 10.0,
            default_link_strengh: 100.0,
            drag_coefficient: 0.5,
            wrap_around: true
        }
    }

    //
    // Load Universe from a JSON String
    //
    pub fn load_from_json(&mut self, json_string: String) {
        let json_parsed = &json::parse(&json_string).unwrap();
        self.width = json_parsed["minimal_distance_for_gravity"].as_f64().unwrap_or(self.minimal_distance_for_gravity);
        self.width = json_parsed["width"].as_f64().unwrap_or(self.width);
        self.height = json_parsed["height"].as_f64().unwrap_or(self.height);
        self.delta_time = json_parsed["delta_time"].as_f64().unwrap_or(self.delta_time);
        self.stabiliser_power = json_parsed["stabiliser_power"].as_i32().unwrap_or(self.stabiliser_power);
        self.gravitational_constant = json_parsed["gravitational_constant"].as_f64().unwrap_or(self.gravitational_constant);
        self.default_link_length = json_parsed["default_link_length"].as_f64().unwrap_or(self.default_link_length);
        self.default_link_strengh = json_parsed["default_link_strengh"].as_f64().unwrap_or(self.default_link_strengh);
        self.drag_coefficient = json_parsed["drag_coefficient"].as_f64().unwrap_or(self.drag_coefficient);
        self.wrap_around = json_parsed["wrap_around"].as_bool().unwrap_or(self.wrap_around);
        self.stabilise_positions_enabled = json_parsed["stabilise_positions_enabled"]
            .as_bool()
            .unwrap_or(self.stabilise_positions_enabled);
        if json_parsed["algorithm"].to_string()  != "null" {
            self.algorithm = Algorithm::from_string(json_parsed["algorithm"].to_string());
        } else {
            // Do nothing
        }
        if json_parsed["intersection_behavior"].to_string() != "null" {
            self.intersection_behavior = IntersectionBehavior::from_string(json_parsed["intersection_behavior"].to_string());
        } else {
            // Do nothing
        }
        if json_parsed["collision_behavior"].to_string() != "null" {
            self.collision_behavior = CollisionBehavior::from_string(json_parsed["collision_behavior"].to_string());
        } else {
            // Do nothing
        }
        if json_parsed["link_intersection_behavior"].to_string() != "null" {
            self.link_intersection_behavior = LinkIntersectionBehavior::from_string(
                json_parsed["link_intersection_behavior"].to_string()
            );
        } else {
            // Do nothing
        }
        self.particles = Vec::new();
        let particles_data = &json_parsed["particles"];
        for i in 0..particles_data.len() {
            let particle_json_string = & particles_data[i];
            self.add_particle_json(particle_json_string.to_string());
        }
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
        self.particles_to_destroy_indexes.clear();
        self.particles_to_disable_indexes.clear();
        self.links_to_destroy_indexes.clear();
        self.links_to_create.clear();
        self.reset_forces();
        if self.wrap_around {
            self.add_gravity_forces_wrap_around();
        } else {
            self.add_gravity_forces();
        }
        self.add_link_forces();
        self.add_drag_forces();
        self.update_acceleration();
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
        if self.wrap_around {
            self.recenter();
        } else {
            // Do nothing
        }
        match self.stabilise_positions_enabled {
            true => {
                self.stabilise_positions();
            },
            false => {
                // Do nothing
            }
        }
        self.update_trajectories();
        self.update_links_coordinates();
        self.compute_tick_intersections();
        self.compute_tick_collisions();
        self.compute_tick_link_intersections();
        self.treat_intersections();
        self.treat_collisions();
        self.treat_link_intersections();
        self.create_links();
        self.disable_particles();
        self.destroy_particles();
        self.destroy_links();
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
        self.particles = Vec::new();
        self.trajectories = Vec::new();
        self.reset_links();
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
    // Setter for gravitational_constant
    //
    pub fn set_gravitational_constant(&mut self, gravitational_constant: f64) {
        self.gravitational_constant = gravitational_constant;
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
        Ok(())
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
                particle.add_gravity_forces_wrap_around(
                    & particles,
                    self.gravitational_constant,
                    self.width,
                    self.height,
                    self.minimal_distance_for_gravity
                );
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
            let p1_index = link.get_p1_index();
            let p2_index = link.get_p2_index();
            let p1 = & self.particles[p1_index];
            let p2 = & self.particles[p2_index];
            if p1.is_enabled() && p2.is_enabled() {
                let forces = Particle::get_link_forces(p1, p2, link);
                let force_x = forces.0 / 2.0;
                let force_y = forces.1 / 2.0;
                self.particles[p1_index].add_force(force_x, force_y);
                self.particles[p2_index].add_force(-force_x, -force_y);
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
    fn recenter(&mut self) {
        for particle in &mut self.particles {
            if particle.is_enabled() {
                particle.recenter(
                    self.width,
                    self.height
                );
            } else {
                // Do nothing
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
    // Update trajectories
    //
    fn update_trajectories(&mut self) {
        let step = self.step;
        for index in 0..self.trajectories.len() {
            if self.particles[index].is_fixed() || !self.particles[index].is_enabled() {
                // Do nothing
            } else {
                let coordinates = self.particles[index].get_coordinates();
                self.trajectories[index].add_position(
                    coordinates.0,
                    coordinates.1,
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
            let p1_coordinates = self.particles[link.get_p1_index()].get_coordinates();
            let p2_coordinates = self.particles[link.get_p2_index()].get_coordinates();
            link.set_coordinates(
                p1_coordinates.0,
                p1_coordinates.1,
                p2_coordinates.0,
                p2_coordinates.1
            );
        }
    }

    //
    // List intersections happening during the current tick
    //
    fn compute_tick_intersections(& mut self) {
        self.intersections.clear();
        for (link_index, link) in self.links.iter().enumerate() {
            for (particle_index, particle) in self.particles.iter().enumerate() {
                if self.links[link_index].has_particle(particle_index) || !particle.is_enabled() {
                    // Do nothing
                } else {
                    let p1_coordinates = particle.get_coordinates();
                    let p1_old_coordinates = particle.get_old_coordinates();
                    let coordinates = link.get_coordinates();
                    let x3 = coordinates.0;
                    let y3 = coordinates.1;
                    let x4 = coordinates.2;
                    let y4 = coordinates.3;
                    match Universe::get_intersect(
                        p1_coordinates,
                        p1_old_coordinates,
                        (x3, y3),
                        (x4, y4)
                    ) {
                        Some(intersect) => {
                            self.intersections.push(
                                Intersection::new(
                                    intersect.0,
                                    intersect.1,
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
            }
        }
    }

    //
    // List collisions happening during the current tick
    //
    fn compute_tick_collisions(& mut self) {
        self.collisions.clear();
        for (i, particle_1) in self.particles.iter().enumerate() {
            for (j, particle_2) in self.particles.iter().enumerate() {
                let particles_collide = Particle::particles_collide(particle_1, particle_2);
                if particles_collide {
                    self.collisions.push(Collision::new(i, j));
                } else {
                    // NTD
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
                let p1_1_index = link_1.get_p1_index();
                let p1_2_index = link_1.get_p2_index();
                let p2_1_index = link_2.get_p1_index();
                let p2_2_index = link_2.get_p2_index();
                if p1_1_index != p2_1_index
                        && p1_1_index != p2_2_index
                        && p1_2_index != p2_1_index
                        && p1_2_index != p2_2_index {
                    let c1 = link_1.get_coordinates();
                    let c2 = link_2.get_coordinates();
                    let links_intersection_option = Universe::get_intersect(
                        (c1.0, c1.1),
                        (c1.2, c1.3),
                        (c2.0, c2.1),
                        (c2.2, c2.3)
                    );
                    match links_intersection_option {
                        Some(links_intersection) => {
                            self.link_intersections.push(LinkIntersection::new(
                                links_intersection.0, links_intersection.1, i, j
                            ));
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
                    self.links_to_create.push((p1_index, p2_index));
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
    // Reset links
    //
    fn reset_links(&mut self) {
        self.links.clear();
        for indexes in &mut self.particle_index_to_links_indexes {
            indexes.clear();
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
    // Create links if they do not already exist
    //
    fn create_links(& mut self) {
        self.links_to_create.sort();
        self.links_to_create.dedup();
        //
        for link_to_create in self.links_to_create.clone().iter() {
            if self.link_between_particles_exists(link_to_create.0, link_to_create.1) {
                // Do nothing
            } else {
                self.add_link(link_to_create.0, link_to_create.1);
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
    fn get_intersect(
            p1: (f64, f64),
            p2: (f64, f64),
            p3: (f64, f64),
            p4: (f64, f64)
    ) -> Option<(f64, f64)> {
        let segment_1 = LineInterval::line_segment(
            Line::new(
                geo::Point::new(p1.0, p1.1),
                geo::Point::new(p2.0, p2.1)
            )
        );
        let segment_2 = LineInterval::line_segment(
            Line::new(
                geo::Point::new(p3.0, p3.1),
                geo::Point::new(p4.0, p4.1)
            )
        );
        match segment_1.relate(&segment_2).unique_intersection() {
            Some(intersect) => {
                Some((intersect.x(), intersect.y()))
            },
            None    => {
                None
            }
        }
    }

    //
    // Add a Particle to the Universe
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
            self.links.push(Link::new(
                p1_index,
                p2_index,
                self.default_link_length,
                self.default_link_strengh
            ));
            self.particle_index_to_links_indexes[p1_index].push(self.links.len() - 1);
            self.particle_index_to_links_indexes[p2_index].push(self.links.len() - 1);
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
}
