use crate::particle::*;
use crate::*;
use rand::Rng;

#[bench]
fn bench_normalize(bencher: &mut Bencher) {
    let mut rng = rand::thread_rng();
    let mut a = Vec::new();
    for _ in 0..1 {
        a.push(Vector {
            x: rng.gen(),
            y: rng.gen(),
        })
    }
    bencher.iter(|| (normalize(&a[0])));
}

#[bench]
fn bench_normalize_inplace(bencher: &mut Bencher) {
    let mut rng = rand::thread_rng();
    let mut a = Vec::new();
    for _ in 0..1 {
        a.push(Vector {
            x: rng.gen(),
            y: rng.gen(),
        })
    }
    bencher.iter(|| normalize_inplace(&mut a[0]));
}

#[bench]
fn bench_norm(bencher: &mut Bencher) {
    let mut rng = rand::thread_rng();
    let mut a = Vec::new();
    for _ in 0..1 {
        a.push(Vector {
            x: rng.gen(),
            y: rng.gen(),
        })
    }
    bencher.iter(|| (norm(&a[0])))
}

#[bench]
fn bench_collision_response(bencher: &mut Bencher) {
    let mut rng = rand::thread_rng();
    let mut ps = Vec::new();
    for _ in 0..2 {
        ps.push(Particle {
            p: Vector {
                x: rng.gen(),
                y: rng.gen(),
            },
            v: Vector {
                x: rng.gen(),
                y: rng.gen(),
            },
            m: rng.gen(),
        });
    }
    bencher.iter(|| collision_response(&ps[0], &ps[1]));
}

#[bench]
fn bench_wrap_around(bencher: &mut Bencher) {
    let mut rng = rand::thread_rng();
    bencher.iter(|| {
        let mut ps = Vec::new();
        for _ in 0..2 {
            ps.push(Vector {
                x: rng.gen(),
                y: rng.gen(),
            })
        }
        wrap_around(&ps[0], &ps[1])
    });
}


#[bench]
fn bench_wrap_around_2(bencher: &mut Bencher) {
    let mut rng = rand::thread_rng();
    bencher.iter(|| {
        let mut ps = Vec::new();
        for _ in 0..2 {
            ps.push(Vector {
                x: rng.gen(),
                y: rng.gen(),
            })
        }
        wrap_around_2(&ps[0], &ps[1])
    });
}


#[test]
fn test_wrap_around() {
    let a = [Vector { x: 0.9431783, y: 0.7500257 }, Vector { x: 0.19544017, y: 0.49620646 }];
    let b = wrap_around(&a[0], &a[1]);
    assert!( b.d_sqrd.sqrt() < 0.37 );

    let a = [Vector { x: 0.7241143, y: 0.487625 }, Vector { x: 0.22385669, y: 0.49002028 }];
    let b = wrap_around(&a[0], &a[1]);
    assert!( b.d_sqrd.sqrt() < 0.51 );

    let a =  [Vector { x: 0.74952763, y: 0.35994428 }, Vector { x: 0.00013560057, y: 0.60556775 }];
    let b = wrap_around(&a[0], &a[1]);
    println!("{}", b.d_sqrd.sqrt());
    assert!( b.d_sqrd.sqrt() < 0.39 );
}
