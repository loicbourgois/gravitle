use crate::point::Point;

//
// Pair of particles to push away from each other
//
pub struct ParticlesToPush {
    pub particle_1_index: usize,
    pub particle_2_index: usize,
    pub collision_point: Point
}
