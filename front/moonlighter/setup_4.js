const setup = (engine, world) => {
    const sp1 = world.add_particle(-2, 0, engine.Kind.SolarPanel);
    const p1 = world.add_particle(-1, 0, engine.Kind.Pump);
    world.add_link(sp1, p1)
}

export {
    setup,
}
