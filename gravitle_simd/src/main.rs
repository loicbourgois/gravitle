#![feature(portable_simd)]
#![feature(test)]
extern crate test;
use std::simd::f32x16;
use test::Bencher;
mod particle;
mod test_main;
mod test_particle;
use rand::Rng;
use crate::particle::wrap_around;
use crate::particle::Vector;
use crate::particle::distance;
use crate::particle::WrapAroundResponse;


fn add(a: &mut f32x16, b: &f32x16) {
    *a = *a + b;
}

fn add2(a: &mut [f32], b: &[f32]) {
    for i in 0..16 {
        a[i] = a[i] + b[i];
    }
}

fn main() {
    let mut max_d: f32 = 0.0;
    let mut rng = rand::thread_rng();
    let mut ps_max = Vec::new();;
    let mut wp_max = WrapAroundResponse {
        a: Vector {
            x: 0.0,
            y: 0.0,
        },
        b: Vector {
            x: 0.0,
            y: 0.0,
        },
        d_sqrd: 0.0,
    };
    for _ in 0..10_000_000 {
        let mut ps = Vec::new();
        for _ in 0..2 {
            ps.push(Vector {
                x: rng.gen(),
                y: rng.gen(),
            })
        }
        let wp = wrap_around(&ps[0], &ps[1]);
        let d = wp.d_sqrd.sqrt();

        if (d > max_d) {
            ps_max = ps;
            wp_max = wp
        }

        max_d = max_d.max(d)
    }
    println!("{}", ((max_d*max_d)/2.0).sqrt() );
    println!("max_d: {}", max_d );
    println!("{:?}", ps_max );
    println!("{:?}", wp_max );

    let mut max_d: f32 = 0.0;
    let mut rng = rand::thread_rng();
    for _ in 0..10_000_000 {
        let mut ps = Vec::new();
        for _ in 0..2 {
            ps.push(Vector {
                x: rng.gen(),
                y: rng.gen(),
            })
        }
        let d = distance(&ps[0], &ps[1]);
        max_d = max_d.max(d)
    }
    println!("{}", ((max_d*max_d)/2.0).sqrt() );
}
