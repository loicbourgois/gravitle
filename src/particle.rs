use crate::link::Link;
use crate::point::Point;
use crate::segment::Segment;
use crate::vector::Vector;

//
// Collision happens when two particles collide
//
#[derive(Copy, Clone)]
pub enum ParticleCollisionBehavior {
    DoNothing,
    DisableSelf,
    DestroySelf
}

//
// Implement convertion function for Algorithm
//
impl ParticleCollisionBehavior {

    //
    // Convert from String to ParticleCollisionBehavior
    //
    fn from_string(value: String) -> ParticleCollisionBehavior {
        match value.as_ref() {
            "do-nothing" => ParticleCollisionBehavior::DoNothing,
            "disable-self" => ParticleCollisionBehavior::DisableSelf,
            "destroy-self" => ParticleCollisionBehavior::DestroySelf,
            _ => {
                panic!("Unknown CollisionBehavior : {}", value);
            }
        }
    }

    //
    // Convert ParticleCollisionBehavior to String
    //
    pub fn as_string(self) -> String {
        match self {
            ParticleCollisionBehavior::DoNothing => "do-nothing".to_string(),
            ParticleCollisionBehavior::DisableSelf => "disable-self".to_string(),
            ParticleCollisionBehavior::DestroySelf => "destroy-self".to_string()
        }
    }
}

//
// Helper struct
//
struct ValueWithDistance {
    value: f64,
    d: f64
}

const FLOAT_COMPARE_MARGIN : f64 = 0.0000000001;
const DEFAULT_LINK_FORCE_X : f64 = 0.0;
const DEFAULT_LINK_FORCE_Y : f64 = 0.0000000001;

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
    is_fixed: bool,
    collision_behavior: ParticleCollisionBehavior,
    is_enabled: bool,
    cycle_x: i32,
    cycle_y: i32
}

//
// Private method for Particle
//
impl Particle {

    //
    // Create a new Particle
    //
    pub fn new(id: u32) -> Particle {
        Particle {
            id,
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
            speed_x: 0.0,
            speed_y: 0.0,
            is_fixed: false,
            collision_behavior: ParticleCollisionBehavior::DoNothing,
            is_enabled: true,
            cycle_x: 0,
            cycle_y: 0
        }
    }

    //
    // Checks whether two particles collide
    //
    pub fn particles_collide(p1: & Particle, p2: & Particle) -> bool {
        let distance_squared_centers = Point::get_distance_squared(p1.x, p1.y, p2.x, p2.y);
        let radiuses_squared = ((p1.diameter * 0.5) + (p2.diameter * 0.5)) * ((p1.diameter * 0.5) + (p2.diameter * 0.5));
        distance_squared_centers < radiuses_squared && p1.id != p2.id && p1.is_enabled() && p2.is_enabled()
    }

    //
    // Load from JSON
    //
    pub fn load_from_json(&mut self, json_string: String) {
        let json_parsed = &json::parse(&json_string).unwrap();
        self.x = json_parsed["x"].as_f64().unwrap_or(self.x);
        self.y = json_parsed["y"].as_f64().unwrap_or(self.y);
        self.diameter = json_parsed["diameter"].as_f64().unwrap_or(self.diameter);
        self.old_x = json_parsed["old_x"].as_f64().unwrap_or(self.x);
        self.old_y = json_parsed["old_y"].as_f64().unwrap_or(self.y);
        self.mass = json_parsed["mass"].as_f64().unwrap_or(self.mass);
        self.is_fixed = json_parsed["fixed"].as_bool().unwrap_or(self.is_fixed);
        self.is_enabled = json_parsed["enabled"].as_bool().unwrap_or(self.is_enabled);
        if json_parsed["collision_behavior"].to_string() != "null" {
            self.collision_behavior = ParticleCollisionBehavior::from_string(json_parsed["collision_behavior"].to_string());
        } else {
            // Do nothing
        }
     }

    //
    // Reset forces for the Particle
    //
    pub fn reset_forces(& mut self) {
        self.forces_x = 0.0;
        self.forces_y = 0.0;
    }

    //
    // Add gravity forces exerced on the Particle by the rest of the Particles
    // in a Universe with wrap_around enabled
    //
    pub fn add_gravity_forces_wrap_around_fixed_clone_count (
            & mut self,
            particles: &[Particle],
            gravitational_constant: f64,
            world_width: f64,
            world_height: f64,
            minimal_distance_for_gravity: f64
    ) {
        if self.is_fixed {
            // NTD
        } else {
            for particle in particles {
                if particle.id != self.id && particle.is_enabled() {
                    let force = self.get_gravitational_force_wrap_around_fixed_clone_count(
                        particle.x,
                        particle.y,
                        particle.mass,
                        world_width,
                        world_height,
                        minimal_distance_for_gravity,
                        gravitational_constant
                    );
                    self.add_force(force);
                } else {
                    // NTD
                }
            }
        }
    }

    pub fn add_gravity_forces_wrap_around_variable_clone_count (
            & mut self,
            particles: &[Particle],
            gravitational_constant: f64,
            world_width: f64,
            world_height: f64,
            minimal_distance_for_gravity: f64
    ) {
        if self.is_fixed {
            // NTD
        } else {
            for particle in particles {
                if particle.id != self.id && particle.is_enabled() {
                    let force = self.get_gravitational_force_wrap_around_variable_clone_count(
                        particle.x,
                        particle.y,
                        particle.mass,
                        world_width,
                        world_height,
                        minimal_distance_for_gravity,
                        gravitational_constant
                    );
                    self.add_force(force);
                } else {
                    // NTD
                }
            }
        }
    }

    //
    // Add gravity forces for a standard Universe
    //
    pub fn add_gravity_forces(
            & mut self,
            particles: &[Particle],
            gravitational_constant: f64,
            minimal_distance_for_gravity: f64
    ) {
        if self.is_fixed {
            // NTD
        } else {
            for particle in particles {
                if particle.id != self.id && particle.is_enabled() {
                        let distance = Point::get_distance(
                            self.x, self.y,
                            particle.x, particle.y
                        );
                        let force = if distance > minimal_distance_for_gravity {
                            - gravitational_constant * self.mass * particle.mass / (distance * distance)
                        } else {
                            // Particles are too close, which can result in instability
                            0.0
                        };
                        let delta_x = self.x - particle.x;
                        let delta_y = self.y - particle.y;
                        let force = Vector {x: delta_x * force, y: delta_y * force};
                        self.add_force(force);
                } else {
                    // NTD
                }
            }
        }
    }

    //
    // Add a force to the particle
    //
    pub fn add_force(&mut self, force: Vector) {
        self.forces_x += force.x;
        self.forces_y += force.y;
    }

    //
    // Get forces applied by a link between two particles if wrap around
    // is disabled
    //
    pub fn get_link_forces_wrap_around_disabled(
            p1: & Particle,  p2: & Particle, link: & Link) -> Vector {
        Particle::get_link_forces_wrap_around_enabled(p1, p2, link, 0.0, 0.0)
    }

    //
    // Get forces applied by a link between two particles if wrap around
    // is enabled
    //
    pub fn get_link_forces_wrap_around_enabled(
            p1: & Particle,  p2: & Particle,
            link: & Link,
            universe_width: f64, universe_height: f64) -> Vector
    {
        let cycle_x_delta = p2.cycle_x - p1.cycle_x - link.get_initial_cycle_delta_x();
        let cycle_y_delta = p2.cycle_y - p1.cycle_y - link.get_initial_cycle_delta_y();
        let x1 = p1.x;
        let y1 = p1.y;
        let x2 = p2.x + cycle_x_delta as f64 * universe_width;
        let y2 = p2.y + cycle_y_delta as f64 * universe_height;
        let delta_length = Point::get_distance(x1, y1, x2, y2) - link.get_length();
        let strengh = link.get_strengh();
        let unit_vector_option = Point::get_normalized_vector(
            x1, y1,
            x2, y2
        );
        let force_x;
        let force_y;
        match unit_vector_option {
            Some(unit_vector) => {
                force_x = unit_vector.0 * delta_length * strengh;
                force_y = unit_vector.1 * delta_length * strengh;
            },
            None => {
                force_x = DEFAULT_LINK_FORCE_X * delta_length * strengh;
                force_y = DEFAULT_LINK_FORCE_Y * delta_length * strengh;
            }
        }
        Vector {
            x: force_x,
            y: force_y
        }
    }

    //
    // Add drag forces
    //
    pub fn add_drag_forces(&mut self, drag_coefficient: f64) {
        self.forces_x -= drag_coefficient * self.speed_x * self.speed_x.abs();
        self.forces_y -= drag_coefficient * self.speed_y * self.speed_y.abs();
    }

    //
    // Update the acceleration of the Particle
    //
    pub fn update_acceleration(& mut self) {
        self.acceleration_x = self.forces_x / self.mass;
        self.acceleration_y = self.forces_y / self.mass;
    }

    //
    // Update the speed of the Particle
    //
    pub fn update_speed_euler(&mut self, delta_time: f64) {
        if self.is_fixed {
            self.speed_x = 0.0;
            self.speed_y = 0.0;
        } else {
            self.speed_x += self.acceleration_x * delta_time;
            self.speed_y += self.acceleration_y * delta_time;
        }
    }

    //
    // Update the speed of the Particle
    //
    pub fn update_speed_verlet(&mut self, delta_time: f64) {
        if self.is_fixed {
            self.speed_x = 0.0;
            self.speed_y = 0.0;
        } else {
            self.speed_x = (self.x - self.old_x ) / delta_time;
            self.speed_y = (self.y - self.old_y ) / delta_time;
        }
    }

    //
    // Update the position of the Particle using the Euler Algorithm
    //
    pub fn update_position_euler(&mut self, delta_time: f64) {
        let current_x = self.x;
        let current_y = self.y;
        if self.is_fixed {
            // Do nothing
        } else {
            self.x += self.speed_x * delta_time;
            self.y += self.speed_y * delta_time;
        }
        self.old_x = current_x;
        self.old_y = current_y;
    }

    //
    // Update the position of the Particle using the Verlet Algorithm
    //
    pub fn update_position_verlet(&mut self, delta_time: f64) {
        let current_x = self.x;
        let current_y = self.y;
        if self.is_fixed {
            // Do nothing
        } else {
            self.x = 2.0 * current_x - self.old_x + self.acceleration_x * delta_time * delta_time;
            self.y = 2.0 * current_y - self.old_y + self.acceleration_y * delta_time * delta_time;
        }
        self.old_x = current_x;
        self.old_y = current_y;
    }

    //
    // Recenter a particle if it got outside the Universe
    //
    pub fn wrap_around (
            &mut self,
            world_width: f64,
            world_height: f64
    ) {
        let x_max = world_width * 0.5;
        let x_min = -x_max;
        let y_max = world_height * 0.5;
        let y_min = -y_max;
        if self.x < x_min {
            self.x += world_width;
            self.old_x += world_width;
            self.cycle_x -= 1;
        } else if self.x > x_max {
            self.x -= world_width;
            self.old_x -= world_width;
            self.cycle_x += 1;
        } else {
            // Do nothing
        }
        if self.y < y_min {
            self.y += world_height;
            self.old_y += world_height;
            self.cycle_y -= 1;
        } else if self.y > y_max {
            self.y -= world_height;
            self.old_y -= world_height;
            self.cycle_y += 1;
        } else {
            // Do nothing
        }
    }

    //
    // Stabilize position by removing last decimals for each coordinate
    // Useful for conserving symetries
    //
    pub fn stabilise_position(&mut self, stabiliser: f64) {
        self.x = (self.x * stabiliser).trunc() / stabiliser;
        self.y = (self.y * stabiliser).trunc() / stabiliser;
    }

    //
    // Returns current coordinates
    //
    pub fn get_coordinates(&self) -> [f64; 2] {
        [self.x, self.y]
    }

    //
    // Returns current coordinates as a Point
    //
    pub fn get_coordinates_as_point(&self) -> Point {
        Point {
            x: self.x,
            y: self.y
        }
    }

    //
    // Getter for x current coordinates
    //
    pub fn get_x(&self) -> f64 {
        self.x
    }

    //
    // Getter for y current coordinates
    //
    pub fn get_y(&self) -> f64 {
        self.y
    }

    //
    // Getter for old coordinates
    //
    pub fn get_old_coordinates_as_point(& self) -> Point {
        Point{x: self.old_x, y:self.old_y}
    }

    //
    // Returns both old and current coordinates as a single segment.
    //
    pub fn get_coordinates_as_segment(& self) -> Segment {
        Segment {
            x1: self.old_x,
            y1: self.old_y,
            x2: self.x,
            y2: self.y
        }
    }

    //
    // Disable the Particle
    //
    pub fn disable(&mut self) {
        self.is_enabled = false;
    }

    //
    // Getter for is_fixed field
    //
    pub fn is_fixed(& self) -> bool {
        self.is_fixed
    }

    //
    // Getter for is_enabled
    //
    pub fn is_enabled(& self) -> bool {
        self.is_enabled
    }

    //
    // Getter for collision_behavior
    //
    pub fn get_collision_behavior(&self) -> ParticleCollisionBehavior {
        self.collision_behavior
    }

    //
    // Get gravitational forces exerced by one particle on another particle
    //
    pub fn get_gravitational_force(
            &self,
            x: f64,
            y: f64,
            mass: f64,
            minimal_distance_for_gravity: f64,
            gravitational_constant: f64
    ) -> Vector {
        let distance = Point::get_distance(
            self.x, self.y,
            x, y
        );
        let force = if distance > minimal_distance_for_gravity {
            - gravitational_constant * self.mass * mass / (distance * distance)
        } else {
            // Particles are too close, which can result in instability
            0.0
        };
        let delta_x = self.x - x;
        let delta_y = self.y - y;
        let force_x = delta_x * force;
        let force_y = delta_y * force;
        Vector {
            x: force_x,
            y: force_y
        }
    }

    //
    // Get gravitational forces exerced by one particle on another particle
    // if wrap around is enabled.
    // Always requires 4 clones.
    //
    pub fn get_gravitational_force_wrap_around_fixed_clone_count(
            &self,
            x: f64,
            y: f64,
            mass: f64,
            universe_width: f64,
            universe_height: f64,
            minimal_distance_for_gravity: f64,
            gravitational_constant: f64
    ) -> Vector {
        let mut forces = Vector{x: 0.0, y: 0.0};
        let clones_coordinates = Particle::get_wrap_around_clones_coordinates_fixed_count(
            self.x,
            self.y,
            x,
            y,
            universe_width,
            universe_height
        );
        for clone in clones_coordinates.iter() {
            let force = self.get_gravitational_force(
                clone.x, clone.y, mass, minimal_distance_for_gravity, gravitational_constant
            );
            forces.x += force.x;
            forces.y += force.y;
        }
        return forces;
    }

    //
    // Get gravitational forces exerced by one particle on another particle
    // if wrap around is enabled.
    // Works with a variable number of clones.
    //
    pub fn get_gravitational_force_wrap_around_variable_clone_count(
            &self,
            x: f64,
            y: f64,
            mass: f64,
            universe_width: f64,
            universe_height: f64,
            minimal_distance_for_gravity: f64,
            gravitational_constant: f64
    ) -> Vector {
        let mut forces = Vector{x: 0.0, y: 0.0};
        let clones_coordinates = Particle::get_wrap_around_clones_coordinates_variable_count(
            self.x,
            self.y,
            x,
            y,
            universe_width,
            universe_height
        );
        for clone_option in clones_coordinates.iter() {
            match clone_option {
                Some(clone) => {
                    let force = self.get_gravitational_force(
                        clone.x, clone.y, mass, minimal_distance_for_gravity, gravitational_constant
                    );
                    forces.x += force.x;
                    forces.y += force.y;
                },
                None => {
                    // Do nothing
                }
            }
        }
        return forces;
    }

    //
    // Get delta cyle between two particles
    //
    pub fn get_cycle_deltas(p1: Particle, p2: Particle) -> [i32; 2] {
        [p2.cycle_x - p1.cycle_x, p2.cycle_y - p1.cycle_y]
    }

    //
    // Returns the particle's diameter.
    //
    pub fn get_diameter(&self) -> f64 {
        self.diameter
    }

    //
    // Returns the particle's radius.
    //
    pub fn get_radius(&self) -> f64 {
        self.diameter * 0.5
    }
}

//
// Private methods
//
impl Particle {

    //
    // Get cloned points based on p1
    // boxing a point p2
    // where p1 clones are translation of p1
    // by width and/or height of the Universe.
    //
    // Used to compute gravity through the visible edges of the Universe.
    //
    fn get_wrap_around_clones_coordinates_fixed_count(
            x1: f64, y1: f64,
            x2: f64, y2: f64,
            width: f64, height: f64
    ) -> [Point; 4] {
        let mut xs = [
            ValueWithDistance {
                value: x2 - width,
                d: Point::get_distance_squared(x1, 0.0, x2 - width, 0.0)
            },
            ValueWithDistance {
                value: x2,
                d: Point::get_distance_squared(x1, 0.0, x2, 0.0)
            },
            ValueWithDistance {
                value: x2 + width,
                d: Point::get_distance_squared(x1, 0.0, x2 + width, 0.0)
            }
        ];
        let mut ys = [
            ValueWithDistance {
                value: y2 - height,
                d: Point::get_distance_squared(0.0, y1, 0.0, y2 - height)
            },
            ValueWithDistance {
                value: y2,
                d: Point::get_distance_squared(0.0, y1, 0.0, y2)
            },
            ValueWithDistance {
                value: y2 + height,
                d: Point::get_distance_squared(0.0, y1, 0.0, y2 + height)
            }
        ];
        // Order by ascending distance
        xs.sort_by(|a, b| a.d.partial_cmp(&b.d).unwrap());
        ys.sort_by(|a, b| a.d.partial_cmp(&b.d).unwrap());
        let x_average = (x1+x2) * 0.5;
        let y_average = (y1+y2) * 0.5;
        //
        if (x1 - x2).abs() < FLOAT_COMPARE_MARGIN && (y1 - y2).abs() < FLOAT_COMPARE_MARGIN {
            return [
                Point {x: x_average, y: y_average},
                Point {x: x_average, y: y_average},
                Point {x: x_average, y: y_average},
                Point {x: x_average, y: y_average}
            ];
        } else if (x1 - x2).abs() < FLOAT_COMPARE_MARGIN {
            return [
                Point {x: x_average, y: ys[0].value},
                Point {x: x_average, y: ys[1].value},
                Point {x: x_average, y: ys[0].value},
                Point {x: x_average, y: ys[1].value}
            ];
        } else if (y1 - y2).abs() < FLOAT_COMPARE_MARGIN {
            return [
                Point {x: xs[0].value, y: y_average},
                Point {x: xs[1].value, y: y_average},
                Point {x: xs[0].value, y: y_average},
                Point {x: xs[1].value, y: y_average}
            ];
        } else {
            return [
                Point {x: xs[0].value, y: ys[0].value},
                Point {x: xs[0].value, y: ys[1].value},
                Point {x: xs[1].value, y: ys[0].value},
                Point {x: xs[1].value, y: ys[1].value}
            ];
        }
    }

    //
    // Returns coordinates for clones of this point given the width and height
    // of the Universe.
    //
    fn get_wrap_around_clones_coordinates_variable_count(
            x1: f64, y1: f64,
            x2: f64, y2: f64,
            width: f64, height: f64
    ) -> [Option<Point>; 4] {
        let mut xs = [
            ValueWithDistance {
                value: x2 - width,
                d: Point::get_distance_squared(x1, 0.0, x2 - width, 0.0)
            },
            ValueWithDistance {
                value: x2,
                d: Point::get_distance_squared(x1, 0.0, x2, 0.0)
            },
            ValueWithDistance {
                value: x2 + width,
                d: Point::get_distance_squared(x1, 0.0, x2 + width, 0.0)
            }
        ];
        let mut ys = [
            ValueWithDistance {
                value: y2 - height,
                d: Point::get_distance_squared(0.0, y1, 0.0, y2 - height)
            },
            ValueWithDistance {
                value: y2,
                d: Point::get_distance_squared(0.0, y1, 0.0, y2)
            },
            ValueWithDistance {
                value: y2 + height,
                d: Point::get_distance_squared(0.0, y1, 0.0, y2 + height)
            }
        ];
        // Order by ascending distance
        xs.sort_by(|a, b| a.d.partial_cmp(&b.d).unwrap());
        ys.sort_by(|a, b| a.d.partial_cmp(&b.d).unwrap());
        let x_average = (x1+x2) * 0.5;
        let y_average = (y1+y2) * 0.5;
        //
        if (x1 - x2).abs() < FLOAT_COMPARE_MARGIN && (y1 - y2).abs() < FLOAT_COMPARE_MARGIN {
            return [
                Some(Point {x: x_average, y: y_average}),
                None,
                None,
                None
            ];
        } else if (x1 - x2).abs() < FLOAT_COMPARE_MARGIN {
            return [
                Some(Point {x: x_average, y: ys[0].value}),
                Some(Point {x: x_average, y: ys[1].value}),
                None,
                None
            ];
        } else if (y1 - y2).abs() < FLOAT_COMPARE_MARGIN {
            return [
                Some(Point {x: xs[0].value, y: y_average}),
                Some(Point {x: xs[1].value, y: y_average}),
                None,
                None
            ];
        } else {
            return [
                Some(Point {x: xs[0].value, y: ys[0].value}),
                Some(Point {x: xs[0].value, y: ys[1].value}),
                Some(Point {x: xs[1].value, y: ys[0].value}),
                Some(Point {x: xs[1].value, y: ys[1].value})
            ];
        }
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
        let d2: f64 = Point::get_distance_squared(x1, y1, x2, y2);
        assert_eq!(d, d2);
    }

    #[test]
    pub fn test_get_distance() {
        let x1: f64 = 0.0;
        let y1: f64 = 0.0;
        let x2: f64 = -4.0;
        let y2: f64 = 0.0;
        let d: f64 = 4.0;
        let d2: f64 = Point::get_distance(x1, y1, x2, y2);
        assert_eq!(d, d2);
    }

    #[test]
    pub fn test_particles_collide() {
        //
        let mut p1 = Particle::new(0);
        p1.load_from_json(r#"{
            "x": 1.0,
            "y": 1.0
        }"#.to_string());
        let mut p2 = Particle::new(1);
        p2.load_from_json(r#"{
            "x": 1.0,
            "y": 0.0
        }"#.to_string());
        let mut p3 = Particle::new(3);
        p3.load_from_json(r#"{
            "x": 1.0,
            "y": 0.0,
            "diameter": 1.01
        }"#.to_string());
        assert_eq!(false, Particle::particles_collide(&p1, &p1));
        assert_eq!(false, Particle::particles_collide(&p1, &p2));
        assert_eq!(true, Particle::particles_collide(&p1, &p3));
    }
}
