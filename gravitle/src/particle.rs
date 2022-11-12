use crate::math::rotate;
use crate::Vector;
use crate::World;
use rand::Rng;
pub type Particles = Vec<Particle>;

#[derive(Debug)]
#[repr(u8)]
pub enum Pkind {
    Armor = 1,
    Booster = 2,
    Gun = 3,
    Matter = 4,
    Energy = 5,
}
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
    pub activation: f32,
    pub direction: Vector,
    pub kind: Pkind,
    pub enabled: bool,
}
pub struct ParticleConfiguration<'a> {
    pub world: &'a World,
    pub pid: usize,
}
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
        for p in particles.iter_mut().take(100) {
            p.p.x = 0.01 * rng.gen::<f32>();
            p.p.y = 0.01 * rng.gen::<f32>() + 0.5 - 0.05 * 0.5;
            p.v.x = world.diameter * 0.5;
            p.v.y = world.diameter * 0.0;
            p.pp = Vector {
                x: p.p.x - p.v.x,
                y: p.p.y - p.v.y,
            };
        }
        particles
    }
    pub fn new_particles_3(world: &World) -> Particles {
        let mut rng = rand::thread_rng();
        let mut particles = Vec::new();
        for pid in 0..world.particle_count {
            particles.push(Particle::new(&ParticleConfiguration { pid, world }));
        }
        for p in &mut particles {
            p.p.x = rng.gen::<f32>();
            p.p.y = rng.gen::<f32>();
            p.v.x = world.diameter * 0.125 * (rng.gen::<f32>() - 0.5);
            p.v.y = world.diameter * 0.125 * (rng.gen::<f32>() - 0.5);
            p.pp = Vector {
                x: p.p.x - p.v.x,
                y: p.p.y - p.v.y,
            };
        }
        particles
    }
    pub fn new_particles_4(world: &World) -> Particles {
        let mut rng = rand::thread_rng();
        let mut particles = Vec::new();
        for pid in 0..world.particle_count {
            particles.push(Particle::new(&ParticleConfiguration { pid, world }));
        }
        for p in &mut particles {
            p.p.x = rng.gen::<f32>() * 0.5 + 0.25;
            p.p.y = rng.gen::<f32>() * 0.5 + 0.25;
            p.v.x = 0.0;
            p.v.y = 0.0;
            p.pp = Vector {
                x: p.p.x - p.v.x,
                y: p.p.y - p.v.y,
            };
        }
        let mut p = &mut particles[0];
        p.p.x = 0.1;
        p.p.y = 0.09;
        p.v.x = 0.0;
        p.v.y = 0.0;
        p.pp = Vector {
            x: p.p.x - p.v.x,
            y: p.p.y - p.v.y,
        };
        let mut p = &mut particles[1];
        p.p.x = 0.105;
        p.p.y = 0.09;
        p.v.x = 0.0;
        p.v.y = 0.0;
        p.pp = Vector {
            x: p.p.x - p.v.x,
            y: p.p.y - p.v.y,
        };

        let mut p = &mut particles[4];
        p.p.x = 0.1;
        p.p.y = 0.1;
        p.v.x = 0.0;
        p.v.y = 0.0;
        p.pp = Vector {
            x: p.p.x - p.v.x,
            y: p.p.y - p.v.y,
        };
        let mut p = &mut particles[3];
        p.p.x = 0.11;
        p.p.y = 0.1;
        p.v.x = -0.000003;
        p.v.y = 0.0;
        p.pp = Vector {
            x: p.p.x - p.v.x,
            y: p.p.y - p.v.y,
        };
        let mut p = &mut particles[2];
        p.p.x = 0.1015;
        p.p.y = 0.1;
        p.v.x = 0.0;
        p.v.y = 0.0;
        p.pp = Vector {
            x: p.p.x - p.v.x,
            y: p.p.y - p.v.y,
        };
        particles
    }

    pub fn new_particles_5(world: &World) -> Particles {
        let mut rng = rand::thread_rng();
        let mut particles = Vec::new();
        for pid in 0..world.particle_count {
            particles.push(Particle::new(&ParticleConfiguration { pid, world }));
        }
        for p in &mut particles {
            p.p.x = rng.gen::<f32>() ;
            p.p.y = rng.gen::<f32>() ;
            p.v.x = 0.0;
            p.v.y = 0.0;
            p.pp = Vector {
                x: p.p.x - p.v.x,
                y: p.p.y - p.v.y,
            };
        }
        {
            let mut p0 = &mut particles[0];
            p0.p.x = 0.5;
            p0.p.y = 0.765;
            p0.v.x = 0.0;
            p0.v.y = 0.0;
            p0.pp = Vector {
                x: p0.p.x - p0.v.x,
                y: p0.p.y - p0.v.y,
            };
        }
        let pos = rotate(&particles[0].p, &Vector {
            x: particles[0].p.x + world.diameter ,
            y: particles[0].p.y
        }, 4.0 / 12.0);
        particles[1].p = pos;
        particles[1].v.x = 0.0;
        particles[1].v.y = 0.0;
        particles[1].pp = Vector {
            x: particles[1].p.x - particles[1].v.x,
            y: particles[1].p.y - particles[1].v.y,
        };
        let parts = [
            (0,1, "armor"),     // 2
            (0,2, "armor"),     // 3
            (0,3, "armor"),     // 4
            (0,4, "armor"),     // 5
            (0,5, "armor"),     // 6
            (3,2, "armor"),     // 7
            (3,7, "armor"),     // 8
            (8,7, "armor"),     // 9
            (9,7, "armor"),     // 10
            (1,6, "armor"),     // 11
            (11,6, "armor"),    // 12
            (11,12, "armor"),   // 13
            (11,13, "armor"),   // 14
            (4, 3, "gun"),      // 15
            (6, 5, "gun"),      // 16
          ];
        let mut pid3 = 2;
        for part in parts {
            let pid1 = part.0;
            let pid2 = part.1;
            let pos = rotate(&particles[pid1].p, &particles[pid2].p, -1.0 / 6.0);
            let mut p = &mut particles[pid3];
            p.p.x = pos.x;
            p.p.y = pos.y;
            p.v.x = 0.0;
            p.v.y = 0.0;
            p.pp = Vector {
                x: p.p.x - p.v.x,
                y: p.p.y - p.v.y,
            };
            pid3 += 1;
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
            activation: 0.0,
            direction: Vector { x: 0.0, y: 0.0 },
            kind: Pkind::Matter,
            enabled: true,
        }
    }
}
