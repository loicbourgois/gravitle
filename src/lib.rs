mod utils;
mod link;
mod intersection;
mod particle;

extern crate web_sys;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

use link::Link;
use intersection::Intersection;
use particle::Particle;

use std::fmt;
use std::mem;

use line_intersection::{LineInterval};
use geo::{Line};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

//
// Bind `console.log` manually, without the help of `web_sys`.
//
#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    fn error(s: &str);
}

//
// Log a message to the browser console
//
macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

//
// Log an error message to the browser console
//
macro_rules! console_error {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => {
        let message = &format_args!($($t)*).to_string();
        log(&format_args!("[Error] {}", message).to_string());
    }
}

//
// Algorithm used to advance by one step
//
#[wasm_bindgen]
pub enum Algorithm {
    Euler = 1,
    Verlet = 2
}

//
// Implement convertion function for Algorithm
//
impl Algorithm {

    //
    // Convert from u32 to Algorithm
    //
    fn from_u32(value: u32) -> Algorithm {
        match value {
            1 => Algorithm::Euler,
            2 => Algorithm::Verlet,
            _ => panic!("Unknown value: {}", value),
        }
    }

    //
    // Convert Algorithm to u32
    //
    fn as_u32(&self) -> u32 {
        match self {
            Algorithm::Euler => 1,
            Algorithm::Verlet => 2
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
    tick_durations: Vec<f64>,
    tick_average_duration: f64
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
        let step = 0;
        let particles = Vec::new();
        let width = 10.0;
        let height = 10.0;
        let delta_time = 0.01;
        let algorithm = Algorithm::Verlet;
        let gravitational_constant = 667.4;
        Universe {
            width: width,
            height: height,
            step: step,
            delta_time: delta_time,
            particles: particles,
            gravitational_constant: gravitational_constant,
            particle_counter: 0,
            algorithm: algorithm,
            minimal_distance_for_gravity: delta_time * 100.0,
            links: Vec::new(),
            intersections: Vec::new(),
            tick_durations: Vec::new(),
            tick_average_duration: 0.0
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
        self.gravitational_constant = json_parsed["gravitational_constant"].as_f64().unwrap_or(self.gravitational_constant);
        self.algorithm = Algorithm::from_u32(json_parsed["algorithm"].as_u32().unwrap_or(self.algorithm.as_u32()));
        self.particles = Vec::new();
        let particles_data = &json_parsed["particles"];
        for i in 0..particles_data.len() {
            self.add_particle();
            let n = self.particles.len() - 1;
            let particle = & mut self.particles[n];
            let particle_json_string = & particles_data[i];
            particle.load_from_json(particle_json_string.to_string());
        }
        self.links = Vec::new();
        let links_data = &json_parsed["links"];
        for i in 0..links_data.len() {
            let p1_id = links_data[i]["p1_index"].as_usize().unwrap();
            let p2_id = links_data[i]["p2_index"].as_usize().unwrap();
            self.add_link(p1_id, p2_id);
        }
    }

    //
    // Advance the Universe by one step
    //
    pub fn tick(&mut self) {
        // Setup permormance analysis
        let t1 = Universe::now();

        // Perform actual computations
        self.step += 1;
        match self.algorithm {
            Algorithm::Euler => self.advance_euler(self.get_delta_time()),
            Algorithm::Verlet => self.advance_verlet(self.get_delta_time())
        }
        self.update_links_coordinates();
        self.intersections = self.get_tick_intersections();

        // Permormance analysis
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
        return self.delta_time * 1000.0;
    }

    //
    // Get delta_time for the Universe in seconds
    //
    pub fn get_delta_time(&self) -> f64 {
        return self.delta_time;
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
}

//
// Implements the Display trait for Universe
//
impl fmt::Display for Universe {

    //
    // Format the Universe as a String
    //
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if write!(f, "Size : {} x {}\n", self.width, self.height).is_ok() {
            // NTD
        } else {
            // Error
        }
        if write!(f, "Step : {}\n", self.step).is_ok() {
            // NTD
        } else {
            // Error
        }
        if write!(f, "Tick : {:.2} ms\n", self.tick_average_duration).is_ok() {
            // NTD
        } else {
            // Error
        }
        Ok(())
    }
}

//
// Private methods
//
impl Universe {

    //
    // Advance using Euler method
    //
    // See Wikipedia for explanation
    // https://en.wikipedia.org/wiki/Euler_method
    //
    fn advance_euler(&mut self, delta_time: f64) {
        for particle in &mut self.particles {
            particle.reset_forces();
        }
        let particles = self.particles.clone();
        for particle in &mut self.particles {
            particle.add_gravity_forces(
                & particles,
                self.gravitational_constant,
                self.width,
                self.height,
                self.minimal_distance_for_gravity
            );
        }
        for particle in &mut self.particles {
            particle.update_acceleration();
        }
        for particle in &mut self.particles {
            particle.update_speed(delta_time);
        }
        for particle in &mut self.particles {
            particle.update_position_euler(delta_time);
        }
        for particle in &mut self.particles {
            particle.recenter(
                self.width,
                self.height
            );
        }
    }

    //
    // Advance using Verlet Integration
    //
    // See wikipedia for explanation
    // https://en.wikipedia.org/wiki/Verlet_integration#Verlet_integration_(without_velocities)
    //
    fn advance_verlet(&mut self, delta_time: f64) {
        for particle in &mut self.particles {
            particle.reset_forces();
        }
        let particles = self.particles.clone();
        for particle in &mut self.particles {
            particle.add_gravity_forces(
                & particles,
                self.gravitational_constant,
                self.width,
                self.height,
                self.minimal_distance_for_gravity
            );
        }
        for particle in &mut self.particles {
            particle.update_acceleration();
        }
        for particle in &mut self.particles {
            particle.update_position_verlet(delta_time);
        }
        for particle in &mut self.particles {
            particle.recenter(
                self.width,
                self.height
            );
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
                & p1_coordinates.0,
                & p1_coordinates.1,
                & p2_coordinates.0,
                & p2_coordinates.1
            );
        }
    }

    //
    // Find intersections happening in a given tick
    //
    fn get_tick_intersections(& mut self) -> Vec<Intersection> {
        let mut intersections = Vec::new();
        for (link_id, link) in self.links.iter().enumerate() {
            for (particle_id, particle) in self.particles.iter().enumerate() {
                let p1_coordinates = particle.get_coordinates();
                let p1_old_coordinates = particle.get_old_coordinates();
                let coordinates = link.get_coordinates();
                let x3 = coordinates.0;
                let y3 = coordinates.1;
                let x4 = coordinates.2;
                let y4 = coordinates.3;
                match Universe::get_intersect(
                    p1_coordinates.0, p1_coordinates.1,
                    p1_old_coordinates.0, p1_old_coordinates.1,
                    x3, y3,
                    x4, y4
                ) {
                    Some(intersect) => {
                        intersections.push(
                            Intersection::new(
                                intersect.0,
                                intersect.1,
                                link_id,
                                particle_id
                            )
                        );
                    },
                    None => {
                        // NTD
                    }
                }
            }
        }
        return intersections;
    }

    //
    // Helper method to find if two links intersect
    //
    fn get_intersect(
            x1: f64, y1: f64,
            x2: f64, y2: f64,
            x3: f64, y3: f64,
            x4: f64, y4: f64
    ) -> Option<(f64, f64)> {
        let link_1 = LineInterval::line_segment(
            Line::new(
                geo::Point::new(x1, y1),
                geo::Point::new(x2, y2)
            )
        );
        let link_2 = LineInterval::line_segment(
            Line::new(
                geo::Point::new(x3, y3),
                geo::Point::new(x4, y4)
            )
        );
        match link_1.relate(&link_2).unique_intersection() {
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
        self.particle_counter += 1;
    }

    //
    // Add a Link to the Universe
    //
    fn add_link(&mut self, p1_id: usize, p2_id: usize) {
        if p1_id >= self.particles.len() {
            console_error!("Cannot add link : particle #{} does not exist", p1_id);
        }
        if p2_id >= self.particles.len() {
            console_error!("Cannot add link : particle #{} does not exist", p2_id);
        }
        self.links.push(Link::new(
            p1_id,
            p2_id
        ));
    }

    //
    // Helper function to get the time from 
    //
    fn now() -> f64 {
        web_sys::window()
            .expect("should have a Window")
            .performance()
            .expect("should have a Performance")
            .now()
    }
}
