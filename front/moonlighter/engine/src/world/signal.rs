use crate::world::Kind;
use crate::world::World;

pub fn transmission(world: &mut World) {
    for i1 in 0..world.particles.len() {
        match world.p(i1).kind {
            Kind::SignalWire => {
                let mut total: f32 = 0.0;
                let mut count: f32 = 0.0;
                for i2 in world.ns(i1) {
                    let p2 = world.p(i2);
                    match p2.kind {
                        Kind::SignalWire | Kind::SignalInput => {
                            total += p2.signal_q as f32;
                            count += 1.0;
                        }
                        _ => {}
                    }
                }
                if count > 0.0 {
                    let average = total / count;
                    world.pm(i1).signal_qn = if average > 0.0 {
                        average.ceil() as i32
                    } else {
                        average.floor() as i32
                    };
                }
            }
            Kind::SignalInput => {
                let mut total: f32 = 0.0;
                let mut count: f32 = 0.0;
                for i2 in world.ns(i1) {
                    let p2 = world.p(i2);
                    match p2.kind {
                        Kind::SensorMoonFull => {
                            total += p2.signal_q as f32;
                            count += 1.0;
                        }
                        _ => {}
                    }
                }
                if count > 0.0 {
                    let average = total / count;
                    world.pm(i1).signal_qn = if average > 0.0 {
                        average.ceil() as i32
                    } else {
                        average.floor() as i32
                    };
                }
            }
            Kind::Trit => {
                let mut total: i32 = 0;
                for i2 in world.ns(i1) {
                    let p2 = world.p(i2);
                    match p2.kind {
                        Kind::SensorMoonFull | Kind::SensorMoonEmpty => {
                            total += p2.signal_q;
                        }
                        _ => {}
                    }
                }
                world.pm(i1).signal_qn = match total.signum() {
                    1 => 1,
                    -1 => -1,
                    _ => world.p(i1).signal_q,
                };
            }
            Kind::SignalInverter => {
                let mut total: i32 = 0;
                for i2 in world.ns(i1) {
                    let p2 = world.p(i2);
                    match p2.kind {
                        Kind::Trit => {
                            total += p2.signal_q;
                        }
                        _ => {}
                    }
                }
                world.pm(i1).signal_qn = match total.signum() {
                    1 => -1,
                    -1 => 1,
                    _ => world.p(i1).signal_q,
                };
            }
            Kind::Valve | Kind::Switch => {
                let mut total: i32 = 0;
                for i2 in world.ns(i1) {
                    let p2 = world.p(i2);
                    match p2.kind {
                        Kind::SignalInverter | Kind::Trit => {
                            total += p2.signal_q;
                        }
                        _ => {}
                    }
                }
                world.pm(i1).signal_qn = match total.signum() {
                    1 => 1,
                    -1 => -1,
                    _ => world.p(i1).signal_q,
                };
            }
            _ => {}
        }
    }
    for particle in &mut world.particles {
        particle.signal_q = particle.signal_qn;
    }
}
