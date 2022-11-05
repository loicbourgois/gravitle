use crate::Vector;
use crate::World;
use rand::Rng;
pub type Particles = Vec<Particle>;
// impl Particles {
//
// }
#[derive(Debug)]
pub struct Particle {
    pub p: Vector,
    pub v: Vector,
    pub pp: Vector,
    pub m: f32,
    pub collisions: u32,
    pub pid: usize, // particle id
    pub tid: usize, // thread id
    pub gid: usize,
}
pub struct ParticleConfiguration<'a> {
    pub world: &'a World,
    pub pid: usize,
}
// todo: elapsed to frontedn as float 
impl Particle {
    pub fn new_particles(world: &World) -> Particles {
        let mut particles = Vec::new();
        for pid in 0..world.particle_count {
            particles.push(Particle::new(&ParticleConfiguration { pid, world }));
        }
        particles
    }
    pub fn new_particles_2(world: &World) -> Particles {
        let mut rng = rand::thread_rng();
        let mut particles = Vec::new();
        for pid in 0..world.particle_count {
            particles.push(Particle::new(&ParticleConfiguration { pid, world }));
        }
        for i in 0..1000 {
            let mut p = &mut particles[i];
            p.p.x = 0.05 * rng.gen::<f32>();
            p.p.y = 0.05 * rng.gen::<f32>();
            p.v.x = world.diameter*0.125;
            p.v.y = world.diameter*0.125;
            p.pp = Vector {
                x: p.p.x - p.v.x,
                y: p.p.y - p.v.y,
            };
        }
        particles
    }
    pub fn new(c: &ParticleConfiguration) -> Particle {
        let mut rng = rand::thread_rng();
        let world = c.world;
        let tid = c.pid % world.thread_count;
        let p = Vector {
            x: rng.gen::<f32>() * 0.5 + 0.25,
            y: rng.gen::<f32>() * 0.5 + 0.25,
        };
        // let v = Vector {
        //     x: world.diameter * 0.9 * rng.gen::<f32>() - 0.5 * world.diameter * 0.9,
        //     y: world.diameter * 0.9 * rng.gen::<f32>() - 0.5 * world.diameter * 0.9,
        // };
        let v = Vector { x: 0.0, y: 0.0 };
        Particle {
            p,
            pp: Vector {
                x: p.x - v.x,
                y: p.y - v.y,
            },
            v,
            m: 1.0,
            tid,
            pid: c.pid,
            collisions: 0,
            gid: 0,
        }
    }
}
