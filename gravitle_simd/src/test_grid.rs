use crate::particle::new_particles;
use crate::*;

// #[bench]
// fn bench_update_1000_010(bencher: &mut Bencher) {
//     let mut particles = new_particles(1000);
//     let mut grid = Grid::new(10);
//     bencher.iter(|| grid.update(&mut particles));
// }
//
// #[bench]
// fn bench_update_1000_020(bencher: &mut Bencher) {
//     let mut particles = new_particles(1000);
//     let mut grid = Grid::new(20);
//     bencher.iter(|| grid.update(&mut particles));
// }
//
// #[bench]
// fn bench_update_1000_100(bencher: &mut Bencher) {
//     let mut particles = new_particles(1000);
//     let mut grid = Grid::new(100);
//     bencher.iter(|| grid.update(&mut particles));
// }
