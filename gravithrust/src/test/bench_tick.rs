extern crate test;
use crate::gravithrust_tick::compute_collision_responses;
use crate::gravithrust_tick::compute_link_responses;
use crate::test::helpers::setup_simulation;
use crate::test::helpers::setup_simulation_grid_side;
use test::Bencher;
#[bench]
fn bench_tick(b: &mut Bencher) {
    let mut g = setup_simulation();
    b.iter(|| g.tick());
}
#[bench]
fn bench_tick_grid_side_1(b: &mut Bencher) {
    let mut g = setup_simulation_grid_side(100);
    b.iter(|| g.tick());
}
#[bench]
fn bench_tick_grid_side_2(b: &mut Bencher) {
    let mut g = setup_simulation_grid_side(128);
    b.iter(|| g.tick());
}
#[bench]
fn bench_tick_grid_side_3(b: &mut Bencher) {
    let mut g = setup_simulation_grid_side(150);
    b.iter(|| g.tick());
}
#[bench]
fn bench_tick_grid_side_4(b: &mut Bencher) {
    let mut g = setup_simulation_grid_side(64);
    b.iter(|| g.tick());
}
#[bench]
fn bench_compute_collision_responses(b: &mut Bencher) {
    let mut g = setup_simulation();
    b.iter(|| {
        compute_collision_responses(
            g.diameter,
            &mut g.particles,
            &mut g.particles_internal,
            &g.grid,
        )
    });
}
#[bench]
fn bench_compute_compute_link_responses(b: &mut Bencher) {
    let mut g = setup_simulation();
    b.iter(|| {
        compute_link_responses(
            g.diameter,
            &mut g.particles,
            &mut g.particles_internal,
            &mut g.links,
            &mut g.links_js,
        )
    });
}
#[bench]
fn bench_update_particles(b: &mut Bencher) {
    let mut g = setup_simulation();
    b.iter(|| g.update_particles());
}
#[bench]
fn bench_update_ships(b: &mut Bencher) {
    let mut g = setup_simulation();
    b.iter(|| g.update_ships());
}
#[bench]
fn bench_grid_update(b: &mut Bencher) {
    let mut g = setup_simulation();
    b.iter(|| {
        g.grid.update_01();
        g.grid.update_02(&mut g.particles);
    });
}
