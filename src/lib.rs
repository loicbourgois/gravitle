mod utils;

use std::fmt;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

//
// Algorithm definition
//
#[wasm_bindgen]
pub enum Algorithm {
    Euler,
    Verlet
}

//
// ValueWithDistance definition
//
struct ValueWithDistance {
    value: f64,
    d: f64
}

//
// Point definition
//
struct Point {
    x: f64,
    y: f64
}

//
// Particle definition
//
#[derive(Clone)]
pub struct Particle {
    x: f64,
    y: f64,
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
    fn new(id: u32, x: f64, y: f64) -> Particle {
        let delta_time = 1.0;
        let speed_x = 0.0;
        let speed_y = 0.0;
        let old_x = x - speed_x * delta_time;
        let old_y = y - speed_y * delta_time;
        Particle {
            x,
            y,
            forces_x: 0.0,
            forces_y: 0.0,
            acceleration_x: 0.0,
            acceleration_y: 0.0,
            mass: 1.0,
            old_x: old_x,
            old_y: old_y,
            id: id,
            speed_x: speed_x,
            speed_y: speed_y,
            is_fixed: false
        }
    }

    //
    // Reset forces for the Particle
    //
    fn reset_forces(& mut self) {
        self.forces_x = 0.0;
        self.forces_y = 0.0;
    }

    //
    // Add graavity forces exerced on the Particle by the rest of the Particles
    //
    fn add_gravity_forces(
            & mut self,
            particles: & Vec<Particle>,
            gravity: f64,
            world_width: f64,
            world_height: f64,
            delta_time: f64
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
                        if distance > delta_time * 100.0 {
                            force = - gravity * self.mass * particle.mass / (distance * distance);
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

    //
    // 
    //
    fn set_fixed(&mut self, is_fixed: bool) {
        self.is_fixed = is_fixed;
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
    gravity: f64,
    particles: Vec<Particle>,
    particle_counter: u32,
    algorithm: Algorithm
}

//
// Public methods for Universe, exported to JavaScript
//
#[wasm_bindgen]
impl Universe {

    //
    // Create a new Universe
    //
    pub fn new(
            width: f64, height: f64,
            delta_time: f64,
            gravity: f64,
            algorithm: Algorithm
    ) -> Universe {
        let step = 0;
        let delta_time = delta_time;
        let gravity = gravity;
        let particles = Vec::new();
        Universe {
            width,
            height,
            step,
            delta_time,
            particles,
            gravity,
            particle_counter: 0,
            algorithm: algorithm
        }
    }

    //
    // Advance the Universe by one step
    //
    pub fn tick(&mut self) {
        self.step += 1;
        match self.algorithm {
            Algorithm::Euler => self.advance_euler(self.get_delta_time()),
            Algorithm::Verlet => self.advance_verlet(self.get_delta_time())
        }
        
    }

    //
    // Reset the Universe
    //
    pub fn reset(&mut self) {
        self.step = 0;
        self.particles = Vec::new();
    }

    //
    // Add a Particle to the Universe at the desirated coordinates
    //
    pub fn add_particle(&mut self, x: f64, y: f64) {
        self.particles.push(Particle::new(
            self.particle_counter,
            x, y
        ));
        self.particle_counter += 1;
    }

    //
    // Add a fixed Particle to the Universe at the desirated coordinates
    //
    pub fn add_fixed_particle(&mut self, x: f64, y: f64) {
        let mut particle = Particle::new(
            self.particle_counter,
            x, y
        );
        particle.set_fixed(true);
        self.particles.push(particle);
        self.particle_counter += 1;
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
                self.gravity,
                self.width,
                self.height,
                delta_time
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
                self.gravity,
                self.width,
                self.height,
                delta_time
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
}

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
