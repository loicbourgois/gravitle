use crate::particle::Particles;
use crate::particle::Pkind;
use crate::rotate;
use crate::HashSet;
use crate::Links;
use crate::Particle;
use crate::Pid;
use crate::Vector;
use crate::World;
pub fn setup_5(
    links: &mut Links,
    particles: &mut Particles,
    world: &World,
    free_ship_pids: &mut HashSet<Pid>,
) {
    *particles = Particle::new_particles(world);
    let l = particles.len();
    let dl = l / world.ships_count;
    for i in 0..world.ships_count {
        let pid0 = i * dl;
        free_ship_pids.insert(pid0);
        let mut p0 = &mut particles[pid0];
        p0.p.x = 0.5 + world.diameter * 10.0 * i as f32
            - world.diameter * 5.0 * world.ships_count as f32;
        p0.p.y = 0.9 + world.diameter * 4.0;
        reset_ship(pid0, particles, world, links);
    }
}

pub fn reset_ship_particles(pid0: usize, particles: &mut Particles, world: &World) {
    let mut p0 = &mut particles[pid0];
    p0.v.x = 0.0;
    p0.v.y = 0.0;
    p0.kind = Pkind::Core;
    p0.pp = Vector {
        x: p0.p.x - p0.v.x,
        y: p0.p.y - p0.v.y,
    };
    let pos = rotate(
        &p0.p,
        &Vector {
            x: p0.p.x + world.diameter,
            y: p0.p.y,
        },
        4.0 / 12.0,
    );
    let mut p1 = &mut particles[pid0 + 1];
    p1.p = pos;
    p1.v.x = 0.0;
    p1.v.y = 0.0;
    p1.pp = Vector {
        x: p1.p.x - p1.v.x,
        y: p1.p.y - p1.v.y,
    };
    let parts = [
        (0, 1, Pkind::Armor),     // 2
        (0, 2, Pkind::Armor),     // 3
        (0, 3, Pkind::Gun),       // 4
        (0, 4, Pkind::Gun),       // 5
        (0, 5, Pkind::Armor),     // 6
        (3, 2, Pkind::Armor),     // 7
        (3, 7, Pkind::Armor),     // 8
        (8, 7, Pkind::Armor),     // 9
        (9, 7, Pkind::Booster),   // 10
        (1, 6, Pkind::Armor),     // 11
        (11, 6, Pkind::Armor),    // 12
        (11, 12, Pkind::Armor),   // 13
        (11, 13, Pkind::Booster), // 14
        (4, 3, Pkind::Armor),     // 15
        (6, 5, Pkind::Armor),     // 16
    ];
    let mut pid3 = pid0 + 2;
    for part in parts {
        let pid1 = pid0 + part.0;
        let pid2 = pid0 + part.1;
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
        p.kind = part.2;
        pid3 += 1;
    }
}

pub fn reset_ship(pid0: usize, particles: &mut Particles, world: &World, links: &mut Links) {
    reset_ship_particles(pid0, particles, world);
    for aa in [
        [0, 1],
        [0, 2],
        [0, 3],
        [0, 4],
        [0, 5],
        [0, 6],
        [1, 2],
        [2, 3],
        [3, 4],
        [4, 5],
        [5, 6],
        [1, 6],
        [3, 8],
        [3, 7],
        [2, 7],
        [7, 8],
        [8, 9],
        [7, 9],
        [9, 10],
        [7, 10],
        [6, 11],
        [11, 12],
        [1, 11],
        [11, 13],
        [11, 14],
        [6, 12],
        [12, 13],
        [13, 14],
        [3, 15],
        [4, 15],
        [8, 15],
        [5, 16],
        [6, 16],
        [12, 16],
    ] {
        links.push([aa[0] + pid0, aa[1] + pid0])
    }
}
