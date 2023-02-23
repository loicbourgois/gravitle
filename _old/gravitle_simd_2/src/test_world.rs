use crate::world::*;
use crate::*;
use rand::Rng;

// #[bench]
// fn bench_update_150(bencher: &mut Bencher) {
//     let mut world = World::new(0.001, 50_000, 150);
//     bencher.iter(|| world.update());
// }
//
// #[bench]
// fn bench_update_180(bencher: &mut Bencher) {
//     let mut world = World::new(0.001, 50_000, 180);
//     bencher.iter(|| world.update());
// }

#[bench]
fn bench_update_200(bencher: &mut Bencher) {
    let mut world = World::new(0.001, 50_000, 200);
    bencher.iter(|| unsafe { world.update() });
}

#[bench]
fn bench_update_200_b(bencher: &mut Bencher) {
    let mut world = World::new(0.001, 50_000, 200);
    bencher.iter(|| world.update_b());
}

// #[bench]
// fn bench_update_210(bencher: &mut Bencher) {
//     let mut world = World::new(0.001, 50_000, 210);
//     bencher.iter(|| world.update());
// }
//
// #[bench]
// fn bench_update_250(bencher: &mut Bencher) {
//     let mut world = World::new(0.001, 50_000, 250);
//     bencher.iter(|| world.update());
// }

// #[bench]
// fn bench_update_d(bencher: &mut Bencher) {
//     let mut world = World::new(0.001, 50_000, 400);
//     bencher.iter(|| world.update());
// }

// #[bench]
// fn bench_update_2(bencher: &mut Bencher) {
//     let mut world = World::new(0.001, 1000, 300);
//     bencher.iter(|| world.update_2());
// }
