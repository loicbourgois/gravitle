const setup = (engine, world) => {
    const b2 = world.add_particle(0, 1, engine.Kind.Battery);
    const b1 = world.add_particle(0, 0, engine.Kind.Battery);
    const sp1 = world.add_particle(1, 0, engine.Kind.SolarPanel);
    world.add_link(b1, b2)
    world.add_link(b1, sp1)
}

export {
    setup,
}
