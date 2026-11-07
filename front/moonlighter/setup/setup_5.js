// b2 is at 10/10
// b1 stays at ~1/10
// Each link can only transmit at most 1 elec/tick.
// There's only 1 link between b1 and b2, going through p2
// p2 consumes 1 elec/tick.
// Over capacity can not flow from b2 to b1 through p2
// We would need a wire bypass
// See setup_6
const setup = (engine, world) => {
    world.add_particle(0, 0, engine.Kind.Moon)
    world.add_particle(1, 0, engine.Kind.Moon)
    world.add_particle(-1, 0, engine.Kind.Moon);
    world.add_particle(0, 1, engine.Kind.Moon);
    world.add_particle(0, -1, engine.Kind.Moon);
    world.add_particle(5, 0.1, engine.Kind.Pipe);
    world.add_particle(4, 0, engine.Kind.Pipe);
    world.add_particle(3, 0.1, engine.Kind.Pipe);
    world.add_particle(2, -0.1, engine.Kind.Pipe);
    world.add_particle(6, 0.1, engine.Kind.Filter);
    const p1 = world.add_particle(7, 0.2, engine.Kind.Pump);
    const b1 = world.add_particle(7.1, 1.2, engine.Kind.Battery);
    const b2 = world.add_particle(9.1, 1., engine.Kind.Battery);
    const sp1 = world.add_particle(7, 2.2, engine.Kind.SolarPanel);
    const p2 = world.add_particle(8.1, 1.1, engine.Kind.Pump);
    const sp2 = world.add_particle(9, 2.2, engine.Kind.SolarPanel);
    const sp3 = world.add_particle(9, -0.2, engine.Kind.SolarPanel);
    const sp4 = world.add_particle(10.2, 1.2, engine.Kind.SolarPanel);
    world.add_link(5, 6)
    world.add_link(6, 7)
    world.add_link(7, 8)
    world.add_link(8, 1)
    world.add_link(5, 9)
    world.add_link(10, 9)
    world.add_link(b1, p1)
    world.add_link(b1, sp1)
    world.add_link(b1, p2)
    world.add_link(b2, p2)
    world.add_link(b2, sp2)
    world.add_link(b2, sp3)
    world.add_link(b2, sp4)
}

export {
    setup,
}
