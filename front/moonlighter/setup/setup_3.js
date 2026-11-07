const setup = (engine, world) => {
    const sp1 = world.add_particle(-2, 0, engine.Kind.SolarPanel);
    const b1 = world.add_particle(-1, 0, engine.Kind.Battery);
    const b2 = world.add_particle(0, 0, engine.Kind.Battery);
    const b3 = world.add_particle(1, 0, engine.Kind.Battery);
    const sp2 = world.add_particle(2, 0, engine.Kind.SolarPanel);
    world.add_link(sp1, b1)
    world.add_link(b1, b2)
    world.add_link(b2, b3)
    world.add_link(b3, sp2)
}

export {
    setup,
}
