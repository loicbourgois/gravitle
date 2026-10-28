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
    world.add_particle(7, 0.2, engine.Kind.Pump);
    // data.links.add(5, 6)
    // data.links.add(6, 7)
    // data.links.add(7, 8)
    // data.links.add(8, 1)
    // data.links.add(5, 9)
    // data.links.add(10, 9)
}

export {
    setup,
}
