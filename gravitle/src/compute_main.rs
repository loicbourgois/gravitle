use crate::approx_equal;
use crate::grid_id;
use crate::grid_xy;
use crate::particle::Particles;
use crate::particle::Pkind;
use crate::reset_ship_particles;
use crate::wait;
use crate::Grid;
use crate::SharedNetworkData;
use crate::Syncers;
use crate::Utc;
use crate::World;
use std::collections::HashSet;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use std::time::Instant;
use tungstenite::Message;
pub fn compute_main(
    shared_network_data: SharedNetworkData,
    syncers: Syncers,
    world: World,
    mut particles: Particles,
    mut grid: Grid,
) {
    let mut elapsed_total = 0;
    let mut step = 0;
    let mut elapsed_network = Instant::now().elapsed().as_micros();
    thread::spawn(move || loop {
        let start = Instant::now();
        //
        // Step 0
        //
        {
            let mut w = syncers[0][world.thread_count].write().unwrap();
            grid.update_01();
            grid.update_02(&mut particles);
            for user in shared_network_data.lock().unwrap().users.values_mut() {
                for (pid, activation) in &user.orders {
                    let mut p1 = &mut particles[*pid + user.ship_pid];
                    match p1.kind {
                        Pkind::Booster => {
                            p1.activation = *activation;
                        }
                        Pkind::Core => {
                            p1.activation = *activation;
                        }
                        Pkind::Gun => {
                            if approx_equal(*activation, 0.0)
                                || approx_equal(*activation, 1.0)
                                    && approx_equal(p1.activation, 0.0)
                            {
                                p1.activation = *activation;
                            }
                        }
                        _ => {}
                    }
                }
                user.orders.clear();
            }
            *w += 1;
        }
        wait(&syncers[0], world.thread_count);
        //
        // Step 1
        //
        {
            let mut w = syncers[1][world.thread_count].write().unwrap();
            *w += 1;
        }
        wait(&syncers[1], world.thread_count);
        //
        // Step 2
        //
        {
            let mut w = syncers[2][world.thread_count].write().unwrap();
            *w += 1;
        }
        wait(&syncers[2], world.thread_count);
        //
        // Step 3
        //
        {
            let mut w = syncers[3][world.thread_count].write().unwrap();
            let mut collisions_count = 0;
            let mut ships_to_reset = HashSet::new();
            for p1 in particles.iter() {
                collisions_count += p1.collisions;
                if p1.kind == Pkind::Core && p1.activation >= 0.9 {
                    ships_to_reset.insert(p1.pid);
                }
            }
            for pid in ships_to_reset {
                reset_ship_particles(pid, &mut particles, &world);
            }
            let elapsed_compute = start.elapsed().as_micros();
            let start_network = Instant::now();
            let part_bytes = 2 + 2 + 1 + 1;
            let common_capacity = 4 * 12 + 8;
            let capacity = world.particle_count * part_bytes + common_capacity;
            let mut data = vec![0; capacity];
            let mut data_common = Vec::with_capacity(common_capacity);
            data_common.extend(Utc::now().timestamp_millis().to_be_bytes().to_vec());
            data_common.extend((step as f32).to_be_bytes().to_vec());
            data_common.extend((elapsed_total as f32).to_be_bytes().to_vec());
            data_common.extend((elapsed_compute as f32).to_be_bytes().to_vec());
            data_common.extend((elapsed_total as f32).to_be_bytes().to_vec());
            data_common.extend(
                (shared_network_data.lock().unwrap().peers.len() as u32)
                    .to_be_bytes()
                    .to_vec(),
            );
            data_common.extend(collisions_count.to_be_bytes().to_vec());
            data_common.extend((world.diameter).to_be_bytes().to_vec());
            data_common.extend((world.particle_count as u32).to_be_bytes().to_vec());
            data_common.extend(((256.0 * 256.0) as f32).to_be_bytes().to_vec());
            data_common.extend((elapsed_network as f32).to_be_bytes().to_vec());
            data_common.extend((world.ships_count as u32).to_be_bytes().to_vec());
            data_common.extend(
                (shared_network_data.lock().unwrap().free_ship_pids.len() as u32)
                    .to_be_bytes()
                    .to_vec(),
            );
            data[..common_capacity].copy_from_slice(&data_common);
            let _data_2: Vec<u8> = vec![0; part_bytes * world.particle_count];
            for (pid, particle) in particles.iter().enumerate() {
                let i = common_capacity + pid * part_bytes;
                let xs = ((particle.p.x * 256.0 * 256.0) as u16).to_be_bytes();
                let ys = ((particle.p.y * 256.0 * 256.0) as u16).to_be_bytes();
                let mut status: u8 = 0;
                if particle.collisions > 0 {
                    status += 1;
                }
                if particle.activation > 0.01 {
                    status += 2;
                }
                data[i..(2 + i)].copy_from_slice(&xs[..2]);
                data[(2 + i)..(2 + 2 + i)].copy_from_slice(&ys[..2]);
                data[(4 + i)..(4 + 1 + i)].copy_from_slice(&status.to_be_bytes()[..1]);
                data[(5 + i)..(5 + 1 + i)]
                    .copy_from_slice(&(particle.kind as u8).to_be_bytes()[..1]);
            }
            assert!(data.len() == capacity);
            let m = Message::Binary(data);

            {
                let mut network_data = shared_network_data.lock().unwrap();
                let mut ship_pids = HashMap::new();
                for (k, v) in &network_data.users {
                    ship_pids.insert(*k, v.ship_pid);
                }
                for peer in &mut network_data.peers.values_mut() {
                    match peer.user_id {
                        Some(user_id) => {
                            let ship_pid = ship_pids.get(&user_id).unwrap();
                            let mut data = data_common.clone();
                            let mut count: u32 = 0;
                            let p1 = &particles[*ship_pid];
                            let grid_xy = grid_xy(&p1.p, grid.side);
                            let gx = grid_xy.x as i32;
                            let gy = grid_xy.y as i32;
                            let uu = 32;
                            data.extend(p1.p.x.to_be_bytes());
                            data.extend(p1.p.y.to_be_bytes());
                            let mut status: u8 = 0;
                            if p1.collisions > 0 {
                                status += 1;
                            }
                            if p1.activation > 0.01 {
                                status += 2;
                            }
                            data.extend(status.to_be_bytes());
                            data.extend((p1.kind as u8).to_be_bytes());
                            count += 1;
                            for x in gx - uu..gx + uu + 1 {
                                let _x_ = (x as usize + grid.side) % grid.side;
                                for y in gy - uu..gy + uu + 1 {
                                    let _y_ = (y as usize + grid.side) % grid.side;
                                    let gid = grid_id(x as usize, y as usize, grid.side);
                                    for pid2 in &grid.pids[gid] {
                                        let p2 = &particles[*pid2];
                                        data.extend(p2.p.x.to_be_bytes());
                                        data.extend(p2.p.y.to_be_bytes());
                                        let mut status: u8 = 0;
                                        if p2.collisions > 0 {
                                            status += 1;
                                        }
                                        if p2.activation > 0.01 {
                                            status += 2;
                                        }
                                        data.extend(status.to_be_bytes());
                                        data.extend((p2.kind as u8).to_be_bytes());
                                        count += 1;
                                    }
                                }
                            }
                            data[8 + 7 * 4..8 + 8 * 4].copy_from_slice(&count.to_be_bytes());
                            data[8 + 8 * 4..8 + 9 * 4].copy_from_slice(&1.0_f32.to_be_bytes());
                            let m = Message::Binary(data);
                            if peer.tx.start_send(m).is_ok() {
                                // println!("send ok");
                            }
                        }
                        None => {
                            if peer.tx.start_send(m.clone()).is_ok() {
                                // println!("send ok");
                            }
                        }
                    }
                }
            }
            elapsed_network = start_network.elapsed().as_micros();
            *w += 1;
        }
        wait(&syncers[3], world.thread_count);
        //
        // More
        //
        elapsed_total += start.elapsed().as_micros();
        step += 1;
        let delta = Duration::from_millis(10);
        if start.elapsed() < delta {
            let sleep_duration = delta - start.elapsed();
            thread::sleep(sleep_duration);
        }
    });
}
