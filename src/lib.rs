mod utils;
mod segment;
mod intersection;

extern crate web_sys;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

use segment::Segment;
use intersection::Intersection;

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
// Algorithm definition
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
// ValueWithDistance definition
//
struct ValueWithDistance {
    value: f64,
    d: f64
}

//
// Represent 2d coordinates
//
struct Point {
    x: f64,
    y: f64
}

//
// Particle definition
//
#[derive(Copy, Clone)]
pub struct Particle {
    x: f64,
    y: f64,
    diameter: f64,
    forces_x: f64,
    forces_y: f64,
    acceleration_x: f64,
    acceleration_y: f64,
    mass: f64,
    old_x: f64,
    old_y: f64,
    speed_x: f64,
    speed_y: f64,
    id: u32,
    is_fixed: bool
}

//
// Private method for Particle
//
impl Particle {

    //
    // Create a new Particle
    //
    fn new(id: u32) -> Particle {
        Particle {
            x: 0.0,
            y: 0.0,
            diameter: 1.0,
            forces_x: 0.0,
            forces_y: 0.0,
            acceleration_x: 0.0,
            acceleration_y: 0.0,
            mass: 1.0,
            old_x: 0.0,
            old_y: 0.0,
            id: id,
            speed_x: 0.0,
            speed_y: 0.0,
            is_fixed: false
        }
    }

    //
    // Load
    //
    fn load_from_json(&mut self, json_string: String) {
        let json_parsed = &json::parse(&json_string).unwrap();
        self.x = json_parsed["x"].as_f64().unwrap_or(self.x);
        self.y = json_parsed["y"].as_f64().unwrap_or(self.y);
        self.diameter = json_parsed["diameter"].as_f64().unwrap_or(self.diameter);
        self.old_x = self.x;
        self.old_y = self.y;
        self.mass = json_parsed["mass"].as_f64().unwrap_or(self.mass);
        self.is_fixed = json_parsed["fixed"].as_bool().unwrap_or(self.is_fixed);
     }

    //
    // Reset forces for the Particle
    //
    fn reset_forces(& mut self) {
        self.forces_x = 0.0;
        self.forces_y = 0.0;
    }

    //
    // Add gravity forces exerced on the Particle by the rest of the Particles
    //
    fn add_gravity_forces(
            & mut self,
            particles: & Vec<Particle>,
            gravitational_constant: f64,
            world_width: f64,
            world_height: f64,
            minimal_distance_for_gravity: f64
    ) {
        if self.is_fixed {
            // NTD
        } else {
    
            for particle in particles {
                if particle.id != self.id {
                    let clones_coordinates = Particle::get_boxing_clones_coordinates(
                        self.x,
                        self.y,
                        particle.x,
                        particle.y,
                        world_width,
                        world_height
                    );
                    for clone in clones_coordinates {
                        let distance = Particle::get_distance(
                            self.x, self.y,
                            clone.x, clone.y
                        );
                        let force;
                        if distance > minimal_distance_for_gravity {
                            force = - gravitational_constant * self.mass * particle.mass / (distance * distance);
                        } else {
                            // Particles are too close, which can result in instability
                            force = 0.0;
                        }
                        let delta_x = self.x - clone.x;
                        let delta_y = self.y - clone.y;
                        let force_x = delta_x * force;
                        let force_y = delta_y * force;
                        self.forces_x += force_x;
                        self.forces_y += force_y;
                    }
                } else {
                    // NTD
                }
            }
        }
    }

    //
    // Get cloned points based on p1
    // boxing a point p2
    // where p1 clones are translation of p1
    // by width and/or height of the Universe.
    //
    // If p1 and p2 have same x or same y,
    // returns only 2 clones
    //
    // Used to compute gravity through the visible edges of the Universe.
    //
    fn get_boxing_clones_coordinates(
            x1: f64, y1: f64,
            x2: f64, y2: f64,
            width: f64, height: f64
    ) -> Vec<Point> {
        let mut clones = Vec::new();
        let ws = [-width, 0.0, width];
        let hs = [-height, 0.0, height];
        let mut xs = Vec::new();
        let mut ys = Vec::new();
        for w in ws.iter() {
            let x = x2 + w;
            let distance_squared = Particle::get_distance_squared(x1, 0.0, x, 0.0);
            xs.push( ValueWithDistance {
                value: x,
                d: distance_squared
            });
        }
        for h in hs.iter() {
            let y = y2 + h;
            let distance_squared = Particle::get_distance_squared(0.0, y1, 0.0, y);
            ys.push( ValueWithDistance {
                value: y,
                d: distance_squared
            });
        }
        // Order by ascending distance
        xs.sort_by(|a, b| a.d.partial_cmp(&b.d).unwrap());
        ys.sort_by(|a, b| a.d.partial_cmp(&b.d).unwrap());
        //
        if x1 == x2 && y1 == y2 {
            // NTD
        } else if x1 == x2 {
            for _ in 0..2 {
                for i in 0..2 {
                    clones.push(Point {
                        x: x1,
                        y: ys[i].value
                    });
                }
            }
        } else if y1 == y2 {
            for _ in 0..2 {
                for i in 0..2 {
                    clones.push(Point {
                        x: xs[i].value,
                        y: y1
                    });
                }
            }
        } else {
            for i in 0..2 {
                for j in 0..2 {
                    clones.push(Point {
                        x: xs[i].value,
                        y: ys[j].value
                    });
                }
            }
        }
        return clones;
    }

    //
    // Helper function to get a distance squared
    //
    fn get_distance_squared(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
        let delta_x = x1 - x2;
        let delta_y = y1 - y2;
        return delta_x * delta_x + delta_y * delta_y;
    }

    //
    // Helper function to get a distance
    //
    fn get_distance(
            x1: f64, y1: f64,
            x2: f64, y2: f64
    ) -> f64 {
        let d2 = Particle::get_distance_squared(x1, y1, x2, y2);
        return d2.sqrt();
    }

    //
    // Update the acceleration of the Particle
    //
    fn update_acceleration(& mut self) {
        self.acceleration_x = self.forces_x / self.mass;
        self.acceleration_y = self.forces_y / self.mass;
    }

    //
    // Update the speed of the Particle
    //
    fn update_speed(&mut self, delta_time: f64) {
        self.speed_x = self.speed_x + self.acceleration_x * delta_time;
        self.speed_y = self.speed_y + self.acceleration_y * delta_time;
    }

    //
    // Update the position of the Particle using the Euler Algorithm
    //
    fn update_position_euler(&mut self, delta_time: f64) {
        self.x += self.speed_x * delta_time;
        self.y += self.speed_y * delta_time;
    }

    //
    // Update the position of the Particle using the Verlet Algorithm
    //
    fn update_position_verlet(&mut self, delta_time: f64) {
        let current_x = self.x;
        let current_y = self.y;
        let new_x = 2.0 * current_x - self.old_x + self.acceleration_x * delta_time * delta_time ;
        let new_y = 2.0 * current_y - self.old_y + self.acceleration_y * delta_time * delta_time ;
        self.old_x = current_x;
        self.old_y = current_y;
        self.x = new_x;
        self.y = new_y;
    }

    //
    // Recenter a particle if it got outside the Universe
    //
    fn recenter(
            &mut self,
            world_width: f64,
            world_height: f64
    ) {
        let x_max = world_width / 2.0;
        let x_min = -x_max;
        let y_max = world_height / 2.0;
        let y_min = -y_max;
        let mut x = self.x + x_max - x_min - x_min;
        while x > x_max - x_min {
            x -= x_max - x_min;
        }
        x += x_min;
        let mut y = self.y + y_max - y_min - y_min;
        while y > y_max - y_min {
            y -= y_max - y_min;
        }
        y += y_min;
        self.x = x;
        self.y = y;
    }
}

//
// Represents a Universe
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
    segments: Vec<Segment>,
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
            segments: Vec::new(),
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
        self.segments = Vec::new();
        let segments_data = &json_parsed["segments"];
        for i in 0..segments_data.len() {
            let p1_id = segments_data[i]["p1_index"].as_usize().unwrap();
            let p2_id = segments_data[i]["p2_index"].as_usize().unwrap();
            self.add_segment(p1_id, p2_id);
        }
    }


    fn now() -> f64 {
        web_sys::window()
            .expect("should have a Window")
            .performance()
            .expect("should have a Performance")
            .now()
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
        self.update_segments_coordinates();
        self.intersections = self.get_tick_intersections();

        // Permormance analysis
        let t2 = Universe::now();
        let duration = t2 - t1;
        self.tick_durations.push(duration);
        if self.tick_durations.len() > (1.0 / self.delta_time) as usize {
            self.tick_durations.drain(0..1);
        }
        self.tick_average_duration = 0.0;
        for duration in self.tick_durations.iter() {
            self.tick_average_duration += duration;
        }
        self.tick_average_duration /= self.tick_durations.len() as f64;
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
    // Get delta_time for the Universe
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
    // Return a pointer to the Vec of Segments
    //
    pub fn get_segments(&self) -> *const Segment {
        self.segments.as_ptr()
    }

    //
    // Return number of Segments in the Universe
    //
    pub fn get_segments_count(&self) -> usize {
        self.segments.len()
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
    // Update segments coordinates by coppying particles coordinates to segments
    //
    fn update_segments_coordinates(&mut self) {
        for segment in &mut self.segments {
            segment.set_coordinates(
                & self.particles[segment.get_p1_id()].x,
                & self.particles[segment.get_p1_id()].y,
                & self.particles[segment.get_p2_id()].x,
                & self.particles[segment.get_p2_id()].y
            );
        }
    }

    //
    // Find intersections happening in a given tick
    //
    fn get_tick_intersections(& mut self) -> Vec<Intersection> {
        let mut intersections = Vec::new();
        for (segment_id, segment) in self.segments.iter().enumerate() {
            for (particle_id, particle) in self.particles.iter().enumerate() {
                let x1 = particle.x;
                let y1 = particle.y;
                let x2 = particle.old_x;
                let y2 = particle.old_y;
                let coordinates = segment.get_coordinates();
                let x3 = coordinates.0;
                let y3 = coordinates.1;
                let x4 = coordinates.2;
                let y4 = coordinates.3;
                match Universe::get_intersect(
                    x1, y1,
                    x2, y2,
                    x3, y3,
                    x4, y4
                ) {
                    Some(intersect) => {
                        intersections.push(
                            Intersection::new(
                                intersect.0,
                                intersect.1,
                                segment_id,
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
    // Helper method to find if two segments intersect
    //
    fn get_intersect(
            x1: f64, y1: f64,
            x2: f64, y2: f64,
            x3: f64, y3: f64,
            x4: f64, y4: f64
    ) -> Option<(f64, f64)> {
        let segment_1 = LineInterval::line_segment(
            Line::new(
                geo::Point::new(x1, y1),
                geo::Point::new(x2, y2)
            )
        );
        let segment_2 = LineInterval::line_segment(
            Line::new(
                geo::Point::new(x3, y3),
                geo::Point::new(x4, y4)
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
        self.particle_counter += 1;
    }

    //
    // Add a Segment to the Universe
    //
    fn add_segment(&mut self, p1_id: usize, p2_id: usize) {
        if p1_id >= self.particles.len() {
            console_error!("Cannot add segment : particle #{} does not exist", p1_id);
        }
        if p2_id >= self.particles.len() {
            console_error!("Cannot add segment : particle #{} does not exist", p2_id);
        }
        self.segments.push(Segment::new(
            p1_id,
            p2_id
        ));
    }
}

//
// Unit tests
//
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_get_boxing_clones_coordinates() {
        let x1: f64 = 0.0;
        let y1: f64 = 0.0;
        let x2: f64 = 1.0;
        let y2: f64 = 2.0;
        let width: f64 = 10.0;
        let height: f64 = 8.0;
        let mut clones = Vec::new();
        clones.push(Point {
            x: 1.0,
            y: 2.0
        });
        clones.push(Point {
            x: 1.0,
            y: -6.0
        });
        clones.push(Point {
            x: -9.0,
            y: 2.0
        });
        clones.push(Point {
            x: -9.0,
            y: -6.0
        });
        let clones2 = Particle::get_boxing_clones_coordinates(
            x1,
            y1,
            x2,
            y2,
            width,
            height
        );
        assert_eq!(4, clones2.len());
        assert_eq!(clones[0].x, clones2[0].x);
        assert_eq!(clones[0].y, clones2[0].y);
        assert_eq!(clones[1].x, clones2[1].x);
        assert_eq!(clones[1].y, clones2[1].y);
        assert_eq!(clones[2].x, clones2[2].x);
        assert_eq!(clones[2].y, clones2[2].y);
        assert_eq!(clones[3].x, clones2[3].x);
        assert_eq!(clones[3].y, clones2[3].y);
    }

    #[test]
    pub fn test_get_distance_squared() {
        let x1: f64 = 0.0;
        let y1: f64 = 0.0;
        let x2: f64 = 1.0;
        let y2: f64 = 2.0;
        let d: f64 = 5.0;
        let d2: f64 = Particle::get_distance_squared(x1, y1, x2, y2);
        assert_eq!(d, d2);
    }

    #[test]
    pub fn test_get_distance() {
        let x1: f64 = 0.0;
        let y1: f64 = 0.0;
        let x2: f64 = -4.0;
        let y2: f64 = 0.0;
        let d: f64 = 4.0;
        let d2: f64 = Particle::get_distance(x1, y1, x2, y2);
        assert_eq!(d, d2);
    }
}
