use crate::collision_response;
use crate::neighbours;
use crate::normalize;
use crate::normalize_2;
use crate::particle::Particles;
use crate::particle::Pkind;
use crate::wait;
use crate::wrap_around;
use crate::Deltas;
use crate::Grid;
use crate::Links;
use crate::Syncers;
use crate::World;
use rand::Rng;
use std::sync::atomic::AtomicPtr;
use std::sync::atomic::Ordering;
use std::thread;
pub fn compute_child<'a>(
    world: &World,
    booster_acceleration: f32,
    crd: f32,
    crdv: f32,
    link_strengh: f32,
    linkt_length_ratio: f32,
    syncers: Syncers,
    grid: &'a mut Grid,
    deltas: &'a mut Deltas,
    particles: &'a mut Particles,
    links: &'a mut Links,
) {
    let mut threads = Vec::new();
    for tid in 0..world.thread_count {
        let particles_ptr = AtomicPtr::new(particles);
        let links_ptr = AtomicPtr::new(links);
        let deltas_ptr = AtomicPtr::new(deltas);
        let grid_ptr = AtomicPtr::new(grid);
        let syncers = syncers.clone();
        let world = *world;
        threads.push(thread::spawn(move || {
            let mut rng = rand::thread_rng();
            unsafe {
                let particles = &mut (*particles_ptr.load(Ordering::Relaxed));
                let _particles2 = &mut (*particles_ptr.load(Ordering::Relaxed));
                let links = &mut (*links_ptr.load(Ordering::Relaxed));
                let deltas = &mut (*deltas_ptr.load(Ordering::Relaxed));
                let particles2 = &mut (*particles_ptr.load(Ordering::Relaxed));
                let grid = &(*grid_ptr.load(Ordering::Relaxed));
                loop {
                    {
                        let mut w = syncers[0][tid].write().unwrap();
                        *w += 1;
                    }
                    wait(&syncers[0], tid);
                    //
                    // Read particles
                    // Read link
                    // Write deltas
                    //
                    {
                        let mut w = syncers[1][tid].write().unwrap();
                        for i in 0..world.particle_per_thread {
                            let pid1 = i * world.thread_count + tid;
                            let p1 = &particles[pid1];
                            for ns in neighbours(&p1.p, grid) {
                                for pid2 in ns {
                                    let p2 = &(*particles2)[*pid2];
                                    if p1.pid < p2.pid {
                                        let wa = wrap_around(&p1.p, &p2.p);
                                        if wa.d_sqrd < world.particle_diameter_sqrd {
                                            let cr = collision_response(&wa, p1, p2);
                                            if !cr.x.is_nan() && !cr.y.is_nan() {
                                                {
                                                    let d1 = &mut deltas
                                                        [tid * world.particle_count + p1.pid];
                                                    d1.collisions += 1;
                                                    d1.v.x += cr.x * crdv;
                                                    d1.v.y += cr.y * crdv;
                                                    d1.p.x -= wa.d.x * crd;
                                                    d1.p.y -= wa.d.y * crd;
                                                }
                                                {
                                                    let d2 = &mut deltas
                                                        [tid * world.particle_count + p2.pid];
                                                    d2.collisions += 1;
                                                    d2.v.x -= cr.x * crdv;
                                                    d2.v.y -= cr.y * crdv;
                                                    d2.p.x += wa.d.x * crd;
                                                    d2.p.y += wa.d.y * crd;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        let size = links.len() / world.thread_count + 1;
                        let start = size * tid;
                        let end = (start + size).min(links.len());
                        for lid in start..end {
                            let p1 = &particles[links[lid][0]];
                            let p2 = &particles[links[lid][1]];
                            let wa = wrap_around(&p1.p, &p2.p);
                            let d = wa.d_sqrd.sqrt();
                            let n = normalize(&wa.d, d);
                            let factor = (world.diameter * linkt_length_ratio - d) * link_strengh;
                            if wa.d_sqrd < world.particle_diameter_sqrd {
                                let cr = collision_response(&wa, p1, p2);
                                if !cr.x.is_nan() && !cr.y.is_nan() {
                                    {
                                        let d1 = &mut deltas[tid * world.particle_count + p1.pid];
                                        d1.collisions += 1;
                                        d1.v.x -= cr.x * crdv * 0.5;
                                        d1.v.y -= cr.y * crdv * 0.5;
                                        d1.p.x += wa.d.x * crd;
                                        d1.p.y += wa.d.y * crd;
                                    }
                                    {
                                        let d2 = &mut deltas[tid * world.particle_count + p2.pid];
                                        d2.collisions += 1;
                                        d2.v.x += cr.x * crdv * 0.5;
                                        d2.v.y += cr.y * crdv * 0.5;
                                        d2.p.x -= wa.d.x * crd;
                                        d2.p.y -= wa.d.y * crd;
                                    }
                                }
                            }
                            if !n.x.is_nan() && !n.y.is_nan() {
                                {
                                    let d1 = &mut deltas[tid * world.particle_count + p1.pid];
                                    d1.v.x -= n.x * factor;
                                    d1.v.y -= n.y * factor;
                                    d1.direction.x += wa.d.x;
                                    d1.direction.y += wa.d.y;
                                }
                                {
                                    let d2 = &mut deltas[tid * world.particle_count + p2.pid];
                                    d2.v.x += n.x * factor;
                                    d2.v.y += n.y * factor;
                                    d2.direction.x -= wa.d.x;
                                    d2.direction.y -= wa.d.y;
                                }
                            }
                        }
                        *w += 1;
                    }
                    wait(&syncers[1], tid);
                    //
                    // Reset particle
                    // Read delta
                    // Reset delta
                    // Write particle
                    //
                    {
                        let mut w = syncers[2][tid].write().unwrap();
                        for i in 0..world.particle_per_thread {
                            let pid1 = i * world.thread_count + tid;
                            let mut p1 = &mut particles[pid1];
                            p1.collisions = 0;
                            p1.direction.x = 0.0;
                            p1.direction.y = 0.0;
                            p1.v.x = p1.p.x - p1.pp.x;
                            p1.v.y = p1.p.y - p1.pp.y;
                            for tid in 0..world.thread_count {
                                let d1 = &mut deltas[tid * world.particle_count + p1.pid];
                                // assert!(d1.p.x >= 0.0, "\n{:?}", d1);
                                // assert!(d1.p.y >= 0.0, "\n{:?}", d1);
                                // assert!(d1.p.x <= 1.0, "\n{:?}", d1);
                                // assert!(d1.p.y <= 1.0, "\n{:?}", d1);
                                assert!(!d1.v.x.is_nan(), "\n{:?}", d1);
                                assert!(!d1.v.y.is_nan(), "\n{:?}", d1);
                                assert!(!d1.p.x.is_nan(), "\n{:?}", d1);
                                assert!(!d1.p.y.is_nan(), "\n{:?}", d1);
                                p1.collisions += d1.collisions;
                                p1.direction.x += d1.direction.x;
                                p1.direction.y += d1.direction.y;
                                p1.v.x += d1.v.x;
                                p1.v.y += d1.v.y;
                                p1.p.x += d1.p.x;
                                p1.p.y += d1.p.y;
                                d1.collisions = 0;
                                d1.p.x = 0.0;
                                d1.p.y = 0.0;
                                d1.v.x = 0.0;
                                d1.v.y = 0.0;
                                d1.direction.x = 0.0;
                                d1.direction.y = 0.0;
                            }
                            p1.direction = normalize_2(&p1.direction);
                            let mut gun_ok = true;
                            if p1.direction.x.is_nan() || p1.direction.y.is_nan() {
                                p1.direction.x = 0.0;
                                p1.direction.y = 0.0;
                                gun_ok = false;
                            }
                            if let Pkind::Booster = p1.kind {
                                p1.v.x += p1.direction.x * p1.activation * booster_acceleration;
                                p1.v.y += p1.direction.y * p1.activation * booster_acceleration;
                            }
                            p1.v.x = p1.v.x.max(-world.diameter * 0.5);
                            p1.v.x = p1.v.x.min(world.diameter * 0.5);
                            p1.v.y = p1.v.y.max(-world.diameter * 0.5);
                            p1.v.y = p1.v.y.min(world.diameter * 0.5);
                            p1.p.x = (10.0 + p1.p.x + p1.v.x) % 1.0;
                            p1.p.y = (10.0 + p1.p.y + p1.v.y) % 1.0;
                            p1.pp.x = p1.p.x - p1.v.x;
                            p1.pp.y = p1.p.y - p1.v.y;
                            assert!(!p1.p.x.is_nan(), "\n{:?}", p1);
                            assert!(!p1.p.y.is_nan(), "\n{:?}", p1);
                            assert!(p1.v.x >= -world.diameter, "\n{:?}", p1);
                            assert!(p1.v.y >= -world.diameter, "\n{:?}", p1);
                            assert!(p1.v.x <= world.diameter, "\n{:?}", p1);
                            assert!(p1.v.y <= world.diameter, "\n{:?}", p1);
                            assert!(p1.p.x >= 0.0, "\n{:?}", p1);
                            assert!(p1.p.y >= 0.0, "\n{:?}", p1);
                            assert!(p1.p.x <= 1.0, "\n{:?}", p1);
                            assert!(p1.p.y <= 1.0, "\n{:?}", p1);

                            if p1.kind == Pkind::Gun && gun_ok && p1.activation >= 0.9 {
                                let mut p2 =
                                    &mut particles2[rng.gen_range(p1.pid + 20..p1.pid + 500)];
                                p2.p.x = p1.p.x - p1.direction.x * world.diameter * 1.1;
                                p2.p.y = p1.p.y - p1.direction.y * world.diameter * 1.1;
                                p2.pp.x =
                                    p2.p.x + p1.direction.x * world.diameter * 0.5 - p1.v.x;
                                p2.pp.y =
                                    p2.p.y + p1.direction.y * world.diameter * 0.5 - p1.v.y;
                                for tid in 0..world.thread_count {
                                    let d2 = &mut deltas[tid * world.particle_count + p2.pid];
                                    d2.collisions = 0;
                                    d2.p.x = 0.0;
                                    d2.p.y = 0.0;
                                    d2.v.x = 0.0;
                                    d2.v.y = 0.0;
                                    d2.direction.x = 0.0;
                                    d2.direction.y = 0.0;
                                }
                                p1.activation = -1.0;
                            }
                        }
                        *w += 1;
                    }
                    wait(&syncers[2], tid);
                    //
                    //
                    //
                    //
                    {
                        let mut w = syncers[3][tid].write().unwrap();
                        *w += 1;
                    }
                    wait(&syncers[3], tid);
                }
            }
        }));
    }
}
